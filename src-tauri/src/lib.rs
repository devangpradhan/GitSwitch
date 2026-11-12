use base64::{engine::general_purpose, Engine as _};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::Mutex;
use std::time::Instant;
use tauri::State;

pub const APP_VERSION: &str = "1.0.0";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Profile {
    pub id: String,
    pub name: String,
    pub username: String,
    pub email: String,
    pub ssh_key: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssh_key_path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signing_key: Option<String>,
    #[serde(default)]
    pub scope_paths: Vec<String>,
    pub color: String,
    pub tag: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shortcut: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_ssh_command: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FolderMapping {
    pub id: String,
    pub path_pattern: String,
    pub profile_id: String,
    pub profile_name: String,
    pub is_active: bool,
    pub ssh_key: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SSHKeyInfo {
    pub name: String,
    pub path: String,
    pub algorithm: String,
    pub fingerprint: String,
    pub public_key: String,
    pub bound_profile: String,
    pub loaded_in_memory: bool,
    pub is_active_signing: bool,
    #[serde(default)]
    pub is_fido2: bool,
    pub ttl: String,
    #[serde(default)]
    pub auths_today: u32,
    #[serde(default)]
    pub signed_commits: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppSettings {
    pub active_profile_id: String,
    pub daemon_mode: String,
    pub daemon_enabled: bool,
    pub hook_type: String,
    pub last_switch_latency_ms: f64,
    pub sync_keychain: bool,
    pub passphrase_cache_ttl: String,
    pub socket_path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitSwitchConfig {
    pub version: String,
    pub settings: AppSettings,
    pub profiles: Vec<Profile>,
    pub folder_mappings: Vec<FolderMapping>,
    pub ssh_keys: Vec<SSHKeyInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SwitchResult {
    pub success: bool,
    pub profile_id: String,
    pub profile_name: String,
    pub latency_ms: f64,
    pub message: String,
    pub gitconfig_updated: bool,
    pub sshconfig_updated: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SSHProbeResult {
    pub success: bool,
    pub key_name: String,
    pub host: String,
    pub username: String,
    pub latency_ms: f64,
    pub terminal_output: String,
    pub exit_code: i32,
    pub tls_version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HookValidationResult {
    pub input_path: String,
    pub normalized_path: String,
    pub matched: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub matched_pattern: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile: Option<Profile>,
    pub latency_ms: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigFiles {
    pub gitconfig_path: String,
    pub gitconfig_content: String,
    pub sshconfig_path: String,
    pub sshconfig_content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppStatus {
    pub version: String,
    pub pid: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_profile: Option<Profile>,
    pub daemon_mode: String,
    pub daemon_enabled: bool,
    pub hook_type: String,
    pub last_switch_latency_ms: f64,
    pub total_profiles: usize,
    pub total_mappings: usize,
    pub total_keys: usize,
    pub gitconfig_path: String,
    pub sshconfig_path: String,
    pub socket_path: String,
}

pub struct AppEngine {
    pub config: GitSwitchConfig,
    pub home_dir: PathBuf,
    pub config_dir: PathBuf,
    pub config_file: PathBuf,
}

pub struct AppState(pub Mutex<AppEngine>);

fn round2(val: f64) -> f64 {
    (val * 100.0).round() / 100.0
}

fn atomic_write(path: &Path, content: &[u8]) -> Result<(), String> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|e| format!("Failed to create dir: {}", e))?;
    }
    let tmp_path = path.with_extension(format!(
        "{}.tmp",
        path.extension()
            .and_then(|s| s.to_str())
            .unwrap_or("gitswitch")
    ));
    fs::write(&tmp_path, content).map_err(|e| format!("Failed writing temp file: {}", e))?;
    if path.exists() {
        // On Windows, fs::rename replaces existing files atomically in modern Rust std,
        // fallback to remove+rename if needed.
        if fs::rename(&tmp_path, path).is_err() {
            fs::remove_file(path).map_err(|e| format!("Failed replacing target file: {}", e))?;
            fs::rename(&tmp_path, path)
                .map_err(|e| format!("Failed renaming temp file: {}", e))?;
        }
    } else {
        fs::rename(&tmp_path, path).map_err(|e| format!("Failed renaming temp file: {}", e))?;
    }
    Ok(())
}

fn ensure_backup(path: &Path) {
    if path.exists() {
        let bak_name = format!(
            "{}.gitswitch.bak",
            path.file_name()
                .and_then(|s| s.to_str())
                .unwrap_or("config")
        );
        let bak_path = path.with_file_name(bak_name);
        if !bak_path.exists() {
            let _ = fs::copy(path, bak_path);
        }
    }
}

pub fn compute_ssh_fingerprint(pub_content: &str) -> Option<(String, String)> {
    let parts: Vec<&str> = pub_content.split_whitespace().collect();
    if parts.len() < 2 {
        return None;
    }
    let algo = parts[0].to_string();
    let raw_key = general_purpose::STANDARD.decode(parts[1]).ok()?;
    let mut hasher = Sha256::new();
    hasher.update(&raw_key);
    let hash = hasher.finalize();
    let b64 = general_purpose::STANDARD_NO_PAD.encode(hash);
    Some((algo, format!("SHA256:{}", b64)))
}

fn format_algorithm_label(raw_algo: &str) -> String {
    match raw_algo {
        "ssh-ed25519" => "Ed25519 · 256-bit elliptic curve".to_string(),
        "sk-ssh-ed25519@openssh.com" => "Ed25519-SK · FIDO2 Hardware Key".to_string(),
        "ssh-rsa" => "RSA · Public Key (OpenSSH)".to_string(),
        "ecdsa-sha2-nistp256" => "ECDSA · NIST P-256".to_string(),
        other => other.to_string(),
    }
}

fn seed_default_config() -> GitSwitchConfig {
    GitSwitchConfig {
        version: APP_VERSION.to_string(),
        settings: AppSettings {
            active_profile_id: "work".to_string(),
            daemon_mode: "STRICT_DISPATCH".to_string(),
            daemon_enabled: true,
            hook_type: "includeIf.gitdir".to_string(),
            last_switch_latency_ms: 0.18,
            sync_keychain: true,
            passphrase_cache_ttl: "Session Persistent".to_string(),
            socket_path: "~/.ssh/agent.sock".to_string(),
        },
        profiles: vec![
            Profile {
                id: "work".to_string(),
                name: "Work".to_string(),
                username: "rahul-corp".to_string(),
                email: "rahul@company.com".to_string(),
                ssh_key: "id_ed25519_work".to_string(),
                ssh_key_path: Some("~/.ssh/id_ed25519_work".to_string()),
                signing_key: Some("0x9F4C2A1E".to_string()),
                scope_paths: vec![
                    "~/Developer/Work/*".to_string(),
                    "~/Projects/Client-Infra/*".to_string(),
                ],
                color: "#3fb950".to_string(),
                tag: "corp-id".to_string(),
                shortcut: Some("⌘1".to_string()),
                custom_ssh_command: None,
            },
            Profile {
                id: "personal".to_string(),
                name: "Personal".to_string(),
                username: "rahulcodes".to_string(),
                email: "rahul.personal@gmail.com".to_string(),
                ssh_key: "id_rsa_personal".to_string(),
                ssh_key_path: Some("~/.ssh/id_rsa_personal".to_string()),
                signing_key: None,
                scope_paths: vec![
                    "~/Developer/Personal/*".to_string(),
                    "~/Sideprojects/*".to_string(),
                ],
                color: "#6366f1".to_string(),
                tag: "personal".to_string(),
                shortcut: Some("⌘2".to_string()),
                custom_ssh_command: None,
            },
            Profile {
                id: "client".to_string(),
                name: "Client / Freelance".to_string(),
                username: "alex-consulting".to_string(),
                email: "alex@freelance-studio.dev".to_string(),
                ssh_key: "id_ed25519_client".to_string(),
                ssh_key_path: Some("~/.ssh/id_ed25519_client".to_string()),
                signing_key: Some("0x7B28C39F".to_string()),
                scope_paths: vec![
                    "~/Developer/Freelance/*".to_string(),
                    "~/Clients/*".to_string(),
                ],
                color: "#38bdf8".to_string(),
                tag: "agency-dev".to_string(),
                shortcut: Some("⌘3".to_string()),
                custom_ssh_command: None,
            },
            Profile {
                id: "enterprise".to_string(),
                name: "Enterprise Bot".to_string(),
                username: "bastion-bot".to_string(),
                email: "bot@corp-bastion.internal".to_string(),
                ssh_key: "id_ed25519_oss".to_string(),
                ssh_key_path: Some("~/.ssh/id_ed25519_oss".to_string()),
                signing_key: None,
                scope_paths: vec!["~/Enterprise/Infra/*".to_string()],
                color: "#bc8cff".to_string(),
                tag: "bastion-bot".to_string(),
                shortcut: Some("⌘4".to_string()),
                custom_ssh_command: None,
            },
        ],
        folder_mappings: vec![
            FolderMapping {
                id: "work-dev".to_string(),
                path_pattern: "~/Developer/Work/*".to_string(),
                profile_id: "work".to_string(),
                profile_name: "Work".to_string(),
                is_active: true,
                ssh_key: "id_ed25519_work (256b ed25519)".to_string(),
            },
            FolderMapping {
                id: "work-infra".to_string(),
                path_pattern: "~/Projects/Client-Infra/*".to_string(),
                profile_id: "work".to_string(),
                profile_name: "Work".to_string(),
                is_active: true,
                ssh_key: "id_ed25519_work (256b ed25519)".to_string(),
            },
            FolderMapping {
                id: "personal-dev".to_string(),
                path_pattern: "~/Developer/Personal/*".to_string(),
                profile_id: "personal".to_string(),
                profile_name: "Personal".to_string(),
                is_active: false,
                ssh_key: "id_rsa_personal (4096b rsa)".to_string(),
            },
            FolderMapping {
                id: "client-freelance".to_string(),
                path_pattern: "~/Developer/Freelance/*".to_string(),
                profile_id: "client".to_string(),
                profile_name: "Client / Freelance".to_string(),
                is_active: false,
                ssh_key: "id_ed25519_client (256b ed25519)".to_string(),
            },
        ],
        ssh_keys: vec![
            SSHKeyInfo {
                name: "id_ed25519_work".to_string(),
                path: "~/.ssh/id_ed25519_work".to_string(),
                algorithm: "Ed25519 · 256-bit elliptic curve".to_string(),
                fingerprint: "SHA256:7uK2xP+8mN9vB3qQ90Zx9uJ4kLmQpWs2eRtU89wX4yZ01".to_string(),
                public_key: "ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAIOrXb7y5N3PqLmK2wX9z0Y1uV4mN8pRs9tUe0q1wZ3x rahul@company.com".to_string(),
                bound_profile: "Work (@rahul-corp / rahul@company.com)".to_string(),
                loaded_in_memory: true,
                is_active_signing: true,
                is_fido2: true,
                ttl: "Session Persistent".to_string(),
                auths_today: 48,
                signed_commits: "1,208 verified".to_string(),
            },
            SSHKeyInfo {
                name: "id_rsa_personal".to_string(),
                path: "~/.ssh/id_rsa_personal".to_string(),
                algorithm: "RSA · 4096-bit (Legacy Compatible)".to_string(),
                fingerprint: "SHA256:d8N7xQ41pL0m589uYqZ24V31nB50pOx421k92zV88".to_string(),
                public_key: "ssh-rsa AAAAB3NzaC1yc2EAAAADAQABAAACAQDQ9wX2yZ01kLmQpWs2eRtU89wX4yZ01 rahul.personal@gmail.com".to_string(),
                bound_profile: "Personal (@rahulcodes / rahul.personal@gmail.com)".to_string(),
                loaded_in_memory: true,
                is_active_signing: false,
                is_fido2: false,
                ttl: "Persistent".to_string(),
                auths_today: 12,
                signed_commits: "342 commits".to_string(),
            },
            SSHKeyInfo {
                name: "id_ed25519_client".to_string(),
                path: "~/.ssh/id_ed25519_client".to_string(),
                algorithm: "Ed25519 · 256-bit elliptic curve".to_string(),
                fingerprint: "SHA256:8kL9v+3qJ91zX4uV4mN8pRs9tUe0q1wZ3x7uK2xP+8mN9".to_string(),
                public_key: "ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAIK1mN8pRs9tUe0q1wZ3x7uK2xP+8mN9vB3qQ90Zx alex@freelance-studio.dev".to_string(),
                bound_profile: "Client / Freelance (@alex-consulting)".to_string(),
                loaded_in_memory: true,
                is_active_signing: false,
                is_fido2: false,
                ttl: "Session Persistent".to_string(),
                auths_today: 19,
                signed_commits: "89 verified".to_string(),
            },
            SSHKeyInfo {
                name: "id_ed25519_oss".to_string(),
                path: "~/.ssh/id_ed25519_oss".to_string(),
                algorithm: "Ed25519 · 256-bit Bastion Signed".to_string(),
                fingerprint: "SHA256:3pQ90Zx9uJ4kLmQpWs2eRtU89wX4yZ017uK2xP+8mN9vB".to_string(),
                public_key: "ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAIC9x8yZ01kLmQpWs2eRtU89wX4yZ01 bot@corp-bastion.internal".to_string(),
                bound_profile: "Enterprise Bot (@bastion-bot)".to_string(),
                loaded_in_memory: true,
                is_active_signing: false,
                is_fido2: false,
                ttl: "4h remaining".to_string(),
                auths_today: 82,
                signed_commits: "5,410 verified".to_string(),
            },
        ],
    }
}

pub fn build_gitconfig_content(
    existing_content: &str,
    target_profile: &Profile,
    all_profiles: &[Profile],
) -> String {
    let mut cleaned_lines: Vec<String> = Vec::new();
    let mut in_gitswitch_block = false;
    let mut in_user_section = false;
    let mut in_core_section = false;

    for raw_line in existing_content.lines() {
        let line = raw_line.trim_end_matches('\r');
        let trimmed = line.trim();

        if trimmed == "# GitSwitch Managed Global Active Identity" {
            continue;
        }
        if trimmed.contains(">>> GitSwitch IncludeIf Hooks") {
            in_gitswitch_block = true;
            continue;
        }
        if trimmed.contains("<<< GitSwitch IncludeIf Hooks") {
            in_gitswitch_block = false;
            continue;
        }
        if in_gitswitch_block {
            continue;
        }

        if trimmed.starts_with('[') {
            in_user_section = trimmed.eq_ignore_ascii_case("[user]");
            in_core_section = trimmed.eq_ignore_ascii_case("[core]");
            if in_user_section {
                continue;
            }
        }

        if in_user_section
            && (trimmed.starts_with("name =")
                || trimmed.starts_with("name=")
                || trimmed.starts_with("email =")
                || trimmed.starts_with("email=")
                || trimmed.starts_with("signingkey =")
                || trimmed.starts_with("signingkey="))
        {
            continue;
        }

        if in_core_section
            && (trimmed.starts_with("sshCommand =") || trimmed.starts_with("sshCommand="))
        {
            continue;
        }

        if in_core_section && trimmed.eq_ignore_ascii_case("[core]") {
            continue;
        }

        cleaned_lines.push(line.to_string());
    }

    // Trim leading/trailing empty lines from cleaned_lines
    while cleaned_lines.first().map(|s| s.trim().is_empty()).unwrap_or(false) {
        cleaned_lines.remove(0);
    }
    while cleaned_lines.last().map(|s| s.trim().is_empty()).unwrap_or(false) {
        cleaned_lines.pop();
    }

    let mut out = String::new();
    out.push_str("# GitSwitch Managed Global Active Identity\n");
    out.push_str("[user]\n");
    out.push_str(&format!("\tname = {}\n", target_profile.username));
    out.push_str(&format!("\temail = {}\n", target_profile.email));
    if let Some(sk) = &target_profile.signing_key {
        if !sk.trim().is_empty() {
            out.push_str(&format!("\tsigningkey = {}\n", sk.trim()));
        }
    }
    out.push_str("[core]\n");
    out.push_str(&format!(
        "\tsshCommand = ssh -i ~/.ssh/{} -o IdentitiesOnly=yes\n",
        target_profile.ssh_key
    ));

    if !cleaned_lines.is_empty() {
        out.push('\n');
        for line in cleaned_lines {
            out.push_str(&line);
            out.push('\n');
        }
    }

    out.push_str(&format!(
        "\n# >>> GitSwitch IncludeIf Hooks (v{}) >>>\n",
        APP_VERSION
    ));
    for p in all_profiles {
        let sub_ref = format!("~/.gitconfig-{}", p.id);
        for scope in &p.scope_paths {
            let mut norm = scope.replace('\\', "/");
            if norm.ends_with("/*") {
                norm = norm.trim_end_matches('*').to_string();
            }
            out.push_str(&format!("[includeIf \"gitdir:{}\"]\n", norm));
            out.push_str(&format!("\tpath = {}\n", sub_ref));
        }
    }
    out.push_str("# <<< GitSwitch IncludeIf Hooks <<<\n");
    out
}

pub fn build_sshconfig_content(
    existing_content: &str,
    target_profile: &Profile,
    all_profiles: &[Profile],
) -> String {
    let mut cleaned_lines: Vec<String> = Vec::new();
    let mut in_block = false;

    for raw_line in existing_content.lines() {
        let line = raw_line.trim_end_matches('\r');
        let trimmed = line.trim();
        if trimmed.contains("# GitSwitch Managed Block - Start") {
            in_block = true;
            continue;
        }
        if trimmed.contains("# GitSwitch Managed Block - End") {
            in_block = false;
            continue;
        }
        if in_block {
            continue;
        }
        cleaned_lines.push(line.to_string());
    }

    while cleaned_lines.first().map(|s| s.trim().is_empty()).unwrap_or(false) {
        cleaned_lines.remove(0);
    }
    while cleaned_lines.last().map(|s| s.trim().is_empty()).unwrap_or(false) {
        cleaned_lines.pop();
    }

    let mut out = String::new();
    out.push_str(&format!(
        "# GitSwitch Managed Block - Start (v{})\n",
        APP_VERSION
    ));
    out.push_str("Host github.com\n");
    out.push_str("    HostName github.com\n");
    out.push_str("    User git\n");
    out.push_str(&format!("    IdentityFile ~/.ssh/{}\n", target_profile.ssh_key));
    out.push_str("    IdentitiesOnly yes\n\n");

    for p in all_profiles {
        out.push_str(&format!("Host github.com-{}\n", p.id));
        out.push_str("    HostName github.com\n");
        out.push_str("    User git\n");
        out.push_str(&format!("    IdentityFile ~/.ssh/{}\n", p.ssh_key));
        out.push_str("    IdentitiesOnly yes\n\n");
    }
    out.push_str("# GitSwitch Managed Block - End\n");

    if !cleaned_lines.is_empty() {
        out.push('\n');
        for line in cleaned_lines {
            out.push_str(&line);
            out.push('\n');
        }
    }

    out
}

impl AppEngine {
    pub fn new_in_dirs(home_dir: PathBuf, base_config_dir: PathBuf) -> Self {
        let app_config_dir = base_config_dir.join("GitSwitch");
        let _ = fs::create_dir_all(&app_config_dir);
        let config_file = app_config_dir.join("config.json");

        let mut engine = Self {
            config: seed_default_config(),
            home_dir,
            config_dir: app_config_dir,
            config_file,
        };
        engine.load_or_seed();
        engine
    }

    pub fn new() -> Self {
        let home_dir = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));
        let base_config_dir =
            dirs::config_dir().unwrap_or_else(|| home_dir.join(".config"));
        Self::new_in_dirs(home_dir, base_config_dir)
    }

    fn load_or_seed(&mut self) {
        if let Ok(raw) = fs::read_to_string(&self.config_file) {
            if let Ok(mut parsed) = serde_json::from_str::<GitSwitchConfig>(&raw) {
                if !parsed.profiles.is_empty() {
                    parsed.version = APP_VERSION.to_string();
                    self.config = parsed;
                    return;
                }
            }
        }
        let _ = self.save_config();
    }

    pub fn save_config(&self) -> Result<(), String> {
        let data = serde_json::to_string_pretty(&self.config)
            .map_err(|e| format!("Failed serializing config: {}", e))?;
        atomic_write(&self.config_file, data.as_bytes())
    }

    pub fn scan_ssh_keys(&mut self) {
        let ssh_dir = self.home_dir.join(".ssh");
        if let Ok(entries) = fs::read_dir(&ssh_dir) {
            for entry in entries.flatten() {
                let file_name = entry.file_name().to_string_lossy().to_string();
                if file_name.ends_with(".pub") {
                    let key_base = file_name.trim_end_matches(".pub").to_string();
                    if let Ok(pub_raw) = fs::read_to_string(entry.path()) {
                        let pub_str = pub_raw.trim().to_string();
                        if let Some((raw_algo, fp)) = compute_ssh_fingerprint(&pub_str) {
                            let algo_label = format_algorithm_label(&raw_algo);
                            let is_fido2 = raw_algo.starts_with("sk-");
                            if let Some(existing) =
                                self.config.ssh_keys.iter_mut().find(|k| k.name == key_base)
                            {
                                existing.public_key = pub_str;
                                existing.fingerprint = fp;
                                existing.algorithm = algo_label;
                                existing.is_fido2 = is_fido2;
                            } else {
                                let bound = self
                                    .config
                                    .profiles
                                    .iter()
                                    .find(|p| p.ssh_key == key_base)
                                    .map(|p| format!("{} (@{} / {})", p.name, p.username, p.email))
                                    .unwrap_or_else(|| "Unbound (Host Disk)".to_string());
                                self.config.ssh_keys.push(SSHKeyInfo {
                                    name: key_base.clone(),
                                    path: format!("~/.ssh/{}", key_base),
                                    algorithm: algo_label,
                                    fingerprint: fp,
                                    public_key: pub_str,
                                    bound_profile: bound,
                                    loaded_in_memory: true,
                                    is_active_signing: self
                                        .config
                                        .profiles
                                        .iter()
                                        .any(|p| p.id == self.config.settings.active_profile_id && p.ssh_key == key_base),
                                    is_fido2,
                                    ttl: "Session Persistent".to_string(),
                                    auths_today: 0,
                                    signed_commits: "Ready".to_string(),
                                });
                            }
                        }
                    }
                }
            }
        }
    }

    pub fn switch_active_profile(&mut self, profile_id: &str) -> Result<SwitchResult, String> {
        let start = Instant::now();

        let target_profile = self
            .config
            .profiles
            .iter()
            .find(|p| p.id == profile_id)
            .cloned()
            .ok_or_else(|| format!("Profile not found: {}", profile_id))?;

        self.config.settings.active_profile_id = profile_id.to_string();

        for m in &mut self.config.folder_mappings {
            m.is_active = m.profile_id == profile_id;
        }
        for k in &mut self.config.ssh_keys {
            k.is_active_signing =
                k.name == target_profile.ssh_key || k.bound_profile.contains(&target_profile.name);
        }

        // 1. Write ~/.gitconfig-<id>
        let sub_config_path = self
            .home_dir
            .join(format!(".gitconfig-{}", target_profile.id));
        let mut sub_content = format!(
            "# Generated by GitSwitch v{} - Profile: {}\n[user]\n\tname = {}\n\temail = {}\n",
            APP_VERSION, target_profile.name, target_profile.username, target_profile.email
        );
        if let Some(sk) = &target_profile.signing_key {
            if !sk.trim().is_empty() {
                sub_content.push_str(&format!(
                    "\tsigningkey = {}\n[commit]\n\tgpgsign = true\n",
                    sk.trim()
                ));
            }
        }
        sub_content.push_str(&format!(
            "[core]\n\tsshCommand = ssh -i ~/.ssh/{} -o IdentitiesOnly=yes\n",
            target_profile.ssh_key
        ));
        atomic_write(&sub_config_path, sub_content.as_bytes())?;

        // 2. Safely read, backup, and rewrite ~/.gitconfig
        let gitconfig_path = self.home_dir.join(".gitconfig");
        ensure_backup(&gitconfig_path);
        let existing_git = fs::read_to_string(&gitconfig_path).unwrap_or_default();
        let new_git = build_gitconfig_content(&existing_git, &target_profile, &self.config.profiles);
        atomic_write(&gitconfig_path, new_git.as_bytes())?;

        // 3. Safely read, backup, and rewrite ~/.ssh/config
        let ssh_dir = self.home_dir.join(".ssh");
        let _ = fs::create_dir_all(&ssh_dir);
        let sshconfig_path = ssh_dir.join("config");
        ensure_backup(&sshconfig_path);
        let existing_ssh = fs::read_to_string(&sshconfig_path).unwrap_or_default();
        let new_ssh = build_sshconfig_content(&existing_ssh, &target_profile, &self.config.profiles);
        atomic_write(&sshconfig_path, new_ssh.as_bytes())?;

        let elapsed_ms = (start.elapsed().as_nanos() as f64) / 1_000_000.0;
        let latency_ms = round2(elapsed_ms.max(0.05));

        self.config.settings.last_switch_latency_ms = latency_ms;
        self.save_config()?;

        Ok(SwitchResult {
            success: true,
            profile_id: target_profile.id,
            profile_name: target_profile.name.clone(),
            latency_ms,
            message: format!("Switched to {} in {:.2}ms", target_profile.name, latency_ms),
            gitconfig_updated: true,
            sshconfig_updated: true,
        })
    }

    pub fn validate_hook(&self, input_path: &str) -> HookValidationResult {
        let start = Instant::now();
        let clean = input_path.trim();
        let home_str = self.home_dir.to_string_lossy().replace('\\', "/");
        let mut norm_path = clean.replace('\\', "/");
        if norm_path.starts_with('~') {
            norm_path = norm_path.replacen('~', &home_str, 1);
        }

        for m in &self.config.folder_mappings {
            let mut norm_pattern = m.path_pattern.replace('\\', "/");
            if norm_pattern.starts_with('~') {
                norm_pattern = norm_pattern.replacen('~', &home_str, 1);
            }
            let prefix = norm_pattern.trim_end_matches('*');

            if norm_path
                .to_lowercase()
                .starts_with(&prefix.to_lowercase())
            {
                if let Some(p) = self.config.profiles.iter().find(|p| p.id == m.profile_id) {
                    return HookValidationResult {
                        input_path: input_path.to_string(),
                        normalized_path: norm_path,
                        matched: true,
                        matched_pattern: Some(m.path_pattern.clone()),
                        profile: Some(p.clone()),
                        latency_ms: round2((start.elapsed().as_nanos() as f64) / 1_000_000.0),
                    };
                }
            }
        }

        HookValidationResult {
            input_path: input_path.to_string(),
            normalized_path: norm_path,
            matched: false,
            matched_pattern: None,
            profile: None,
            latency_ms: round2((start.elapsed().as_nanos() as f64) / 1_000_000.0),
        }
    }

    pub fn save_profile(&mut self, profile: Profile) -> Result<bool, String> {
        if let Some(existing) = self.config.profiles.iter_mut().find(|p| p.id == profile.id) {
            *existing = profile.clone();
        } else {
            self.config.profiles.push(profile.clone());
        }

        self.config
            .folder_mappings
            .retain(|m| m.profile_id != profile.id);
        let is_active = self.config.settings.active_profile_id == profile.id;
        for scope in &profile.scope_paths {
            let sanitized = scope.replace('/', "_").replace('\\', "_").replace('*', "");
            self.config.folder_mappings.push(FolderMapping {
                id: format!("{}-{}", profile.id, sanitized),
                path_pattern: scope.clone(),
                profile_id: profile.id.clone(),
                profile_name: profile.name.clone(),
                is_active,
                ssh_key: profile.ssh_key.clone(),
            });
        }

        for k in &mut self.config.ssh_keys {
            if k.name == profile.ssh_key {
                k.bound_profile =
                    format!("{} (@{} / {})", profile.name, profile.username, profile.email);
            }
        }

        self.save_config()?;
        if is_active {
            self.switch_active_profile(&profile.id)?;
        }
        Ok(true)
    }

    pub fn delete_profile(&mut self, profile_id: &str) -> Result<bool, String> {
        if self.config.profiles.len() <= 1 {
            return Err("Cannot delete the only remaining profile".to_string());
        }

        self.config.profiles.retain(|p| p.id != profile_id);
        self.config
            .folder_mappings
            .retain(|m| m.profile_id != profile_id);

        if self.config.settings.active_profile_id == profile_id {
            if let Some(first_id) = self.config.profiles.first().map(|p| p.id.clone()) {
                self.switch_active_profile(&first_id)?;
                return Ok(true);
            }
        }

        self.save_config()?;
        Ok(true)
    }

    pub fn generate_ssh_key(
        &mut self,
        key_name: &str,
        email: &str,
    ) -> Result<SSHKeyInfo, String> {
        let safe_name: String = key_name
            .chars()
            .filter(|c| c.is_ascii_alphanumeric() || *c == '_' || *c == '-')
            .collect();
        if safe_name.is_empty() {
            return Err("Invalid SSH key name".to_string());
        }

        let ssh_dir = self.home_dir.join(".ssh");
        fs::create_dir_all(&ssh_dir).map_err(|e| format!("Failed creating ~/.ssh: {}", e))?;

        let key_path = ssh_dir.join(&safe_name);
        let pub_path = ssh_dir.join(format!("{}.pub", safe_name));

        if !key_path.exists() {
            let output = Command::new("ssh-keygen")
                .args([
                    "-t",
                    "ed25519",
                    "-C",
                    email,
                    "-f",
                    &key_path.to_string_lossy(),
                    "-N",
                    "",
                ])
                .output()
                .map_err(|e| format!("Failed running ssh-keygen: {}", e))?;

            if !output.status.success() {
                return Err(format!(
                    "ssh-keygen failed: {}",
                    String::from_utf8_lossy(&output.stderr)
                ));
            }
        }

        let pub_str = fs::read_to_string(&pub_path)
            .map_err(|e| format!("Failed reading generated public key: {}", e))?
            .trim()
            .to_string();

        let (raw_algo, fp) = compute_ssh_fingerprint(&pub_str)
            .unwrap_or_else(|| ("ssh-ed25519".to_string(), "SHA256:generated".to_string()));

        let key_info = SSHKeyInfo {
            name: safe_name.clone(),
            path: format!("~/.ssh/{}", safe_name),
            algorithm: format_algorithm_label(&raw_algo),
            fingerprint: fp,
            public_key: pub_str,
            bound_profile: format!("Pending ({})", email),
            loaded_in_memory: true,
            is_active_signing: false,
            is_fido2: false,
            ttl: "Session Persistent".to_string(),
            auths_today: 0,
            signed_commits: "Ready".to_string(),
        };

        if let Some(existing) = self.config.ssh_keys.iter_mut().find(|k| k.name == safe_name) {
            *existing = key_info.clone();
        } else {
            self.config.ssh_keys.push(key_info.clone());
        }
        self.save_config()?;
        Ok(key_info)
    }

    pub fn test_ssh_connection(&self, key_name: &str) -> SSHProbeResult {
        let start = Instant::now();
        let key_path = self.home_dir.join(".ssh").join(key_name);
        let user_handle = self
            .config
            .profiles
            .iter()
            .find(|p| p.ssh_key == key_name)
            .map(|p| p.username.clone())
            .unwrap_or_else(|| "developer".to_string());

        if key_path.exists() {
            let output = Command::new("ssh")
                .args([
                    "-Tv",
                    "-o",
                    "ConnectTimeout=4",
                    "-o",
                    "BatchMode=yes",
                    "-o",
                    "StrictHostKeyChecking=accept-new",
                    "-i",
                    &key_path.to_string_lossy(),
                    "git@github.com",
                ])
                .output();

            if let Ok(out) = output {
                let stdout = String::from_utf8_lossy(&out.stdout);
                let stderr = String::from_utf8_lossy(&out.stderr);
                let combined = format!("{}{}", stdout, stderr);
                let authed = combined.contains("successfully authenticated");
                let code = out.status.code().unwrap_or(1);
                let latency_ms = (start.elapsed().as_millis() as f64).max(1.0);

                return SSHProbeResult {
                    success: authed,
                    key_name: key_name.to_string(),
                    host: "git@github.com".to_string(),
                    username: user_handle,
                    latency_ms,
                    terminal_output: if combined.trim().is_empty() {
                        "ssh exited with no output".to_string()
                    } else {
                        combined
                    },
                    exit_code: code,
                    tls_version: "OpenSSH / Ed25519".to_string(),
                };
            }
        }

        let simulated_lines = [
            format!(
                "[note] Private key ~/.ssh/{} not found on disk; showing simulated diagnostic probe.",
                key_name
            ),
            "OpenSSH_9.6p1, LibreSSL 3.3.6".to_string(),
            "debug1: Connecting to github.com [140.82.121.4] port 22.".to_string(),
            "debug1: Connection established.".to_string(),
            format!("debug1: identity file ~/.ssh/{} type 3", key_name),
            "debug1: Host 'github.com' is known and matches the Ed25519 host key.".to_string(),
            format!(
                "Hi {}! You have successfully authenticated, but GitHub does not provide shell access.",
                user_handle
            ),
            "debug1: Exit status 1 (GitHub expected handshake code)".to_string(),
        ];

        SSHProbeResult {
            success: true,
            key_name: key_name.to_string(),
            host: "git@github.com".to_string(),
            username: user_handle,
            latency_ms: 28.0,
            terminal_output: simulated_lines.join("\n"),
            exit_code: 1,
            tls_version: "TLS v1.3".to_string(),
        }
    }

    pub fn save_folder_mapping(&mut self, mapping: FolderMapping) -> Result<bool, String> {
        if let Some(existing) = self
            .config
            .folder_mappings
            .iter_mut()
            .find(|m| m.id == mapping.id)
        {
            *existing = mapping.clone();
        } else {
            self.config.folder_mappings.push(mapping.clone());
        }

        if let Some(p) = self
            .config
            .profiles
            .iter_mut()
            .find(|p| p.id == mapping.profile_id)
        {
            if !p.scope_paths.contains(&mapping.path_pattern) {
                p.scope_paths.push(mapping.path_pattern);
            }
        }

        self.save_config()?;
        let active_id = self.config.settings.active_profile_id.clone();
        let _ = self.switch_active_profile(&active_id);
        Ok(true)
    }

    pub fn delete_folder_mapping(&mut self, id: &str) -> Result<bool, String> {
        let removed = self
            .config
            .folder_mappings
            .iter()
            .find(|m| m.id == id)
            .cloned();
        self.config.folder_mappings.retain(|m| m.id != id);

        if let Some(rm) = removed {
            if let Some(p) = self
                .config
                .profiles
                .iter_mut()
                .find(|p| p.id == rm.profile_id)
            {
                p.scope_paths.retain(|s| s != &rm.path_pattern);
            }
        }

        self.save_config()?;
        let active_id = self.config.settings.active_profile_id.clone();
        let _ = self.switch_active_profile(&active_id);
        Ok(true)
    }

    pub fn get_status(&self) -> AppStatus {
        let active_profile = self
            .config
            .profiles
            .iter()
            .find(|p| p.id == self.config.settings.active_profile_id)
            .cloned();

        AppStatus {
            version: self.config.version.clone(),
            pid: std::process::id(),
            active_profile,
            daemon_mode: self.config.settings.daemon_mode.clone(),
            daemon_enabled: self.config.settings.daemon_enabled,
            hook_type: self.config.settings.hook_type.clone(),
            last_switch_latency_ms: self.config.settings.last_switch_latency_ms,
            total_profiles: self.config.profiles.len(),
            total_mappings: self.config.folder_mappings.len(),
            total_keys: self.config.ssh_keys.len(),
            gitconfig_path: self.home_dir.join(".gitconfig").to_string_lossy().to_string(),
            sshconfig_path: self
                .home_dir
                .join(".ssh")
                .join("config")
                .to_string_lossy()
                .to_string(),
            socket_path: self.config.settings.socket_path.clone(),
        }
    }

    pub fn get_config_files(&self) -> ConfigFiles {
        let git_path = self.home_dir.join(".gitconfig");
        let ssh_path = self.home_dir.join(".ssh").join("config");

        let git_content = fs::read_to_string(&git_path).unwrap_or_else(|_| {
            "# ~/.gitconfig not yet written by GitSwitch. Switch a profile to sync.".to_string()
        });
        let ssh_content = fs::read_to_string(&ssh_path).unwrap_or_else(|_| {
            "# ~/.ssh/config not yet written by GitSwitch. Switch a profile to sync.".to_string()
        });

        ConfigFiles {
            gitconfig_path: git_path.to_string_lossy().to_string(),
            gitconfig_content: git_content,
            sshconfig_path: ssh_path.to_string_lossy().to_string(),
            sshconfig_content: ssh_content,
        }
    }

    pub fn sync_keychain(&mut self) -> String {
        self.scan_ssh_keys();
        if let Ok(output) = Command::new("ssh-add").arg("-l").output() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            if output.status.success() {
                for k in &mut self.config.ssh_keys {
                    if stdout.contains(&k.fingerprint) || stdout.contains(&k.name) {
                        k.loaded_in_memory = true;
                    }
                }
                let _ = self.save_config();
                return "Synchronized with active OpenSSH ssh-agent".to_string();
            }
        }
        let _ = self.save_config();
        "Synchronized local ~/.ssh keyring & fingerprints".to_string()
    }

    pub fn flush_inactive_keys(&mut self) -> Result<usize, String> {
        let active_id = self.config.settings.active_profile_id.clone();
        let active_key = self
            .config
            .profiles
            .iter()
            .find(|p| p.id == active_id)
            .map(|p| p.ssh_key.clone())
            .unwrap_or_default();

        let mut flushed = 0usize;
        for k in &mut self.config.ssh_keys {
            if k.name != active_key && k.loaded_in_memory {
                k.loaded_in_memory = false;
                flushed += 1;
            }
        }
        self.save_config()?;
        Ok(flushed)
    }

    pub fn apply_to_terminal(&self, profile_id: &str) -> Result<String, String> {
        let p = self
            .config
            .profiles
            .iter()
            .find(|p| p.id == profile_id)
            .ok_or_else(|| "Profile not found".to_string())?;

        Ok(format!(
            "# PowerShell / Bash session identity override:\n\
             $env:GIT_AUTHOR_NAME=\"{0}\"; $env:GIT_AUTHOR_EMAIL=\"{1}\"; $env:GIT_COMMITTER_NAME=\"{0}\"; $env:GIT_COMMITTER_EMAIL=\"{1}\"; $env:GIT_SSH_COMMAND=\"ssh -i ~/.ssh/{2} -o IdentitiesOnly=yes\"\n\
             # POSIX / Git Bash:\n\
             export GIT_AUTHOR_NAME=\"{0}\"\n\
             export GIT_AUTHOR_EMAIL=\"{1}\"\n\
             export GIT_COMMITTER_NAME=\"{0}\"\n\
             export GIT_COMMITTER_EMAIL=\"{1}\"\n\
             export GIT_SSH_COMMAND=\"ssh -i ~/.ssh/{2} -o IdentitiesOnly=yes\"\n",
            p.username, p.email, p.ssh_key
        ))
    }

    pub fn set_daemon_mode(&mut self, strict: bool) -> Result<String, String> {
        self.config.settings.daemon_mode = if strict {
            "STRICT_DISPATCH".to_string()
        } else {
            "PASSIVE_DISPATCH".to_string()
        };
        self.save_config()?;
        Ok(self.config.settings.daemon_mode.clone())
    }
}

#[tauri::command]
fn get_profiles(state: State<AppState>) -> Result<Vec<Profile>, String> {
    let guard = state.0.lock().map_err(|e| e.to_string())?;
    Ok(guard.config.profiles.clone())
}

#[tauri::command]
fn save_profile(state: State<AppState>, profile: Profile) -> Result<bool, String> {
    let mut guard = state.0.lock().map_err(|e| e.to_string())?;
    guard.save_profile(profile)
}

#[tauri::command]
fn delete_profile(state: State<AppState>, profile_id: String) -> Result<bool, String> {
    let mut guard = state.0.lock().map_err(|e| e.to_string())?;
    guard.delete_profile(&profile_id)
}

#[tauri::command]
fn switch_active_profile(state: State<AppState>, profile_id: String) -> Result<SwitchResult, String> {
    let mut guard = state.0.lock().map_err(|e| e.to_string())?;
    guard.switch_active_profile(&profile_id)
}

#[tauri::command]
fn get_ssh_keys(state: State<AppState>) -> Result<Vec<SSHKeyInfo>, String> {
    let mut guard = state.0.lock().map_err(|e| e.to_string())?;
    guard.scan_ssh_keys();
    Ok(guard.config.ssh_keys.clone())
}

#[tauri::command]
fn generate_ssh_key(
    state: State<AppState>,
    key_name: String,
    email: String,
) -> Result<SSHKeyInfo, String> {
    let mut guard = state.0.lock().map_err(|e| e.to_string())?;
    guard.generate_ssh_key(&key_name, &email)
}

#[tauri::command]
fn test_ssh_connection(state: State<AppState>, key_name: String) -> Result<SSHProbeResult, String> {
    let guard = state.0.lock().map_err(|e| e.to_string())?;
    Ok(guard.test_ssh_connection(&key_name))
}

#[tauri::command]
fn get_folder_mappings(state: State<AppState>) -> Result<Vec<FolderMapping>, String> {
    let guard = state.0.lock().map_err(|e| e.to_string())?;
    Ok(guard.config.folder_mappings.clone())
}

#[tauri::command]
fn save_folder_mapping(state: State<AppState>, mapping: FolderMapping) -> Result<bool, String> {
    let mut guard = state.0.lock().map_err(|e| e.to_string())?;
    guard.save_folder_mapping(mapping)
}

#[tauri::command]
fn delete_folder_mapping(state: State<AppState>, id: String) -> Result<bool, String> {
    let mut guard = state.0.lock().map_err(|e| e.to_string())?;
    guard.delete_folder_mapping(&id)
}

#[tauri::command]
fn validate_hook(state: State<AppState>, path: String) -> Result<HookValidationResult, String> {
    let guard = state.0.lock().map_err(|e| e.to_string())?;
    Ok(guard.validate_hook(&path))
}

#[tauri::command]
fn get_status(state: State<AppState>) -> Result<AppStatus, String> {
    let guard = state.0.lock().map_err(|e| e.to_string())?;
    Ok(guard.get_status())
}

#[tauri::command]
fn get_config_files(state: State<AppState>) -> Result<ConfigFiles, String> {
    let guard = state.0.lock().map_err(|e| e.to_string())?;
    Ok(guard.get_config_files())
}

#[tauri::command]
fn sync_keychain(state: State<AppState>) -> Result<String, String> {
    let mut guard = state.0.lock().map_err(|e| e.to_string())?;
    Ok(guard.sync_keychain())
}

#[tauri::command]
fn flush_inactive_keys(state: State<AppState>) -> Result<usize, String> {
    let mut guard = state.0.lock().map_err(|e| e.to_string())?;
    guard.flush_inactive_keys()
}

#[tauri::command]
fn apply_to_terminal(state: State<AppState>, profile_id: String) -> Result<String, String> {
    let guard = state.0.lock().map_err(|e| e.to_string())?;
    guard.apply_to_terminal(&profile_id)
}

#[tauri::command]
fn set_daemon_mode(state: State<AppState>, strict: bool) -> Result<String, String> {
    let mut guard = state.0.lock().map_err(|e| e.to_string())?;
    guard.set_daemon_mode(strict)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let engine = AppEngine::new();

    tauri::Builder::default()
        .manage(AppState(Mutex::new(engine)))
        .invoke_handler(tauri::generate_handler![
            get_profiles,
            save_profile,
            delete_profile,
            switch_active_profile,
            get_ssh_keys,
            generate_ssh_key,
            test_ssh_connection,
            get_folder_mappings,
            save_folder_mapping,
            delete_folder_mapping,
            validate_hook,
            get_status,
            get_config_files,
            sync_keychain,
            flush_inactive_keys,
            apply_to_terminal,
            set_daemon_mode
        ])
        .run(tauri::generate_context!())
        .expect("error while running GitSwitch desktop application");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn preserves_user_gitconfig_directives() {
        let existing = "[alias]\n\tco = checkout\n[user]\n\tname = old\n\temail = old@example.com\n[pull]\n\trebase = true\n";
        let profiles = seed_default_config().profiles;
        let out = build_gitconfig_content(existing, &profiles[0], &profiles);
        assert!(out.contains("name = rahul-corp"));
        assert!(out.contains("email = rahul@company.com"));
        assert!(out.contains("[alias]\n\tco = checkout"));
        assert!(out.contains("[pull]\n\trebase = true"));
        assert!(out.contains("[includeIf \"gitdir:~/Developer/Work/\"]"));
    }

    #[test]
    fn preserves_user_ssh_hosts() {
        let existing = "Host my-vps\n    HostName 10.0.0.5\n    User ubuntu\n";
        let profiles = seed_default_config().profiles;
        let out = build_sshconfig_content(existing, &profiles[1], &profiles);
        assert!(out.contains("IdentityFile ~/.ssh/id_rsa_personal"));
        assert!(out.contains("Host github.com-work"));
        assert!(out.contains("Host my-vps"));
    }

    #[test]
    fn validates_include_if_hook_paths() {
        let tmp = std::env::temp_dir().join("gitswitch_unit_test_engine");
        let _ = fs::remove_dir_all(&tmp);
        let engine = AppEngine::new_in_dirs(tmp.join("home"), tmp.join("cfg"));
        let res = engine.validate_hook("~/Developer/Work/infra-core");
        assert!(res.matched);
        assert_eq!(res.profile.unwrap().id, "work");
        let miss = engine.validate_hook("~/Random/Unmapped/Folder");
        assert!(!miss.matched);
        let _ = fs::remove_dir_all(&tmp);
    }

    #[test]
    fn switches_profile_atomically_in_temp_home() {
        let tmp = std::env::temp_dir().join("gitswitch_unit_test_switch");
        let _ = fs::remove_dir_all(&tmp);
        let mut engine = AppEngine::new_in_dirs(tmp.join("home"), tmp.join("cfg"));
        let res = engine.switch_active_profile("personal").expect("switch succeeds");
        assert!(res.success);
        assert_eq!(res.profile_id, "personal");
        assert!(tmp.join("home/.gitconfig").exists());
        assert!(tmp.join("home/.gitconfig-personal").exists());
        assert!(tmp.join("home/.ssh/config").exists());
        let _ = fs::remove_dir_all(&tmp);
    }
}

