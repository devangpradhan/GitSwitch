import { writable } from 'svelte/store';
import type { AppStatus } from '../types';
import { GetStatus } from '../wailsjs/go/main/App';
import { activeProfileId, switchLatency } from './profiles';

export const appStatus = writable<AppStatus>({
  version: '1.0.0',
  pid: 0,
  daemon_mode: 'STRICT_DISPATCH',
  daemon_enabled: true,
  hook_type: 'includeIf.gitdir',
  last_switch_latency_ms: 0,
  total_profiles: 0,
  total_mappings: 0,
  total_keys: 0,
  gitconfig_path: '~/.gitconfig',
  sshconfig_path: '~/.ssh/config',
  socket_path: '~/.ssh/agent.sock',
});

export async function loadAppStatus() {
  try {
    const s = await GetStatus();
    appStatus.set(s);
    if (s?.active_profile?.id) {
      activeProfileId.set(s.active_profile.id);
    }
    if (typeof s?.last_switch_latency_ms === 'number') {
      switchLatency.set(s.last_switch_latency_ms);
    }
    return s;
  } catch (err) {
    console.error('Failed to load status:', err);
  }
}
