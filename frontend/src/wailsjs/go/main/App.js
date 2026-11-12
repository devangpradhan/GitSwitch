// @ts-check
import { invoke } from '@tauri-apps/api/core';

const isTauri = () => typeof window !== 'undefined' && Boolean(window.__TAURI_INTERNALS__);

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
  return Promise.resolve(getStored('profiles', []));
}

export function SaveProfile(profile) {
  if (isTauri()) return invoke('save_profile', { profile });
  if (window['go']?.['main']?.['App']?.['SaveProfile']) {
    return window['go']['main']['App']['SaveProfile'](profile);
  }
  const profiles = getStored('profiles', []);
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
  const profiles = getStored('profiles', []).filter(p => p.id !== profileId);
  setStored('profiles', profiles);
  return Promise.resolve(true);
}

export function SwitchActiveProfile(profileId) {
  if (isTauri()) return invoke('switch_active_profile', { profileId });
  if (window['go']?.['main']?.['App']?.['SwitchActiveProfile']) {
    return window['go']['main']['App']['SwitchActiveProfile'](profileId);
  }
  const profiles = getStored('profiles', []);
  const profile = profiles.find(p => p.id === profileId);
  if (!profile) {
    return Promise.reject(new Error(`Profile not found: ${profileId}`));
  }
  setStored('active_id', profileId);
  return Promise.resolve({
    success: true,
    profile_id: profile.id,
    profile_name: profile.name,
    latency_ms: 0.1,
    message: `Switched to ${profile.name}`,
    gitconfig_updated: false,
    sshconfig_updated: false
  });
}

export function GetSSHKeys() {
  if (isTauri()) return invoke('get_ssh_keys');
  if (window['go']?.['main']?.['App']?.['GetSSHKeys']) {
    return window['go']['main']['App']['GetSSHKeys']();
  }
  return Promise.resolve(getStored('keys', []));
}

export function GenerateSSHKey(keyName, email) {
  if (isTauri()) return invoke('generate_ssh_key', { keyName, email });
  return Promise.reject(new Error('SSH key generation requires the GitSwitch desktop runtime.'));
}

export function TestSSHConnection(keyName) {
  if (isTauri()) return invoke('test_ssh_connection', { keyName });
  if (window['go']?.['main']?.['App']?.['TestSSHConnection']) {
    return window['go']['main']['App']['TestSSHConnection'](keyName);
  }
  return Promise.resolve({
    success: false,
    key_name: keyName,
    host: 'git@github.com',
    username: '',
    latency_ms: 0,
    terminal_output: 'SSH connection probing requires the GitSwitch desktop runtime.',
    exit_code: -1,
    tls_version: 'N/A'
  });
}

export function GetFolderMappings() {
  if (isTauri()) return invoke('get_folder_mappings');
  if (window['go']?.['main']?.['App']?.['GetFolderMappings']) {
    return window['go']['main']['App']['GetFolderMappings']();
  }
  return Promise.resolve(getStored('mappings', []));
}

export function SaveFolderMapping(mapping) {
  if (isTauri()) return invoke('save_folder_mapping', { mapping });
  if (window['go']?.['main']?.['App']?.['SaveFolderMapping']) {
    return window['go']['main']['App']['SaveFolderMapping'](mapping);
  }
  const mappings = getStored('mappings', []);
  mappings.push(mapping);
  setStored('mappings', mappings);
  return Promise.resolve(true);
}

export function DeleteFolderMapping(id) {
  if (isTauri()) return invoke('delete_folder_mapping', { id });
  if (window['go']?.['main']?.['App']?.['DeleteFolderMapping']) {
    return window['go']['main']['App']['DeleteFolderMapping'](id);
  }
  const mappings = getStored('mappings', []).filter(m => m.id !== id);
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
    latency_ms: 0
  });
}

export function GetStatus() {
  if (isTauri()) return invoke('get_status');
  if (window['go']?.['main']?.['App']?.['GetStatus']) {
    return window['go']['main']['App']['GetStatus']();
  }
  const profiles = getStored('profiles', []);
  const mappings = getStored('mappings', []);
  const keys = getStored('keys', []);
  const activeId = getStored('active_id', '');
  return Promise.resolve({
    version: '1.0.0',
    pid: 0,
    active_profile: profiles.find(p => p.id === activeId) || profiles[0] || null,
    daemon_mode: 'STRICT_DISPATCH',
    daemon_enabled: true,
    hook_type: 'includeIf.gitdir',
    last_switch_latency_ms: 0,
    total_profiles: profiles.length,
    total_mappings: mappings.length,
    total_keys: keys.length,
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
  return Promise.resolve('Synchronized local keychain');
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
