<script lang="ts">
  import { onMount } from 'svelte';
  import { Dialog, Button, Tabs, TabsList, TabsTrigger, TabsContent } from '../ui';
  import { GetConfigFiles } from '../../wailsjs/go/main/App';
  import type { ConfigFiles } from '../../types';

  export let open: boolean = false;

  let configFiles: ConfigFiles = {
    gitconfig_path: '~/.gitconfig',
    gitconfig_content: '',
    sshconfig_path: '~/.ssh/config',
    sshconfig_content: ''
  };

  let activeTab = 'gitconfig';
  let copied = false;

  onMount(async () => {
    await refresh();
  });

  $: if (open) {
    refresh();
  }

  async function refresh() {
    try {
      configFiles = await GetConfigFiles();
    } catch (e) {
      console.error(e);
    }
  }

  async function copyCurrent() {
    const text = activeTab === 'gitconfig' ? configFiles.gitconfig_content : configFiles.sshconfig_content;
    await navigator.clipboard.writeText(text);
    copied = true;
    setTimeout(() => { copied = false; }, 2000);
  }
</script>

<Dialog 
  bind:open 
  title="Host Configuration Inspector" 
  description="Real-time inspect of generated ~/.gitconfig and ~/.ssh/config system files."
  maxWidth="max-w-[760px]"
>
  <svelte:fragment slot="badge">
    <span class="text-[10px] font-mono px-2 py-0.5 rounded bg-[#38bdf8]/15 text-[#38bdf8] border border-[#38bdf8]/30">
      Zero Latency Sync
    </span>
  </svelte:fragment>

  <div class="space-y-3 font-mono text-xs">
    <Tabs bind:value={activeTab}>
      <div class="flex items-center justify-between pb-2">
        <TabsList>
          <TabsTrigger value="gitconfig">~/.gitconfig</TabsTrigger>
          <TabsTrigger value="sshconfig">~/.ssh/config</TabsTrigger>
        </TabsList>

        <Button variant="secondary" size="sm" on:click={copyCurrent}>
          {#if copied}
            <span class="text-[#3fb950]">✓ Copied!</span>
          {:else}
            <span>📋 Copy File</span>
          {/if}
        </Button>
      </div>

      <TabsContent value="gitconfig">
        <div class="rounded-lg bg-[#070b10] border border-[#1b2734] p-4 text-[11px] font-mono text-slate-300 max-h-[380px] overflow-y-auto leading-relaxed select-text">
          <div class="text-slate-500 pb-2 border-b border-[#141e2a] mb-2 flex justify-between">
            <span>FILE: {configFiles.gitconfig_path}</span>
            <span class="text-[#3fb950]">PARSED & SYNCED</span>
          </div>
          <pre class="whitespace-pre-wrap">{configFiles.gitconfig_content || '# Empty or loading...'}</pre>
        </div>
      </TabsContent>

      <TabsContent value="sshconfig">
        <div class="rounded-lg bg-[#070b10] border border-[#1b2734] p-4 text-[11px] font-mono text-slate-300 max-h-[380px] overflow-y-auto leading-relaxed select-text">
          <div class="text-slate-500 pb-2 border-b border-[#141e2a] mb-2 flex justify-between">
            <span>FILE: {configFiles.sshconfig_path}</span>
            <span class="text-[#a2c9ff]">MANAGED HOSTS</span>
          </div>
          <pre class="whitespace-pre-wrap">{configFiles.sshconfig_content || '# Empty or loading...'}</pre>
        </div>
      </TabsContent>
    </Tabs>
  </div>
</Dialog>
