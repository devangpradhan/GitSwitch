<script lang="ts">
  import { createEventDispatcher } from 'svelte';
  import { switchLatency } from '../stores/profiles';
  import { Quit, WindowMinimise, WindowToggleMaximise } from '../wailsjs/runtime/runtime';

  export let activeTab: string = 'dashboard';

  const dispatch = createEventDispatcher();
</script>

<header data-tauri-drag-region class="h-14 bg-[#0a1017] border-b border-[#182330] px-5 py-3 flex items-center justify-between select-none">
  <!-- Left: Window controls & Brand Logo -->
  <div class="flex items-center gap-4">
    <!-- Window Bullets -->
    <div class="flex items-center gap-2">
      <button 
        on:click={Quit}
        class="w-3 h-3 rounded-full bg-[#ff5f56] opacity-80 hover:opacity-100 transition-opacity cursor-pointer border border-[#e0443e]"
        title="Close"
      ></button>
      <button 
        on:click={WindowMinimise}
        class="w-3 h-3 rounded-full bg-[#ffbd2e] opacity-80 hover:opacity-100 transition-opacity cursor-pointer border border-[#dea123]"
        title="Minimise"
      ></button>
      <button 
        on:click={WindowToggleMaximise}
        class="w-3 h-3 rounded-full bg-[#27c93f] opacity-80 hover:opacity-100 transition-opacity cursor-pointer border border-[#1aab29]"
        title="Maximise"
      ></button>
    </div>

    <div class="h-4 w-[1px] bg-[#1d2a3a]"></div>

    <!-- App Brand -->
    <div class="flex items-center gap-2.5">
      <div class="w-6 h-6 rounded-lg bg-gradient-to-br from-[#3fb950] to-[#2ea043] flex items-center justify-center shadow-[0_0_12px_rgba(63,185,80,0.4)]">
        <svg class="w-3.5 h-3.5 text-black" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.6" stroke-linecap="round" stroke-linejoin="round">
          <path d="M13 2L3 14h9l-1 8 10-12h-9l1-8z"></path>
        </svg>
      </div>
      <span class="text-sm font-semibold tracking-tight text-white flex items-center gap-1.5 font-sans">
        GitSwitch
        <span class="text-[10px] font-mono font-medium px-1.5 py-0.5 rounded bg-[#162433] text-[#3fb950] border border-[#233547]">v1.8.0</span>
      </span>
    </div>

    <!-- Daemon Pulse Indicator -->
    <div class="hidden lg:flex items-center gap-2 pl-3">
      <span class="relative flex h-2 w-2">
        <span class="animate-ping absolute inline-flex h-full w-full rounded-full bg-[#3fb950] opacity-75"></span>
        <span class="relative inline-flex rounded-full h-2 w-2 bg-[#3fb950]"></span>
      </span>
      <span class="text-[11px] font-mono text-emerald-400 font-medium">DAEMON: STRICT_DISPATCH ({$switchLatency}ms)</span>
    </div>
  </div>

  <!-- Center Tabs Nav -->
  <nav class="flex items-center bg-[#070b10] p-1 rounded-xl border border-[#1b2636]">
    <button 
      class="flex items-center gap-2 px-3.5 py-1.5 rounded-lg text-xs font-medium transition-all {activeTab === 'dashboard' ? 'bg-[#14202d] text-[#3fb950] shadow-sm border border-[#223348]' : 'text-slate-400 hover:text-slate-200'}"
      on:click={() => dispatch('tab-change', 'dashboard')}
    >
      <svg class="w-3.5 h-3.5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 5a1 1 0 011-1h4a1 1 0 011 1v4a1 1 0 01-1 1H5a1 1 0 01-1-1V5zM14 5a1 1 0 011-1h4a1 1 0 011 1v4a1 1 0 01-1 1h-4a1 1 0 01-1-1V5zM4 15a1 1 0 011-1h4a1 1 0 011 1v4a1 1 0 01-1 1H5a1 1 0 01-1-1v-4zM14 15a1 1 0 011-1h4a1 1 0 011 1v4a1 1 0 01-1 1h-4a1 1 0 01-1-1v-4z"></path>
      </svg>
      <span>Dashboard</span>
      {#if activeTab === 'dashboard'}
        <span class="w-1.5 h-1.5 rounded-full bg-[#3fb950]"></span>
      {/if}
    </button>

    <button 
      class="flex items-center gap-2 px-3.5 py-1.5 rounded-lg text-xs font-medium transition-all {activeTab === 'profiles' ? 'bg-[#14202d] text-[#3fb950] shadow-sm border border-[#223348]' : 'text-slate-400 hover:text-slate-200'}"
      on:click={() => dispatch('tab-change', 'profiles')}
    >
      <svg class="w-3.5 h-3.5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M16 7a4 4 0 11-8 0 4 4 0 018 0zM12 14a7 7 0 00-7 7h14a7 7 0 00-7 7z"></path>
      </svg>
      <span>Profiles</span>
      {#if activeTab === 'profiles'}
        <span class="w-1.5 h-1.5 rounded-full bg-[#3fb950]"></span>
      {/if}
    </button>

    <button 
      class="flex items-center gap-2 px-3.5 py-1.5 rounded-lg text-xs font-medium transition-all {activeTab === 'folder-mappings' ? 'bg-[#14202d] text-[#3fb950] shadow-sm border border-[#223348]' : 'text-slate-400 hover:text-slate-200'}"
      on:click={() => dispatch('tab-change', 'folder-mappings')}
    >
      <svg class="w-3.5 h-3.5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 7v10a2 2 0 002 2h14a2 2 0 002-2V9a2 2 0 00-2-2h-6l-2-2H5a2 2 0 00-2 2z"></path>
      </svg>
      <span>Folder Mappings</span>
      {#if activeTab === 'folder-mappings'}
        <span class="w-1.5 h-1.5 rounded-full bg-[#3fb950]"></span>
      {/if}
    </button>

    <button 
      class="flex items-center gap-2 px-3.5 py-1.5 rounded-lg text-xs font-medium transition-all {activeTab === 'ssh-keys' ? 'bg-[#14202d] text-[#3fb950] shadow-sm border border-[#223348]' : 'text-slate-400 hover:text-slate-200'}"
      on:click={() => dispatch('tab-change', 'ssh-keys')}
    >
      <svg class="w-3.5 h-3.5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 7a2 2 0 012 2m4 0a6 6 0 01-7.743 5.743L11 17H9v2H7v2H4a1 1 0 01-1-1v-2.586a1 1 0 01.293-.707l5.964-5.964A6 6 0 1121 9z"></path>
      </svg>
      <span>SSH Keys</span>
      {#if activeTab === 'ssh-keys'}
        <span class="w-1.5 h-1.5 rounded-full bg-[#3fb950]"></span>
      {/if}
    </button>
  </nav>

  <!-- Right: Add Identity & Config Inspector Button -->
  <div class="flex items-center gap-3">
    <button 
      on:click={() => dispatch('open-add')}
      class="flex items-center gap-1.5 text-xs font-mono font-medium bg-[#131c26] hover:bg-[#1a2636] border border-[#233346] px-2.5 py-1.5 rounded-lg text-slate-300 hover:text-white transition-colors"
    >
      <span class="text-[#3fb950] font-bold text-sm">+</span>
      <span>Add</span>
      <kbd class="text-[10px] font-mono text-slate-500 bg-[#0c1219] px-1 py-0.5 rounded border border-[#1e2a38]">⌘N</kbd>
    </button>

    <button 
      on:click={() => dispatch('open-config')}
      class="w-7 h-7 rounded-lg bg-[#14202d] border border-[#25374b] flex items-center justify-center text-slate-300 hover:text-white cursor-pointer transition-colors" 
      title="Inspect System Configuration Files (⌘,)"
    >
      <svg class="w-3.5 h-3.5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z"></path>
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z"></path>
      </svg>
    </button>
  </div>
</header>
