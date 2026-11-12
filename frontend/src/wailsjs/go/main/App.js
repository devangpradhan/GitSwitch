// @ts-check
import { invoke } from '@tauri-apps/api/core';

const isTauri = () => typeof window !== 'undefined' && Boolean(window.__TAURI_INTERNALS__);

const defaultProfiles = [
  {
    id: 'work',
    name: 'Work',
    username: 'rahul-corp',
    email: 'rahul@company.com',
    ssh_key: 'id_ed25519_work',
    ssh_key_path: '~/.ssh/id_ed25519_work',
    signing_key: '0x9F4C2A1E',
    scope_paths: ['~/Developer/Work/*', '~/Projects/Client-Infra/*'],
    color: '#3fb950',
    tag: 'corp-id',
    shortcut: '⌘1'
  },
  {
    id: 'personal',
    name: 'Personal',
    username: 'rahulcodes',
    email: 'rahul.personal@gmail.com',
    ssh_key: 'id_rsa_personal',
    ssh_key_path: '~/.ssh/id_rsa_personal',
    scope_paths: ['~/Developer/Personal/*', '~/Sideprojects/*'],
    color: '#6366f1',
    tag: 'personal',
    shortcut: '⌘2'
  },
  {
    id: 'client',
    name: 'Client / Freelance',
    username: 'alex-consulting',
    email: 'alex@freelance-studio.dev',
    ssh_key: 'id_ed25519_client',
    ssh_key_path: '~/.ssh/id_ed25519_client',
    signing_key: '0x7B28C39F',
    scope_paths: ['~/Developer/Freelance/*', '~/Clients/*'],
    color: '#38bdf8',
    tag: 'agency-dev',
    shortcut: '⌘3'
  },
  {
    id: 'enterprise',
    name: 'Enterprise Bot',
    username: 'bastion-bot',
    email: 'bot@corp-bastion.internal',
    ssh_key: 'id_ed25519_oss',
    ssh_key_path: '~/.ssh/id_ed25519_oss',
    scope_paths: ['~/Enterprise/Infra/*'],
    color: '#bc8cff',
    tag: 'bastion-bot',
    shortcut: '⌘4'
  }
];

const defaultMappings = [
  {
    id: 'work-dev',
    path_pattern: '~/Developer/Work/*',
    profile_id: 'work',
    profile_name: 'Work',
    is_active: true,
    ssh_key: 'id_ed25519_work (256b ed25519)'
  },
  {
    id: 'work-infra',
    path_pattern: '~/Projects/Client-Infra/*',
    profile_id: 'work',
    profile_name: 'Work',
    is_active: true,
    ssh_key: 'id_ed25519_work (256b ed25519)'
  },
  {
    id: 'personal-dev',
    path_pattern: '~/Developer/Personal/*',
    profile_id: 'personal',
    profile_name: 'Personal',
    is_active: false,
    ssh_key: 'id_rsa_personal (4096b rsa)'
  },
  {
    id: 'client-freelance',
    path_pattern: '~/Developer/Freelance/*',
    profile_id: 'client',
    profile_name: 'Client / Freelance',
    is_active: false,
    ssh_key: 'id_ed25519_client (256b ed25519)'
  }
];

const defaultKeys = [
  {
    name: 'id_ed25519_work',
    path: '~/.ssh/id_ed25519_work',
    algorithm: 'Ed25519 · 256-bit elliptic curve',
    fingerprint: 'SHA256:7uK2xP+8mN9vB3qQ90Zx9uJ4kLmQpWs2eRtU89wX4yZ01',
    public_key: 'ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAIOrXb7y5N3PqLmK2wX9z0Y1uV4mN8pRs9tUe0q1wZ3x rahul@company.com',
    bound_profile: 'Work (@rahul-corp / rahul@company.com)',
    loaded_in_memory: true,
    is_active_signing: true,
    is_fido2: true,
    ttl: 'Session Persistent',
    auths_today: 48,
    signed_commits: '1,208 verified'
  },
  {
    name: 'id_rsa_personal',
    path: '~/.ssh/id_rsa_personal',
    algorithm: 'RSA · 4096-bit (Legacy Compatible)',
    fingerprint: 'SHA256:d8N7xQ41pL0m589uYqZ24V31nB50pOx421k92zV88',
    public_key: 'ssh-rsa AAAAB3NzaC1yc2EAAAADAQABAAACAQDQ9wX2yZ01kLmQpWs2eRtU89wX4yZ01 rahul.personal@gmail.com',
    bound_profile: 'Personal (@rahulcodes / rahul.personal@gmail.com)',
    loaded_in_memory: true,
    is_active_signing: false,
    is_fido2: false,
    ttl: 'Persistent',
    auths_today: 12,
    signed_commits: '342 commits'
  },
  {
    name: 'id_ed25519_oss',
    path: '~/.ssh/id_ed25519_oss',
    algorithm: 'Ed25519 · 256-bit Bastion Signed',
    fingerprint: 'SHA256:3pQ90Zx9uJ4kLmQpWs2eRtU89wX4yZ017uK2xP+8mN9vB',
    public_key: 'ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAIC9x8yZ01kLmQpWs2eRtU89wX4yZ01 bot@corp-bastion.internal',
    bound_profile: 'Enterprise Bot (@bastion-bot)',
    loaded_in_memory: true,
    is_active_signing: false,
    is_fido2: false,
    ttl: '4h remaining',
    auths_today: 82,
    signed_commits: '5,410 verified'
  },
  {
    name: 'id_ed25519_client',
    path: '~/.ssh/id_ed25519_client',
    algorithm: 'Ed25519 · 256-bit elliptic curve',
    fingerprint: 'SHA256:8kL9v+3qJ91zX4uV4mN8pRs9tUe0q1wZ3x7uK2xP+8mN9',
    public_key: 'ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAIK1mN8pRs9tUe0q1wZ3x7uK2xP+8mN9vB3qQ90Zx alex@freelance-studio.dev',
    bound_profile: 'Client / Freelance (@alex-consulting)',
    loaded_in_memory: true,
    is_active_signing: false,
    is_fido2: false,
    ttl: 'Session Persistent',
    auths_today: 19,
    signed_commits: '89 verified'
  }
];

function getStored(key, fallback) {
  try {
    const val = localStorage.getItem(`gitswitch_${key}`);
    return val ? JSON.parse(val) : fallback;
  } catch (e) {
    return fallback;
  }
}

function setStored(key, val) {
  try {
    localStorage.setItem(`gitswitch_${key}`, JSON.stringify(val));
  } catch (e) {}
}

export function GetProfiles() {
  if (isTauri()) return invoke('get_profiles');
  if (window['go']?.['main']?.['App']?.['GetProfiles']) {
    return window['go']['main']['App']['GetProfiles']();
  }
  return Promise.resolve(getStored('profiles', defaultProfiles));
}

export function SaveProfile(profile) {
  if (isTauri()) return invoke('save_profile', { profile });
  if (window['go']?.['main']?.['App']?.['SaveProfile']) {
    return window['go']['main']['App']['SaveProfile'](profile);
  }
  const profiles = getStored('profiles', defaultProfiles);
  const index = profiles.findIndex(p => p.id === profile.id);
  if (index >= 0) profiles[index] = profile;
  else profiles.push(profile);
  setStored('profiles', profiles);
  return Promise.resolve(true);
}

export function DeleteProfile(profileId) {
  if (isTauri()) return invoke('delete_profile', { profileId });
  if (window['go']?.['main']?.['App']?.['DeleteProfile']) {
    return window['go']['main']['App']['DeleteProfile'](profileId);
  }
  const profiles = getStored('profiles', defaultProfiles).filter(p => p.id !== profileId);
  setStored('profiles', profiles);
  return Promise.resolve(true);
}

export function SwitchActiveProfile(profileId) {
  if (isTauri()) return invoke('switch_active_profile', { profileId });
  if (window['go']?.['main']?.['App']?.['SwitchActiveProfile']) {
    return window['go']['main']['App']['SwitchActiveProfile'](profileId);
  }
  const profiles = getStored('profiles', defaultProfiles);
  const profile = profiles.find(p => p.id === profileId) || profiles[0];
  setStored('active_id', profileId);
  return Promise.resolve({
    success: true,
    profile_id: profile.id,
    profile_name: profile.name,
    latency_ms: 0.18,
    message: `Switched to ${profile.name} in 0.18ms`,
    gitconfig_updated: true,
    sshconfig_updated: true
  });
}

export function GetSSHKeys() {
  if (isTauri()) return invoke('get_ssh_keys');
  if (window['go']?.['main']?.['App']?.['GetSSHKeys']) {
    return window['go']['main']['App']['GetSSHKeys']();
  }
  return Promise.resolve(getStored('keys', defaultKeys));
}

export function GenerateSSHKey(keyName, email) {
  if (isTauri()) return invoke('generate_ssh_key', { keyName, email });
  return Promise.resolve({
    name: keyName,
    path: `~/.ssh/${keyName}`,
    algorithm: 'Ed25519 · 256-bit elliptic curve',
    fingerprint: 'SHA256:generatedPreviewFingerprint',
    public_key: `ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAIStub ${email}`,
    bound_profile: `Pending (${email})`,
    loaded_in_memory: true,
    is_active_signing: false,
    is_fido2: false,
    ttl: 'Session Persistent',
    auths_today: 0,
    signed_commits: 'Ready'
  });
}

export function TestSSHConnection(keyName) {
  if (isTauri()) return invoke('test_ssh_connection', { keyName });
  if (window['go']?.['main']?.['App']?.['TestSSHConnection']) {
    return window['go']['main']['App']['TestSSHConnection'](keyName);
  }
  return Promise.resolve({
    success: true,
    key_name: keyName,
    host: 'git@github.com',
    username: 'developer',
    latency_ms: 28,
    terminal_output: 'Simulated SSH probe output',
    exit_code: 1,
    tls_version: 'TLS v1.3'
  });
}

export function GetFolderMappings() {
  if (isTauri()) return invoke('get_folder_mappings');
  if (window['go']?.['main']?.['App']?.['GetFolderMappings']) {
    return window['go']['main']['App']['GetFolderMappings']();
  }
  return Promise.resolve(getStored('mappings', defaultMappings));
}

export function SaveFolderMapping(mapping) {
  if (isTauri()) return invoke('save_folder_mapping', { mapping });
  if (window['go']?.['main']?.['App']?.['SaveFolderMapping']) {
    return window['go']['main']['App']['SaveFolderMapping'](mapping);
  }
  const mappings = getStored('mappings', defaultMappings);
  mappings.push(mapping);
  setStored('mappings', mappings);
  return Promise.resolve(true);
}

export function DeleteFolderMapping(id) {
  if (isTauri()) return invoke('delete_folder_mapping', { id });
  if (window['go']?.['main']?.['App']?.['DeleteFolderMapping']) {
    return window['go']['main']['App']['DeleteFolderMapping'](id);
  }
  const mappings = getStored('mappings', defaultMappings).filter(m => m.id !== id);
  setStored('mappings', mappings);
  return Promise.resolve(true);
}

export function ValidateHook(path) {
  if (isTauri()) return invoke('validate_hook', { path });
  if (window['go']?.['main']?.['App']?.['ValidateHook']) {
    return window['go']['main']['App']['ValidateHook'](path);
  }
  return Promise.resolve({
    input_path: path,
    normalized_path: path,
    matched: false,
    latency_ms: 0.08
  });
}

export function GetStatus() {
  if (isTauri()) return invoke('get_status');
  if (window['go']?.['main']?.['App']?.['GetStatus']) {
    return window['go']['main']['App']['GetStatus']();
  }
  return Promise.resolve({
    version: '1.8.0',
    pid: 49201,
    active_profile: defaultProfiles[0],
    daemon_mode: 'STRICT_DISPATCH',
    daemon_enabled: true,
    hook_type: 'includeIf.gitdir',
    last_switch_latency_ms: 0.18,
    total_profiles: 4,
    total_mappings: 4,
    total_keys: 4,
    gitconfig_path: '~/.gitconfig',
    sshconfig_path: '~/.ssh/config',
    socket_path: '~/.ssh/agent.sock'
  });
}

export function GetConfigFiles() {
  if (isTauri()) return invoke('get_config_files');
  if (window['go']?.['main']?.['App']?.['GetConfigFiles']) {
    return window['go']['main']['App']['GetConfigFiles']();
  }
  return Promise.resolve({
    gitconfig_path: '~/.gitconfig',
    gitconfig_content: '',
    sshconfig_path: '~/.ssh/config',
    sshconfig_content: ''
  });
}

export function SyncKeychain() {
  if (isTauri()) return invoke('sync_keychain');
  if (window['go']?.['main']?.['App']?.['SyncKeychain']) {
    return window['go']['main']['App']['SyncKeychain']();
  }
  return Promise.resolve('Synchronized with system keychain & ssh-agent');
}

export function FlushInactiveKeys() {
  if (isTauri()) return invoke('flush_inactive_keys');
  if (window['go']?.['main']?.['App']?.['FlushInactiveKeys']) {
    return window['go']['main']['App']['FlushInactiveKeys']();
  }
  return Promise.resolve(0);
}

export function ApplyToTerminal(profileId) {
  if (isTauri()) return invoke('apply_to_terminal', { profileId });
  if (window['go']?.['main']?.['App']?.['ApplyToTerminal']) {
    return window['go']['main']['App']['ApplyToTerminal'](profileId);
  }
  return Promise.resolve('');
}

export function SetDaemonMode(strict) {
  if (isTauri()) return invoke('set_daemon_mode', { strict });
  return Promise.resolve(strict ? 'STRICT_DISPATCH' : 'PASSIVE_DISPATCH');
}
