<script lang="ts">
  import { onMount } from 'svelte';
  import { folderMappings, currentValidation, validatePath, deleteFolderMapping, saveFolderMapping, loadFolderMappings } from '../stores/mappings';
  import { profiles, activeProfileId } from '../stores/profiles';
  import { Button, Badge, Switch, Dialog, Select } from './ui';
  import type { FolderMapping } from '../types';

  let testPath = '~/Developer/Work/infra-core';
  let daemonStrict = true;
  let addMappingOpen = false;

  let newPathPattern = '~/Projects/NewClient/*';
  let selectedProfileId = 'work';

  onMount(() => {
    loadFolderMappings();
    handleValidate();
  });

  async function handleValidate() {
    if (!testPath.trim()) return;
    try {
      await validatePath(testPath);
    } catch (e) {
      console.error(e);
    }
  }

  async function handleUnlink(mapping: FolderMapping) {
    if (confirm(`Unlink scope path "${mapping.path_pattern}"?`)) {
      await deleteFolderMapping(mapping.id);
    }
  }

  async function handleCreateMapping() {
    if (!newPathPattern.trim()) return;
    const p = $profiles.find(x => x.id === selectedProfileId) || $profiles[0];
    const newMapping: FolderMapping = {
      id: `map-${Date.now()}`,
      path_pattern: newPathPattern.trim(),
      profile_id: p.id,
      profile_name: p.name,
      is_active: $activeProfileId === p.id,
      ssh_key: p.ssh_key,
    };
    await saveFolderMapping(newMapping);
    addMappingOpen = false;
    handleValidate();
  }

  $: profileOptions = $profiles.map(p => ({
    value: p.id,
    label: p.name,
    description: `@${p.username}`
  }));
</script>

<div class="flex flex-col w-full gap-5 max-w-5xl mx-auto">
  <!-- Top Engine Explanation & Daemon Switch -->
  <div class="flex flex-col lg:flex-row lg:items-center justify-between gap-4 p-5 rounded-xl bg-[#0d141e] border border-[#182330] shadow-sm">
    <div class="flex flex-col gap-1.5 max-w-2xl font-mono text-xs">
      <div class="flex items-center gap-2">
        <span class="inline-block w-2 h-2 rounded-full bg-[#3fb950] animate-pulse"></span>
        <span class="text-[11px] uppercase tracking-wider text-[#3fb950] font-semibold">Runtime Engine Active</span>
        <span class="text-slate-500">· includeIf.gitdir</span>
      </div>
      <h2 class="text-base font-semibold text-white tracking-tight font-sans">
        Automatic Profile Detection
      </h2>
      <p class="text-slate-400 font-sans leading-relaxed">
        Sub-shell navigation hooks monitor file system transitions. Entering mapped directories triggers zero-latency 
        <code class="px-1.5 py-0.5 rounded bg-[#162230] text-[#3fb950] font-mono text-[11px]">~/.gitconfig</code> swapping 
        and immediate SSH identity re-binding in <code class="px-1.5 py-0.5 rounded bg-[#162230] text-[#a2c9ff] font-mono text-[11px]">ssh-agent</code>.
      </p>
    </div>

    <div class="flex items-center gap-4 self-start lg:self-center font-mono text-xs">
      <div class="flex flex-col items-end">
        <span class="text-slate-400 text-[11px]">Daemon Switcher</span>
        <span class="text-[#3fb950] font-semibold">{daemonStrict ? 'STRICT_DISPATCH' : 'PASSIVE_DISPATCH'}</span>
      </div>
      <Switch bind:checked={daemonStrict} />
    </div>
  </div>

  <!-- Interactive Live Path Resolver Probe Bar -->
  <div class="flex flex-col md:flex-row items-stretch md:items-center justify-between gap-3 px-5 py-3.5 rounded-xl bg-[#060b10] border border-[#182330]">
    <div class="flex items-center gap-2 flex-1 min-w-0 font-mono text-xs">
      <span class="text-[#3fb950] text-sm font-bold">⚡</span>
      <span class="text-slate-500">pwd:</span>
      <input 
        type="text" 
        bind:value={testPath}
        on:input={handleValidate}
        placeholder="Enter path to test includeIf matching (e.g. ~/Developer/Work/repo)"
        class="bg-transparent border-b border-transparent hover:border-[#2a3c50] focus:border-[#3fb950] text-slate-100 px-1 py-0.5 flex-1 focus:outline-none transition-colors"
      />
    </div>

    <!-- Resolved Scope Badge -->
    <div class="flex items-center gap-2 shrink-0 font-mono text-xs">
      <span class="text-slate-500 text-[11px] uppercase tracking-wider">Resolved Scope:</span>
      {#if $currentValidation && $currentValidation.matched}
        <div class="flex items-center gap-1.5 px-2.5 py-1 rounded bg-[#3fb950]/15 border border-[#3fb950]/30 text-[#3fb950]">
          <span class="w-1.5 h-1.5 rounded-full bg-[#3fb950]"></span>
          <span class="font-semibold">{$currentValidation.profile?.name} ({$currentValidation.profile?.ssh_key})</span>
          <span class="text-slate-500 text-[10px] ml-1">({$currentValidation.latency_ms.toFixed(2)}ms)</span>
        </div>
      {:else}
        <div class="flex items-center gap-1.5 px-2.5 py-1 rounded bg-[#182330] text-slate-400 border border-[#233547]">
          <span>Fallback Default (Work)</span>
        </div>
      {/if}
    </div>
  </div>

  <!-- Configured Workspaces Table -->
  <div class="flex flex-col gap-3">
    <div class="flex items-center justify-between px-1 font-mono text-xs">
      <div class="flex items-center gap-2">
        <span class="uppercase tracking-wider text-slate-500 text-[11px]">Configured Workspaces</span>
        <span class="px-2 py-0.5 rounded bg-[#162230] text-slate-300 text-[11px] font-semibold">
          {$folderMappings.length} Paths
        </span>
      </div>

      <div class="flex items-center gap-3">
        <span class="text-slate-500 text-[11px] hidden sm:inline">Evaluated sequentially (first-match wins)</span>
        <Button 
          variant="primary" 
          size="sm"
          on:click={() => addMappingOpen = true}
        >
          <span>+ Add Path</span>
        </Button>
      </div>
    </div>

    <div class="flex flex-col gap-2">
      {#each $folderMappings as mapping}
        {@const isCurrentMatch = $currentValidation?.matched_pattern === mapping.path_pattern}
        <div 
          class="group relative flex flex-col md:flex-row md:items-center justify-between p-4 rounded-xl bg-[#0d141e] border transition-all duration-150
            {isCurrentMatch ? 'border-[#3fb950]/50 shadow-[0_0_18px_rgba(63,185,80,0.15)] bg-[#0f1824]' : 'border-[#1b2636] hover:border-[#26374a]'}"
        >
          {#if isCurrentMatch}
            <div class="absolute left-0 top-3 bottom-3 w-1 bg-[#3fb950] rounded-r"></div>
          {/if}

          <div class="flex flex-col sm:flex-row sm:items-center gap-4 {isCurrentMatch ? 'pl-2' : ''}">
            <div class="flex items-center gap-3">
              <div class="w-8 h-8 rounded bg-[#162330] border border-[#233547] flex items-center justify-center shrink-0 text-slate-300">
                📁
              </div>
              <div class="flex flex-col font-mono text-xs">
                <div class="flex items-center gap-2">
                  <span class="text-white font-semibold text-xs">{mapping.path_pattern}</span>
                  {#if isCurrentMatch}
                    <span class="px-1.5 py-0.5 rounded text-[10px] bg-[#3fb950]/20 text-[#3fb950] font-bold border border-[#3fb950]/30">
                      ACTIVE MATCH
                    </span>
                  {/if}
                </div>
                <span class="text-slate-500 text-[11px]">
                  Target Profile: <span class="text-slate-200">{mapping.profile_name}</span>
                </span>
              </div>
            </div>

            <div class="hidden xl:flex items-center gap-1.5 font-mono text-[11px] text-[#a2c9ff] bg-[#14202d] border border-[#223348] px-2.5 py-1 rounded">
              <span>🔑 {mapping.ssh_key}</span>
            </div>
          </div>

          <div class="flex items-center justify-between sm:justify-end gap-3 mt-3 md:mt-0 font-mono text-xs">
            <div class="flex items-center gap-1.5 px-2 py-0.5 rounded text-[11px] {isCurrentMatch ? 'bg-[#3fb950]/15 text-[#3fb950] border border-[#3fb950]/30' : 'bg-[#14202d] text-slate-400 border border-[#1b2734]'}">
              <span class="w-1.5 h-1.5 rounded-full {isCurrentMatch ? 'bg-[#3fb950]' : 'bg-slate-500'}"></span>
              <span>{isCurrentMatch ? 'includeIf ACTIVE' : 'includeIf READY'}</span>
            </div>

            <button 
              on:click={() => handleUnlink(mapping)}
              class="p-1.5 rounded hover:bg-[#1a2636] text-slate-500 hover:text-red-400 transition-colors"
              title="Unlink path"
            >
              ✕
            </button>
          </div>
        </div>
      {/each}
    </div>
  </div>
</div>

<!-- Add Scope Path Modal -->
<Dialog 
  bind:open={addMappingOpen}
  title="Link New Directory to Identity"
  description="Automatically activates the selected Git identity when navigating into this folder."
  maxWidth="max-w-[540px]"
>
  <form on:submit|preventDefault={handleCreateMapping} class="space-y-4 font-mono text-xs">
    <div>
      <label class="block text-slate-300 mb-1.5 font-medium">DIRECTORY SCOPE PATTERN</label>
      <input 
        type="text" 
        bind:value={newPathPattern}
        required
        placeholder="e.g. ~/Developer/Projects/* or C:/Work/Clients/*"
        class="w-full px-3.5 py-2 rounded-lg bg-[#070d13] border border-[#1b2734] text-white focus:outline-none focus:border-[#3fb950]"
      />
      <span class="text-[10px] text-slate-500 mt-1 block">Glob suffix '/*' will match all sub-repositories inside this root.</span>
    </div>

    <div>
      <label class="block text-slate-300 mb-1.5 font-medium">TARGET IDENTITY</label>
      <Select 
        bind:value={selectedProfileId}
        options={profileOptions}
      />
    </div>

    <div class="flex items-center justify-end gap-3 pt-3 border-t border-[#1b2734]">
      <Button variant="ghost" type="button" on:click={() => addMappingOpen = false}>Cancel</Button>
      <Button variant="primary" type="submit">Link Path</Button>
    </div>
  </form>
</Dialog>
