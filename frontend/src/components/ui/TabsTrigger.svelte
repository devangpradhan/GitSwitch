<script lang="ts">
  import { getContext } from 'svelte';
  import type { Writable } from 'svelte/store';

  export let value: string;
  export let className: string = '';

  const { activeTab, selectTab } = getContext<{
    activeTab: Writable<string>;
    selectTab: (val: string) => void;
  }>('bits-tabs');

  $: isActive = $activeTab === value;
</script>

<button
  type="button"
  role="tab"
  aria-selected={isActive}
  tabindex={isActive ? 0 : -1}
  class="flex items-center gap-2 px-3.5 py-1.5 rounded-lg text-xs font-medium transition-all select-none
    {isActive 
      ? 'bg-[#14202d] text-[#3fb950] shadow-sm border border-[#223348]' 
      : 'text-slate-400 hover:text-slate-200 hover:bg-[#0f1722] border border-transparent'} {className}"
  on:click={() => selectTab(value)}
>
  <slot />
  {#if isActive}
    <span class="w-1.5 h-1.5 rounded-full bg-[#3fb950] animate-pulse"></span>
  {/if}
</button>
