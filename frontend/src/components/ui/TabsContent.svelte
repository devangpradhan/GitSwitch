<script lang="ts">
  import { getContext } from 'svelte';
  import type { Writable } from 'svelte/store';

  export let value: string;
  export let className: string = '';

  const { activeTab } = getContext<{
    activeTab: Writable<string>;
  }>('bits-tabs');

  $: isCurrent = $activeTab === value;
</script>

{#if isCurrent}
  <div role="tabpanel" tabindex="0" class="w-full animate-in fade-in-50 duration-150 {className}">
    <slot />
  </div>
{/if}
