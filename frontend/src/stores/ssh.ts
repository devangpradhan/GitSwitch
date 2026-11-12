import { writable } from 'svelte/store';
import type { SSHKeyInfo, SSHProbeResult } from '../types';
import { GetSSHKeys, TestSSHConnection, SyncKeychain as apiSyncKeychain, FlushInactiveKeys as apiFlushInactive } from '../wailsjs/go/main/App';

export const sshKeys = writable<SSHKeyInfo[]>([]);
export const isProbing = writable<boolean>(false);
export const probeResult = writable<SSHProbeResult | null>(null);
export const isSyncing = writable<boolean>(false);
export const isFlushing = writable<boolean>(false);
export const terminalLines = writable<string[]>([
  'Ready. Select a real SSH key in ~/.ssh/ and click "Test Connection" to run `ssh -T -i <key> git@github.com`.'
]);
export const probeLatency = writable<string>('--');
export const probeTime = writable<string>('--');

export async function loadSSHKeys() {
  try {
    const list = await GetSSHKeys();
    sshKeys.set(list);
    return list;
  } catch (err) {
    console.error('Failed to load SSH keys:', err);
    return [];
  }
}

export async function testConnection(keyName: string) {
  isProbing.set(true);
  try {
    const res = await TestSSHConnection(keyName);
    probeResult.set(res);
    terminalLines.set(res.terminal_output.split('\n').filter(Boolean));
    probeLatency.set(`${Math.round(res.latency_ms)}ms`);
    probeTime.set(`${(res.latency_ms / 1000).toFixed(3)}s`);
    return res;
  } catch (err) {
    console.error('Probe failed:', err);
    throw err;
  } finally {
    isProbing.set(false);
  }
}

export async function syncKeychain() {
  isSyncing.set(true);
  try {
    const msg = await apiSyncKeychain();
    await loadSSHKeys();
    return msg;
  } finally {
    isSyncing.set(false);
  }
}

export async function flushInactive() {
  isFlushing.set(true);
  try {
    const count = await apiFlushInactive();
    await loadSSHKeys();
    return count;
  } finally {
    isFlushing.set(false);
  }
}
