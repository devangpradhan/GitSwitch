<script lang="ts">
  import { onMount } from 'svelte';
  import { sshKeys, loadSSHKeys, testConnection, isProbing, terminalLines, probeLatency, probeTime, syncKeychain, isSyncing, flushInactive, isFlushing } from '../stores/ssh';
  import { Button, Badge } from './ui';
  import type { SSHKeyInfo } from '../types';

  let currentKeyProbe = 'id_ed25519_work';
  let copiedFingerprint: string | null = null;
  let copiedPublicKey: string | null = null;
  let syncSuccessMsg = '';

  onMount(() => {
    loadSSHKeys();
  });

  async function handleCopyFingerprint(key: SSHKeyInfo) {
    await navigator.clipboard.writeText(key.fingerprint);
    copiedFingerprint = key.name;
    setTimeout(() => { copiedFingerprint = null; }, 1800);
  }

  async function handleCopyPubKey(key: SSHKeyInfo) {
    await navigator.clipboard.writeText(key.public_key);
    copiedPublicKey = key.name;
    setTimeout(() => { copiedPublicKey = null; }, 1800);
  }

  async function handleTest(key: SSHKeyInfo) {
    currentKeyProbe = key.name;
    await testConnection(key.name);
  }

  async function handleSync() {
    const msg = await syncKeychain();
    syncSuccessMsg = msg;
    setTimeout(() => { syncSuccessMsg = ''; }, 2500);
  }

  function clearTerminal() {
    terminalLines.set(['Console cleared. Waiting for SSH connection test...']);
  }
</script>

<div class="flex flex-col w-full gap-5 max-w-5xl mx-auto">
  <!-- Top Agent Status Bar / Command Deck -->
  <div class="bg-[#0d141e] rounded-xl p-5 border border-[#182330] shadow-sm relative overflow-hidden">
    <div class="absolute -right-16 -top-16 w-64 h-64 bg-[#3fb950]/5 rounded-full blur-3xl pointer-events-none"></div>

    <div class="relative z-10 flex flex-col lg:flex-row lg:items-center justify-between gap-4">
      <div class="space-y-1">
        <div class="flex items-center gap-2 font-mono text-[11px] text-slate-500">
          <span class="text-[#3fb950]">gittu://crypt/ssh-agent</span>
          <span>·</span>
          <span>SYSTEM ENCLAVE INTEGRATED</span>
        </div>
        <div class="flex items-center gap-2.5 flex-wrap">
          <div class="w-8 h-8 rounded bg-[#16202c] border border-[#233547] flex items-center justify-center text-[#3fb950]">
            🔑
          </div>
          <h1 class="text-base font-semibold text-white tracking-tight">
            SSH-Agent Keyring & Cryptographic Identities
          </h1>
          <div class="inline-flex items-center gap-1.5 px-2.5 py-0.5 rounded bg-[#3fb950]/10 text-[#3fb950] font-mono text-[11px]">
            <span class="w-1.5 h-1.5 rounded-full bg-[#3fb950] animate-pulse"></span>
            <span>AGENT DAEMON ACTIVE · {$sshKeys.length} Keys In Memory</span>
          </div>
        </div>
      </div>

      <div class="flex items-center gap-2 flex-wrap font-mono text-xs">
        <div class="flex items-center gap-1.5 px-3 py-1.5 rounded bg-[#070b10] border border-[#1b2636] text-slate-400 text-[11px]">
          <span>Passphrase Cache:</span>
          <span class="text-white font-semibold">TTL: Persistent</span>
        </div>
        <Button 
          variant="secondary" 
          size="sm"
          loading={$isSyncing}
          on:click={handleSync}
        >
          <span>{syncSuccessMsg || '↻ Sync Keychain'}</span>
        </Button>
        <Button 
          variant="ghost" 
          size="sm"
          loading={$isFlushing}
          on:click={flushInactive}
        >
          <span>Flush Inactive</span>
        </Button>
      </div>
    </div>

    <!-- Agent Telemetry Bar Strip -->
    <div class="mt-4 pt-3 border-t border-[#162230] flex flex-wrap items-center justify-between gap-3 text-slate-400 font-mono text-[11px]">
      <div class="flex items-center gap-4 flex-wrap">
        <div class="flex items-center gap-1.5">
          <span class="text-slate-500">Socket path:</span>
          <span class="text-slate-200 bg-[#060b10] px-2 py-0.5 rounded border border-[#182330]">
            ~/.ssh/agent.sock
          </span>
        </div>
        <div class="flex items-center gap-1.5">
          <span class="text-slate-500">Enclave:</span>
          <span class="text-[#a2c9ff]">Hardware Safe + FIDO2 Support</span>
        </div>
      </div>
      <div class="flex items-center gap-1.5">
        <span class="text-slate-500">Identity Switches (24h):</span>
        <span class="text-[#3fb950] font-bold">142 OK</span>
        <span class="text-slate-600">/ 0 ERR</span>
      </div>
    </div>
  </div>

  <!-- Active Keys Cards -->
  <div class="flex flex-col gap-3">
    <div class="flex items-center justify-between px-1 font-mono text-xs text-slate-500">
      <span class="uppercase tracking-wider">Configured Key Identifiers ({$sshKeys.length} registered)</span>
      <span>Isolated per Directory Context</span>
    </div>

    {#each $sshKeys as key}
      <div class="relative bg-[#0d141e] rounded-xl p-5 border border-[#1b2636] hover:border-[#27384b] transition-all shadow-md">
        {#if key.is_active_signing}
          <div class="absolute left-0 top-3 bottom-3 w-1 bg-[#3fb950] rounded-r"></div>
        {/if}

        <div class="flex flex-col lg:flex-row lg:items-start justify-between gap-4 {key.is_active_signing ? 'pl-2' : ''}">
          <div class="space-y-2 flex-1 min-w-0 font-mono text-xs">
            <!-- Title Row -->
            <div class="flex items-center gap-2.5 flex-wrap">
              <span class="text-sm font-bold text-white">{key.name}</span>
              {#if key.is_active_signing}
                <span class="px-2 py-0.5 rounded bg-[#3fb950] text-black text-[10px] font-bold tracking-wide">
                  ACTIVE GIT SIGNING
                </span>
              {:else}
                <span class="px-2 py-0.5 rounded bg-[#182330] text-slate-400 text-[10px] border border-[#233547]">
                  STANDBY GIT KEY
                </span>
              {/if}
              <span class="px-2 py-0.5 rounded bg-[#070b10] text-[#3fb950] text-[10px] border border-[#182330] flex items-center gap-1">
                <span class="w-1.5 h-1.5 rounded-full bg-[#3fb950]"></span>
                <span>Loaded in Memory</span>
              </span>
            </div>

            <!-- Specs Grid -->
            <div class="grid grid-cols-1 md:grid-cols-2 gap-2 text-slate-400 text-[11px]">
              <div>
                <span class="text-slate-500">Bound Profile:</span>
                <span class="text-slate-200 font-medium ml-1">{key.bound_profile}</span>
              </div>
              <div class="flex items-center gap-2">
                <span class="text-slate-500">Algorithm:</span>
                <span class="text-[#a2c9ff]">{key.algorithm}</span>
              </div>
            </div>

            <!-- Fingerprint container with copy button -->
            <div class="flex items-center gap-2 p-1.5 px-3 bg-[#060b10] border border-[#162230] rounded-lg max-w-2xl text-[11px]">
              <span class="text-slate-500 shrink-0">SHA-256:</span>
              <span class="text-slate-200 truncate select-all">{key.fingerprint}</span>
              <button 
                type="button"
                on:click={() => handleCopyFingerprint(key)}
                class="shrink-0 p-1 text-slate-400 hover:text-[#3fb950] transition-colors"
                title="Copy SHA256 Fingerprint"
              >
                {#if copiedFingerprint === key.name}
                  <span class="text-emerald-400 font-bold">✓ Copied</span>
                {:else}
                  <span>📋</span>
                {/if}
              </button>
            </div>

            <!-- Telemetry Stats -->
            <div class="flex items-center gap-3 text-slate-500 text-[11px] pt-1">
              <span>Auths: <strong class="text-slate-300">{key.auths_today || 12} today</strong></span>
              <span>·</span>
              <span>Expires: <strong class="text-slate-300">{key.ttl}</strong></span>
              <span>·</span>
              <span>Signed commits: <strong class="text-[#3fb950]">{key.signed_commits || 'Verified'}</strong></span>
            </div>
          </div>

          <!-- Actions -->
          <div class="flex items-center gap-2 shrink-0 self-end lg:self-center">
            <Button 
              variant="secondary" 
              size="sm"
              on:click={() => handleCopyPubKey(key)}
            >
              {#if copiedPublicKey === key.name}
                <span class="text-[#3fb950]">✓ Public Key Copied</span>
              {:else}
                <span>📋 Copy Public Key</span>
              {/if}
            </Button>

            <Button 
              variant="primary" 
              size="sm"
              loading={$isProbing && currentKeyProbe === key.name}
              on:click={() => handleTest(key)}
            >
              <span>⚡ Test Connection</span>
            </Button>
          </div>
        </div>
      </div>
    {/each}
  </div>

  <!-- Interactive Terminal Connection Test Console / Drawer -->
  <div class="bg-[#060b10] rounded-xl p-4 border border-[#182330] shadow-2xl space-y-3 font-mono">
    <div class="flex items-center justify-between px-1">
      <div class="flex items-center gap-2">
        <div class="w-2.5 h-2.5 rounded-full bg-[#ff5f56]"></div>
        <div class="w-2.5 h-2.5 rounded-full bg-[#ffbd2e]"></div>
        <div class="w-2.5 h-2.5 rounded-full bg-[#27c93f]"></div>
        <span class="text-slate-500 text-xs ml-2">diagnostics://ssh-probe-terminal</span>
      </div>

      <div class="flex items-center gap-2 text-xs">
        <span class="px-2.5 py-0.5 rounded bg-[#101924] border border-[#1c2a3a] text-[#3fb950] font-semibold text-[11px]">
          {$probeLatency} · TLS v1.3 · Handshake OK (Status 1)
        </span>
        <button 
          on:click={clearTerminal}
          class="p-1 text-slate-500 hover:text-slate-300 text-xs" 
          title="Clear console buffer"
        >
          ✕ Clear
        </button>
      </div>
    </div>

    <!-- Terminal Viewport -->
    <div class="bg-[#05080c] rounded-lg p-4 font-mono text-xs space-y-1.5 select-text text-slate-200 overflow-x-auto border border-[#141e2a]">
      <div class="flex items-center gap-2 text-slate-500 pb-1 border-b border-[#111924]">
        <span class="text-[#3fb950] font-bold">git@gitswitch</span>
        <span>:</span>
        <span class="text-[#38bdf8] font-bold">~/.ssh</span>
        <span class="text-[#3fb950]">$</span>
        <span class="text-white">ssh -T -i ~/.ssh/{currentKeyProbe} git@github.com</span>
      </div>

      <div class="space-y-1 pt-1 text-[11px] text-slate-300 leading-relaxed">
        {#each $terminalLines as line}
          <div class="{line.includes('successfully authenticated') ? 'text-[#3fb950] font-semibold' : 'text-slate-400'}">
            {line}
          </div>
        {/each}
      </div>

      <div class="flex items-center gap-1.5 text-[#3fb950] pt-2 text-[11px]">
        <span>●</span>
        <span class="text-slate-500">SSH Session finished in</span>
        <span class="text-emerald-400 font-semibold">{$probeTime}</span>
        <span class="inline-block w-2 h-3.5 bg-[#3fb950] animate-pulse ml-1"></span>
      </div>
    </div>
  </div>
</div>
