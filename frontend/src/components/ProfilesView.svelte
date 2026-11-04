<script lang="ts">
  import { createEventDispatcher } from 'svelte';
  import { profiles, activeProfileId, switchProfile, deleteProfile } from '../stores/profiles';
  import { Button, Badge, Card } from './ui';
  import type { Profile } from '../types';

  const dispatch = createEventDispatcher();

  let switchingId = '';

  async function handleSwitch(profile: Profile) {
    if (profile.id === $activeProfileId) return;
    switchingId = profile.id;
    try {
      await switchProfile(profile.id);
    } finally {
      switchingId = '';
    }
  }

  async function handleDelete(profile: Profile) {
    if (confirm(`Are you sure you want to delete profile "${profile.name}"?`)) {
      try {
        await deleteProfile(profile.id);
      } catch (e: any) {
        alert(e?.message || 'Error deleting profile');
      }
    }
  }
</script>

<div class="flex flex-col w-full gap-4 max-w-5xl mx-auto">
  <!-- Top Telemetry & Switcher Quick Bar -->
  <div class="flex flex-col sm:flex-row sm:items-center justify-between gap-3 bg-[#0d141e] px-5 py-3 rounded-xl border border-[#182330]">
    <div class="flex items-center gap-3 font-mono text-xs">
      <span class="text-slate-500 uppercase tracking-wider text-[11px]">Identities Matrix</span>
      <span class="text-slate-600">/</span>
      <span class="text-slate-300 font-semibold">{$profiles.length} Configured Environments</span>
    </div>

    <div class="flex items-center gap-2 font-mono text-xs flex-wrap">
      <span class="text-slate-500 text-[11px]">Runtime Hook:</span>
      <span class="bg-[#14202d] border border-[#1e2d3d] px-2 py-0.5 rounded text-[11px] text-[#3fb950]">
        pre-commit-exec: ON
      </span>
      <span class="bg-[#14202d] border border-[#1e2d3d] px-2 py-0.5 rounded text-[11px] text-[#a2c9ff]">
        ssh-agent-sync: SYNCED
      </span>
      <Button 
        variant="primary" 
        size="sm"
        on:click={() => dispatch('add-identity')}
      >
        <span>+ Add Identity</span>
      </Button>
    </div>
  </div>

  <!-- Profiles Cards List -->
  <div class="flex flex-col gap-3">
    {#each $profiles as profile}
      {@const isActive = profile.id === $activeProfileId}
      <div 
        class="relative bg-[#0d141e] rounded-xl p-4 sm:p-5 border transition-all duration-200 shadow-lg
          {isActive ? 'border-[#2a3c50] shadow-[0_8px_30px_rgba(0,0,0,0.6)]' : 'border-[#1b2636] hover:border-[#27384b]'}"
      >
        <!-- Glow accent pillar on left edge -->
        {#if isActive}
          <div 
            class="absolute left-0 top-0 bottom-0 w-1 rounded-l-xl shadow-[0_0_12px_rgba(63,185,80,0.65)]"
            style="background-color: {profile.color || '#3fb950'};"
          ></div>
        {/if}

        <div class="flex flex-col gap-3 {isActive ? 'pl-2' : ''}">
          <!-- Header row -->
          <div class="flex items-center justify-between gap-3 flex-wrap">
            <div class="flex items-center gap-3">
              <div class="flex items-center gap-2">
                <span class="text-base font-semibold text-white tracking-tight">{profile.name}</span>
                {#if profile.shortcut}
                  <span class="bg-[#182330] px-1.5 py-0.5 rounded font-mono text-[10px] text-slate-400 border border-[#233547]">
                    {profile.shortcut}
                  </span>
                {/if}
              </div>

              {#if isActive}
                <div class="flex items-center gap-1.5 bg-[#3fb950]/15 border border-[#3fb950]/30 px-2.5 py-0.5 rounded">
                  <span class="w-1.5 h-1.5 rounded-full bg-[#3fb950] animate-pulse"></span>
                  <span class="font-mono text-[10px] text-[#3fb950] uppercase font-bold tracking-widest">CURRENT ACTIVE</span>
                </div>
              {:else}
                <span class="bg-[#14202d] border border-[#223348] px-2 py-0.5 rounded font-mono text-[10px] text-slate-500 uppercase tracking-wider">
                  Standby
                </span>
              {/if}
            </div>

            <!-- Actions -->
            <div class="flex items-center gap-2">
              {#if isActive}
                <div class="bg-[#3fb950]/15 text-[#3fb950] border border-[#3fb950]/30 px-3 py-1 rounded font-mono text-xs flex items-center gap-1.5">
                  <span>✓</span>
                  <span>Active</span>
                </div>
              {:else}
                <Button 
                  variant="secondary" 
                  size="sm"
                  loading={switchingId === profile.id}
                  on:click={() => handleSwitch(profile)}
                >
                  <span>⇆ Switch</span>
                  {#if profile.shortcut}
                    <span class="opacity-60 text-[10px] ml-0.5">{profile.shortcut}</span>
                  {/if}
                </Button>
              {/if}

              <Button 
                variant="ghost" 
                size="sm"
                on:click={() => dispatch('edit-profile', profile)}
              >
                <span>Config</span>
              </Button>

              {#if $profiles.length > 1}
                <button 
                  on:click={() => handleDelete(profile)}
                  class="p-1.5 text-slate-500 hover:text-red-400 rounded hover:bg-[#1a2330] transition-colors text-xs font-mono"
                  title="Delete profile"
                >
                  ✕
                </button>
              {/if}
            </div>
          </div>

          <!-- Identity Credentials Bar -->
          <div class="grid grid-cols-1 md:grid-cols-12 gap-3 items-center bg-[#070d13]/70 p-3 rounded-lg border border-[#162230] font-mono text-xs">
            <div class="md:col-span-4 flex items-center gap-2 min-w-0">
              <span 
                class="w-2 h-2 rounded-full shrink-0" 
                style="background-color: {profile.color || '#3fb950'};"
              ></span>
              <div class="flex items-baseline gap-1.5 truncate">
                <span class="text-white font-medium">{profile.username}</span>
                <span class="text-slate-500 text-[11px] truncate">@{profile.username}</span>
              </div>
            </div>

            <div class="md:col-span-4 flex items-center gap-1.5 min-w-0">
              <span class="text-slate-500 text-[11px] shrink-0">mailto:</span>
              <span class="text-slate-300 truncate">{profile.email}</span>
            </div>

            <div class="md:col-span-4 flex items-center justify-start md:justify-end gap-2 flex-wrap">
              <span class="bg-[#121c26] border border-[#1e2d3e] px-2 py-0.5 rounded text-[11px] text-[#a2c9ff] flex items-center gap-1">
                <span>🔑 {profile.ssh_key}</span>
              </span>
              {#if profile.signingKey}
                <span class="bg-[#121c26] border border-[#3fb950]/30 px-2 py-0.5 rounded text-[11px] text-[#3fb950] flex items-center gap-1">
                  <span>✓ {profile.signingKey}</span>
                </span>
              {/if}
            </div>
          </div>

          <!-- Bound Directory Scopes -->
          <div class="flex items-center gap-2 bg-[#090e15] px-3 py-1.5 rounded-lg border border-[#162230] font-mono text-xs min-w-0">
            <span class="text-slate-500 uppercase text-[10px] tracking-wider shrink-0">Scope Paths:</span>
            <span class="text-slate-300 truncate text-[11px]">
              {profile.scope_paths?.join(' · ') || 'Unmapped'}
            </span>
            <span class="ml-auto text-slate-500 shrink-0 text-[10px] hidden sm:inline">auto-switch on cd</span>
          </div>
        </div>
      </div>
    {/each}
  </div>
</div>
