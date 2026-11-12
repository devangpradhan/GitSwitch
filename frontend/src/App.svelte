<script lang="ts">
  import { onMount } from 'svelte';
  import TitleBar from './components/TitleBar.svelte';
  import DashboardRotary from './components/DashboardRotary.svelte';
  import ProfileCard from './components/ProfileCard.svelte';
  import ProfilesView from './components/ProfilesView.svelte';
  import FolderMappingsView from './components/FolderMappingsView.svelte';
  import SSHKeysView from './components/SSHKeysView.svelte';
  import AddProfileModal from './components/Modals/AddProfileModal.svelte';
  import ConfigViewerModal from './components/Modals/ConfigViewerModal.svelte';

  import { profiles, loadProfiles, activeProfile, switchProfile, switchLatency } from './stores/profiles';
  import { loadSSHKeys } from './stores/ssh';
  import { loadFolderMappings } from './stores/mappings';
  import { appStatus, loadAppStatus } from './stores/status';
  import type { Profile } from './types';

  let currentTab: string = 'dashboard';
  let isAddModalOpen: boolean = false;
  let isConfigModalOpen: boolean = false;
  let editingProfile: Profile | null = null;

  function openNewProfileModal() {
    editingProfile = null;
    isAddModalOpen = true;
  }

  function openEditProfileModal(profile: Profile) {
    editingProfile = profile;
    isAddModalOpen = true;
  }

  onMount(async () => {
    await loadProfiles();
    await loadSSHKeys();
    await loadFolderMappings();
    await loadAppStatus();

    window.addEventListener('keydown', handleGlobalKeydown);
    return () => window.removeEventListener('keydown', handleGlobalKeydown);
  });

  function handleGlobalKeydown(e: KeyboardEvent) {
    const isMeta = e.metaKey || e.ctrlKey;
    if (isMeta && e.key.toLowerCase() === 'n') {
      e.preventDefault();
      isAddModalOpen = true;
    } else if (isMeta && e.key === ',') {
      e.preventDefault();
      isConfigModalOpen = true;
    } else if (isMeta && e.key === '1' && $profiles[0]) {
      e.preventDefault();
      switchProfile($profiles[0].id);
    } else if (isMeta && e.key === '2' && $profiles[1]) {
      e.preventDefault();
      switchProfile($profiles[1].id);
    } else if (isMeta && e.key === '3' && $profiles[2]) {
      e.preventDefault();
      switchProfile($profiles[2].id);
    } else if (isMeta && e.key === '4' && $profiles[3]) {
      e.preventDefault();
      switchProfile($profiles[3].id);
    }
  }

  function handleTabChange(e: CustomEvent<string>) {
    currentTab = e.detail;
  }
</script>

<!-- App Window Wrapper -->
<div class="h-screen overflow-hidden border border-[#182330] text-slate-100 flex flex-col justify-between antialiased selection:bg-[#3fb950]/30 selection:text-white bg-[#090e15]">
  
  <!-- Outer Window Shell -->
  <div class="w-full flex-1 flex flex-col min-h-0">
    <!-- Window Titlebar -->
    <TitleBar 
      activeTab={currentTab}
      on:tab-change={handleTabChange}
      on:open-add={openNewProfileModal}
      on:open-config={() => isConfigModalOpen = true}
    />

    <!-- Main Dynamic Canvas -->
    <main class="flex-1 min-h-0 p-4 md:p-8 flex flex-col items-center justify-start relative overflow-y-auto">
      <!-- Subtle Background Grid & Terminal Aesthetics -->
      <div class="absolute inset-0 pointer-events-none opacity-20">
        <svg class="w-full h-full" xmlns="http://www.w3.org/2000/svg">
          <defs>
            <pattern id="grid" width="40" height="40" patternUnits="userSpaceOnUse">
              <path d="M 40 0 L 0 0 0 40" fill="none" stroke="#233547" stroke-width="1"></path>
            </pattern>
          </defs>
          <rect width="100%" height="100%" fill="url(#grid)"></rect>
        </svg>
      </div>

      <!-- Tab Content 1: Dashboard (Rotary Dial UI) -->
      {#if currentTab === 'dashboard'}
        <div class="w-full flex flex-col items-center justify-between gap-6 relative z-10 animate-in fade-in duration-150">
          <DashboardRotary />
          <ProfileCard on:open-ssh-tab={() => currentTab = 'ssh-keys'} />
        </div>
      {/if}

      <!-- Tab Content 2: Profiles Management Matrix -->
      {#if currentTab === 'profiles'}
        <div class="w-full relative z-10 animate-in fade-in duration-150">
          <ProfilesView 
            on:add-identity={openNewProfileModal}
            on:edit-profile={(e) => openEditProfileModal(e.detail)}
          />
        </div>
      {/if}

      <!-- Tab Content 3: Folder Mappings -->
      {#if currentTab === 'folder-mappings'}
        <div class="w-full relative z-10 animate-in fade-in duration-150">
          <FolderMappingsView />
        </div>
      {/if}

      <!-- Tab Content 4: SSH Keys & Cryptographic Identities -->
      {#if currentTab === 'ssh-keys'}
        <div class="w-full relative z-10 animate-in fade-in duration-150">
          <SSHKeysView />
        </div>
      {/if}
    </main>

    <!-- Bottom Telemetry Status Deck Footer -->
    <footer class="h-10 shrink-0 bg-[#060b10] border-t border-[#162230] px-5 flex items-center justify-between font-mono text-xs text-slate-400 select-none">
      <div class="flex items-center gap-6">
        <div class="flex items-center gap-1.5">
          <span class="text-slate-500">config:</span>
          <span class="text-[#3fb950]">~/.gitconfig</span>
          <span class="text-slate-600">→</span>
          <span class="text-slate-200">~/.gitconfig-{$activeProfile?.id || 'work'}</span>
        </div>
        <div class="hidden sm:flex items-center gap-1.5">
          <span class="text-slate-500">ssh-agent:</span>
          <span class="text-[#a2c9ff]">active (PID {$appStatus.pid})</span>
        </div>
      </div>

      <div class="flex items-center gap-4">
        <div class="flex items-center gap-1.5 text-slate-400">
          <span class="w-1.5 h-1.5 rounded-full bg-[#3fb950] animate-pulse"></span>
          <span>latency: <strong class="text-white">{$switchLatency}ms</strong></span>
        </div>

        <button 
          on:click={async () => { await loadProfiles(); await loadAppStatus(); }}
          class="flex items-center gap-1 px-2 py-0.5 bg-[#121c28] hover:bg-[#1a2838] border border-[#1e2d3e] rounded text-slate-300 hover:text-white transition-colors text-[11px]"
          title="Reload daemon state"
        >
          <span>↻ reload daemon</span>
        </button>
      </div>
    </footer>
  </div>
</div>

<!-- Add Profile Identity Modal -->
<AddProfileModal 
  bind:open={isAddModalOpen}
  {editingProfile}
  on:close={() => { isAddModalOpen = false; editingProfile = null; }}
/>

<!-- Config Inspector Modal -->
<ConfigViewerModal 
  bind:open={isConfigModalOpen} 
  on:close={() => isConfigModalOpen = false}
/>
