import { writable } from 'svelte/store';
import type { SSHKeyInfo, SSHProbeResult } from '../types';
import { GetSSHKeys, TestSSHConnection, SyncKeychain as apiSyncKeychain, FlushInactiveKeys as apiFlushInactive } from '../wailsjs/go/main/App';

export const sshKeys = writable<SSHKeyInfo[]>([]);
export const isProbing = writable<boolean>(false);
export const probeResult = writable<SSHProbeResult | null>(null);
export const isSyncing = writable<boolean>(false);
export const isFlushing = writable<boolean>(false);
export const terminalLines = writable<string[]>([
  'OpenSSH_9.6p1, LibreSSL 3.3.6',
  'debug1: Connecting to github.com [140.82.121.4] port 22.',
  'debug1: Connection established.',
  'debug1: identity file ~/.ssh/id_ed25519_work type 3',
  'Hi rahul-corp! You have successfully authenticated, but GitHub does not provide shell access.',
  'debug1: Exit status 1 (GitHub expected handshake exit code)'
]);
export const probeLatency = writable<string>('28ms');
export const probeTime = writable<string>('0.028s');

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
