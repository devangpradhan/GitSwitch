import { writable } from 'svelte/store';
import type { AppStatus } from '../types';
import { GetStatus } from '../wailsjs/go/main/App';

export const appStatus = writable<AppStatus>({
  version: '1.0.0',
  pid: 49201,
  daemon_mode: 'STRICT_DISPATCH',
  daemon_enabled: true,
  hook_type: 'includeIf.gitdir',
  last_switch_latency_ms: 0.18,
  total_profiles: 4,
  total_mappings: 4,
  total_keys: 4,
  gitconfig_path: '~/.gitconfig',
  sshconfig_path: '~/.ssh/config',
  socket_path: '~/.ssh/agent.sock',
});

export async function loadAppStatus() {
  try {
    const s = await GetStatus();
    appStatus.set(s);
    return s;
  } catch (err) {
    console.error('Failed to load status:', err);
  }
}
