<script lang="ts">
  import { createEventDispatcher } from 'svelte';
  import { activeProfile, switchLatency, isSwitching } from '../stores/profiles';
  import { testConnection } from '../stores/ssh';
  import { ApplyToTerminal } from '../wailsjs/go/main/App';
  import { Button, Badge } from './ui';

  const dispatch = createEventDispatcher();
  let copiedTerminal = false;
  let probing = false;

  async function handleApplyToTerminal() {
    if (!$activeProfile) return;
    try {
      const cmd = await ApplyToTerminal($activeProfile.id);
      await navigator.clipboard.writeText(cmd);
      copiedTerminal = true;
      setTimeout(() => {
        copiedTerminal = false;
      }, 2000);
    } catch (e) {
      console.error(e);
    }
  }

  async function handleTestProbe() {
    if (!$activeProfile) return;
    probing = true;
    try {
      await testConnection($activeProfile.ssh_key);
      dispatch('open-ssh-tab');
    } finally {
      probing = false;
    }
  }
</script>

{#if $activeProfile}
  <div class="w-full max-w-[840px] bg-[#0c131c] border border-[#1d2b3c] rounded-xl p-5 shadow-2xl relative overflow-hidden transition-all duration-300">
    <!-- Left Accent Glow Bar -->
    <div 
      class="absolute left-0 top-0 bottom-0 w-1.5 transition-colors duration-300"
      style="background-color: {$activeProfile.color || '#3fb950'}; box-shadow: 0 0 16px {$activeProfile.color || '#3fb950'};"
    ></div>

    <div class="flex flex-col gap-4 pl-2">
      <!-- Header Row: Active Profile Name & Badges -->
      <div class="flex flex-col sm:flex-row sm:items-center justify-between gap-3">
        <div class="flex items-center gap-3">
          <div class="flex items-center gap-2">
            <span class="text-base font-semibold text-white tracking-tight">{$activeProfile.name}</span>
            {#if $activeProfile.shortcut}
              <span class="bg-[#182330] px-1.5 py-0.5 rounded font-mono text-[10px] text-slate-400 border border-[#233547]">
                {$activeProfile.shortcut}
              </span>
            {/if}
          </div>

          <div class="flex items-center gap-1.5 px-2.5 py-0.5 rounded bg-[#3fb950]/15 border border-[#3fb950]/30 text-[#3fb950] font-mono text-[11px] font-semibold">
            <span class="w-1.5 h-1.5 rounded-full bg-[#3fb950] animate-pulse"></span>
            <span>CURRENT ACTIVE</span>
          </div>
        </div>

        <!-- Telemetry Pill -->
        <div class="flex items-center gap-2 font-mono text-[11px] text-slate-400">
          <span class="text-slate-500">SWITCH LATENCY:</span>
          <span class="text-emerald-400 font-semibold px-1.5 py-0.5 rounded bg-[#101924] border border-[#1b2736]">
            {$switchLatency}ms
          </span>
          <span class="text-slate-600">·</span>
          <span class="text-indigo-400">ZERO_DROP</span>
        </div>
      </div>

      <!-- Identity Credentials Grid -->
      <div class="grid grid-cols-1 md:grid-cols-12 gap-3 items-center bg-[#070d13]/80 p-3 rounded-lg border border-[#172230] font-mono text-xs">
        <!-- User Handle -->
        <div class="md:col-span-4 flex items-center gap-2 min-w-0">
          <span class="w-2 h-2 rounded-full bg-emerald-400/50 shrink-0"></span>
          <div class="flex items-baseline gap-1.5 truncate">
            <span class="text-white font-medium">{$activeProfile.username}</span>
            <span class="text-slate-500 text-[11px]">@{ $activeProfile.username }</span>
          </div>
        </div>

        <!-- Email -->
        <div class="md:col-span-4 flex items-center gap-1.5 min-w-0">
          <span class="text-slate-500 text-[11px] shrink-0">mailto:</span>
          <span class="text-slate-200 truncate">{$activeProfile.email}</span>
        </div>

        <!-- SSH Key & GPG Signing -->
        <div class="md:col-span-4 flex items-center justify-start md:justify-end gap-2 flex-wrap">
          <span class="bg-[#14202d] border border-[#223348] px-2 py-0.5 rounded text-[11px] text-[#a2c9ff] flex items-center gap-1">
            <span>🔑 {$activeProfile.ssh_key}</span>
          </span>
          {#if $activeProfile.signing_key}
            <span class="bg-[#14202d] border border-[#3fb950]/30 px-2 py-0.5 rounded text-[11px] text-[#3fb950] flex items-center gap-1">
              <span>✓ {$activeProfile.signing_key}</span>
            </span>
          {/if}
        </div>
      </div>

      <!-- Bound Directory Scopes & Actions Bar -->
      <div class="flex flex-col sm:flex-row sm:items-center justify-between gap-3 pt-1">
        <div class="flex items-center gap-2 bg-[#090e15] px-3 py-1.5 rounded-lg border border-[#182330] font-mono text-[11px] text-slate-400 min-w-0 flex-1">
          <span class="text-slate-500 uppercase tracking-wider text-[10px] shrink-0">SCOPE PATHS:</span>
          <span class="text-slate-300 truncate">
            {$activeProfile.scope_paths?.join(' · ') || 'Global scope'}
          </span>
          <span class="ml-auto text-emerald-400/80 text-[10px] shrink-0 bg-[#0e1722] px-1.5 py-0.5 rounded">
            includeIf.gitdir
          </span>
        </div>

        <!-- Action Triggers -->
        <div class="flex items-center gap-2 shrink-0">
          <Button 
            variant="secondary" 
            size="sm"
            on:click={handleApplyToTerminal}
          >
            {#if copiedTerminal}
              <span class="text-[#3fb950]">✓ Commands Copied!</span>
            {:else}
              <span>⚡ Apply to Terminal</span>
            {/if}
          </Button>

          <Button 
            variant="primary" 
            size="sm"
            loading={probing}
            on:click={handleTestProbe}
          >
            <span>⚡ Test Probe</span>
          </Button>
        </div>
      </div>
    </div>
  </div>
{/if}
