export interface Profile {
  id: string;
  name: string;
  username: string;
  email: string;
  ssh_key: string;
  ssh_key_path?: string;
  signing_key?: string;
  scope_paths: string[];
  color: string;
  tag: string;
  shortcut?: string;
  custom_ssh_command?: string;
}

export interface FolderMapping {
  id: string;
  path_pattern: string;
  profile_id: string;
  profile_name: string;
  is_active: boolean;
  ssh_key: string;
}

export interface SSHKeyInfo {
  name: string;
  path: string;
  algorithm: string;
  fingerprint: string;
  public_key: string;
  bound_profile: string;
  loaded_in_memory: boolean;
  is_active_signing: boolean;
  is_fido2?: boolean;
  ttl: string;
  auths_today?: number;
  signed_commits?: string;
}

export interface AppSettings {
  active_profile_id: string;
  daemon_mode: string;
  daemon_enabled: boolean;
  hook_type: string;
  last_switch_latency_ms: number;
  sync_keychain: boolean;
  passphrase_cache_ttl: string;
  socket_path: string;
}

export interface SwitchResult {
  success: boolean;
  profile_id: string;
  profile_name: string;
  latency_ms: number;
  message: string;
  gitconfig_updated: boolean;
  sshconfig_updated: boolean;
}

export interface SSHProbeResult {
  success: boolean;
  key_name: string;
  host: string;
  username: string;
  latency_ms: number;
  terminal_output: string;
  exit_code: number;
  tls_version: string;
}

export interface HookValidationResult {
  input_path: string;
  normalized_path: string;
  matched: boolean;
  matched_pattern?: string;
  profile?: Profile;
  latency_ms: number;
}

export interface ConfigFiles {
  gitconfig_path: string;
  gitconfig_content: string;
  sshconfig_path: string;
  sshconfig_content: string;
}

export interface AppStatus {
  version: string;
  pid: number;
  active_profile?: Profile;
  daemon_mode: string;
  daemon_enabled: boolean;
  hook_type: string;
  last_switch_latency_ms: number;
  total_profiles: number;
  total_mappings: number;
  total_keys: number;
  gitconfig_path: string;
  sshconfig_path: string;
  socket_path: string;
}
