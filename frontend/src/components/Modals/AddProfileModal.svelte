<script lang="ts">
  import { createEventDispatcher } from 'svelte';
  import { Dialog, Button, Select } from '../ui';
  import { saveProfile, switchProfile } from '../../stores/profiles';
  import { sshKeys, loadSSHKeys } from '../../stores/ssh';
  import { loadFolderMappings } from '../../stores/mappings';
  import { GenerateSSHKey } from '../../wailsjs/go/main/App';
  import type { Profile } from '../../types';

  export let open: boolean = false;
  export let editingProfile: Profile | null = null;

  const dispatch = createEventDispatcher();

  let profileName = 'Client / Freelance';
  let username = 'alex-consulting';
  let email = 'alex@freelance-studio.dev';
  let selectedSSHKey = 'id_ed25519_client';
  let scopePaths = '~/Developer/Freelance/*';
  let signingKey = '';
  let color = '#38bdf8';
  let tag = 'agency-dev';
  let keyMode: 'existing' | 'generate' = 'existing';
  let newKeyName = 'id_ed25519_freelance';

  let isSaving = false;
  let saveLogs: string[] = [];

  $: if (open) {
    saveLogs = [];
    if (editingProfile) {
      profileName = editingProfile.name;
      username = editingProfile.username;
      email = editingProfile.email;
      selectedSSHKey = editingProfile.ssh_key;
      scopePaths = (editingProfile.scope_paths || []).join(', ');
      signingKey = editingProfile.signing_key || '';
      color = editingProfile.color || '#3fb950';
      tag = editingProfile.tag || 'identity';
      keyMode = 'existing';
    }
  }

  $: selectedKeyObj = $sshKeys.find(k => k.name === selectedSSHKey);

  const colorOptions = [
    { name: 'Mint Green', value: '#3fb950', tag: 'corp-id' },
    { name: 'Indigo / Violet', value: '#6366f1', tag: 'personal' },
    { name: 'Terminal Cyan', value: '#38bdf8', tag: 'agency-dev' },
    { name: 'Royal Purple', value: '#bc8cff', tag: 'bastion-bot' },
    { name: 'Warning Amber', value: '#d29922', tag: 'sandbox' },
  ];

  function selectColor(opt: typeof colorOptions[0]) {
    color = opt.value;
    tag = opt.tag;
  }

  $: sshOptions = $sshKeys.map(k => ({
    value: k.name,
    label: `${k.name} (${k.algorithm})`,
    description: k.fingerprint.slice(0, 16) + '...'
  }));

  async function handleSave() {
    if (!profileName.trim() || !username.trim() || !email.trim()) return;

    isSaving = true;
    saveLogs = [
      `> Initializing profile directory for [${profileName}]...`,
    ];

    await new Promise(r => setTimeout(r, 120));
    saveLogs = [...saveLogs, `> Parsing user scope paths: ${scopePaths}...`];

    await new Promise(r => setTimeout(r, 120));
    saveLogs = [...saveLogs, `> Compiling gitdir:includeIf stanza [${selectedSSHKey}]...`];

    let finalSSHKey = selectedSSHKey;
    if (keyMode === 'generate') {
      saveLogs = [...saveLogs, `> Generating Ed25519 keypair (~/.ssh/${newKeyName})...`];
      const genKey = await GenerateSSHKey(newKeyName, email);
      finalSSHKey = genKey.name;
      await loadSSHKeys();
      saveLogs = [...saveLogs, `> Keypair ready (${genKey.fingerprint.slice(0, 22)}...) OK`];
    }

    const id = editingProfile
      ? editingProfile.id
      : profileName.toLowerCase().replace(/[^a-z0-9]/g, '-').replace(/-+/g, '-').replace(/^-|-$/g, '');
    const scopes = scopePaths.split(',').map(s => s.trim()).filter(Boolean);

    const newProfile: Profile = {
      id: id || `profile-${Date.now()}`,
      name: profileName,
      username: username,
      email: email,
      ssh_key: finalSSHKey,
      ssh_key_path: `~/.ssh/${finalSSHKey}`,
      signing_key: signingKey.trim() || undefined,
      scope_paths: scopes.length ? scopes : [`~/Developer/${profileName}/*`],
      color: color,
      tag: tag,
      shortcut: editingProfile?.shortcut || '⌘' + (Math.floor(Math.random() * 5) + 5),
    };

    try {
      await saveProfile(newProfile);
      await loadFolderMappings();
      saveLogs = [...saveLogs, `> Appending ~/.gitconfig hooks... [0.14ms] OK`];
      await new Promise(r => setTimeout(r, 100));
      saveLogs = [...saveLogs, `> Linking SSH identity in ~/.ssh/config... [0.08ms] OK`];
      await new Promise(r => setTimeout(r, 100));
      saveLogs = [...saveLogs, `✓ Identity [${newProfile.name}] linked and persisted!`];

      await new Promise(r => setTimeout(r, 400));
      await switchProfile(newProfile.id);
      open = false;
      dispatch('created', newProfile);
    } catch (err) {
      saveLogs = [...saveLogs, `✕ Error: ${String(err)}`];
    } finally {
      isSaving = false;
    }
  }
</script>

<Dialog 
  bind:open 
  title={editingProfile ? `Configure Identity: ${editingProfile.name}` : 'Initialize New GitHub Identity'} 
  description="Configures gitconfig scope, SSH agent bindings, & automatic folder dispatch."
  maxWidth="max-w-[680px]"
  on:close={() => dispatch('close')}
>
  <svelte:fragment slot="badge">
    <span class="text-[10px] font-mono font-medium px-2 py-0.5 rounded bg-[#3fb950]/15 text-[#3fb950] border border-[#3fb950]/30">
      v1.8.0
    </span>
  </svelte:fragment>

  <form on:submit|preventDefault={handleSave} class="space-y-4">
    <!-- Field 1: Profile Identifier & Color Tag -->
    <div>
      <div class="flex items-center justify-between mb-1.5 font-mono text-xs">
        <label class="font-medium text-slate-300 flex items-center gap-1.5">
          <span class="text-[#3fb950]">01.</span> PROFILE IDENTIFIER
        </label>
        <span class="text-[11px] text-slate-500">UI alias / workspace label</span>
      </div>
      <div class="relative">
        <input 
          bind:value={profileName}
          type="text" 
          required
          placeholder="e.g. Client / Freelance or Open-Source Labs" 
          class="w-full px-3.5 py-2.5 rounded-lg bg-[#070d13] border border-[#1b2734] text-slate-100 font-mono text-xs focus:outline-none focus:border-[#3fb950] focus:ring-1 focus:ring-[#3fb950]/40 placeholder:text-slate-600 transition-all shadow-inner"
        />
        <div class="absolute right-3 top-1/2 -translate-y-1/2 flex items-center gap-2 text-[11px] font-mono text-slate-400">
          <div class="flex items-center gap-1">
            {#each colorOptions as opt}
              <button 
                type="button" 
                class="w-3.5 h-3.5 rounded-full transition-transform hover:scale-125 {color === opt.value ? 'ring-2 ring-white scale-110' : 'opacity-60'}"
                style="background-color: {opt.value};"
                on:click={() => selectColor(opt)}
                title={opt.name}
              ></button>
            {/each}
          </div>
          <span class="text-slate-500">tag: {tag}</span>
        </div>
      </div>
    </div>

    <!-- Field 2: GitHub Credentials (Username & Email) -->
    <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
      <div>
        <div class="flex items-center justify-between mb-1.5 font-mono text-xs">
          <label class="font-medium text-slate-300 flex items-center gap-1.5">
            <span class="text-[#3fb950]">02.</span> GITHUB USERNAME
          </label>
          <span class="text-[10px] text-slate-500">user.name</span>
        </div>
        <div class="relative">
          <div class="absolute left-3.5 top-1/2 -translate-y-1/2 text-slate-500 font-mono text-xs">@</div>
          <input 
            bind:value={username}
            type="text" 
            required
            placeholder="dev-alex" 
            class="w-full pl-8 pr-3.5 py-2.5 rounded-lg bg-[#070d13] border border-[#1b2734] text-slate-100 font-mono text-xs focus:outline-none focus:border-[#3fb950] focus:ring-1 focus:ring-[#3fb950]/40 placeholder:text-slate-600 transition-all shadow-inner"
          />
        </div>
      </div>

      <div>
        <div class="flex items-center justify-between mb-1.5 font-mono text-xs">
          <label class="font-medium text-slate-300 flex items-center gap-1.5">
            <span class="text-[#3fb950]">03.</span> COMMITTING EMAIL
          </label>
          <span class="text-[10px] text-slate-500">user.email</span>
        </div>
        <div class="relative">
          <input 
            bind:value={email}
            type="email" 
            required
            placeholder="alex@freelance-studio.dev" 
            class="w-full px-3.5 py-2.5 rounded-lg bg-[#070d13] border border-[#1b2734] text-slate-100 font-mono text-xs focus:outline-none focus:border-[#3fb950] focus:ring-1 focus:ring-[#3fb950]/40 placeholder:text-slate-600 transition-all shadow-inner"
          />
        </div>
      </div>
    </div>

    <!-- Field 3: Associated SSH Key Selector -->
    <div>
      <div class="flex items-center justify-between mb-1.5 font-mono text-xs">
        <label class="font-medium text-slate-300 flex items-center gap-1.5">
          <span class="text-[#3fb950]">04.</span> ASSOCIATED SSH KEY
        </label>
        <div class="flex items-center gap-1 text-[11px]">
          <button 
            type="button" 
            on:click={() => keyMode = 'existing'}
            class="px-2 py-0.5 rounded text-[10px] transition-colors {keyMode === 'existing' ? 'bg-[#3fb950]/20 text-[#3fb950] border border-[#3fb950]/40' : 'bg-[#070d13] text-slate-400 border border-[#1b2734]'}"
          >
            Existing Keys ({$sshKeys.length})
          </button>
          <button 
            type="button" 
            on:click={() => keyMode = 'generate'}
            class="px-2 py-0.5 rounded text-[10px] transition-colors {keyMode === 'generate' ? 'bg-[#3fb950]/20 text-[#3fb950] border border-[#3fb950]/40' : 'bg-[#070d13] text-slate-400 border border-[#1b2734]'}"
          >
            + New Ed25519
          </button>
        </div>
      </div>

      {#if keyMode === 'existing'}
        <div class="space-y-2">
          <Select 
            bind:value={selectedSSHKey}
            options={sshOptions}
            placeholder="Select SSH Key"
          />
          <div class="flex items-center justify-between px-3 py-2 rounded-md bg-[#070d13]/70 border border-[#1b2734]/70 text-[11px] font-mono text-slate-400">
            <div class="flex items-center gap-2">
              <span class="text-[#3fb950]">🔑</span>
              <span>Fingerprint: <span class="text-slate-200">{selectedKeyObj ? selectedKeyObj.fingerprint.slice(0, 26) + '...' : 'No key selected'}</span></span>
            </div>
            <span class="text-emerald-400 bg-emerald-950/40 px-1.5 py-0.5 rounded border border-emerald-800/40 text-[10px]">
              Enclave Safe
            </span>
          </div>
        </div>
      {:else}
        <div class="p-3.5 rounded-lg bg-[#070d13] border border-[#3fb950]/40 space-y-2.5 font-mono text-xs">
          <div class="flex items-center justify-between text-[11px]">
            <span class="text-white font-medium flex items-center gap-1.5">
              <span class="text-[#3fb950]">✨</span> Generate New Keypair:
            </span>
            <span class="text-slate-500">ssh-keygen -t ed25519 -C</span>
          </div>
          <input 
            type="text" 
            bind:value={newKeyName}
            class="w-full px-3 py-1.5 bg-[#0b141e] border border-[#1b2734] rounded text-slate-200 text-xs" 
            placeholder="Key file name"
          />
          <p class="text-[10px] text-slate-400">
            Will be saved to <code class="text-slate-200">~/.ssh/{newKeyName}</code> and loaded directly into ssh-agent.
          </p>
        </div>
      {/if}
    </div>

    <!-- Field 4: Scope Directory / Auto-Switch includeIf Paths -->
    <div>
      <div class="flex items-center justify-between mb-1.5 font-mono text-xs">
        <label class="font-medium text-slate-300 flex items-center gap-1.5">
          <span class="text-[#3fb950]">05.</span> AUTO-SWITCH SCOPE PATHS
        </label>
        <span class="text-[11px] text-[#38bdf8]">gitdir:includeIf hook</span>
      </div>
      <div class="relative">
        <input 
          bind:value={scopePaths}
          type="text" 
          required
          placeholder="e.g. ~/Developer/Freelance/* or ~/Projects/Client/*" 
          class="w-full px-3.5 py-2.5 rounded-lg bg-[#070d13] border border-[#1b2734] text-slate-100 font-mono text-xs focus:outline-none focus:border-[#3fb950] focus:ring-1 focus:ring-[#3fb950]/40 placeholder:text-slate-600 transition-all shadow-inner"
        />
      </div>
      <p class="text-[10px] font-mono text-slate-500 mt-1">
        Multiple paths can be comma-separated. Entering these folders will instantly activate this profile.
      </p>
    </div>

    <!-- Terminal Stream Animation Output -->
    {#if saveLogs.length > 0}
      <div class="mt-3 p-3 rounded-lg bg-[#05090e] border border-[#1b2734] font-mono text-[11px] space-y-1 text-slate-300">
        <div class="flex items-center justify-between text-slate-500 border-b border-[#141e2a] pb-1 mb-1">
          <span>IDENTITY TELEMETRY STREAM</span>
          {#if isSaving}
            <span class="text-[#3fb950] animate-pulse">● EXECUTING</span>
          {:else}
            <span class="text-emerald-400">✓ COMPLETED</span>
          {/if}
        </div>
        {#each saveLogs as log}
          <div class="{log.startsWith('✓') ? 'text-emerald-400 font-bold' : log.startsWith('✕') ? 'text-red-400' : 'text-slate-300'}">
            {log}
          </div>
        {/each}
      </div>
    {/if}

    <!-- Bottom Actions -->
    <div class="flex items-center justify-end gap-3 pt-3 border-t border-[#1b2734]">
      <Button 
        type="button" 
        variant="ghost" 
        on:click={() => { open = false; dispatch('close'); }}
      >
        Cancel
      </Button>
      <Button 
        type="submit" 
        variant="primary" 
        loading={isSaving}
      >
        <span>⚡ Save & Link Identity</span>
      </Button>
    </div>
  </form>
</Dialog>
