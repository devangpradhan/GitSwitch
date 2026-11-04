<script lang="ts">
  import { setContext, createEventDispatcher } from 'svelte';
  import { writable } from 'svelte/store';

  export let value: string = 'dashboard';
  export let className: string = '';

  const dispatch = createEventDispatcher();
  const activeTab = writable<string>(value);

  $: activeTab.set(value);

  setContext('bits-tabs', {
    activeTab,
    selectTab: (val: string) => {
      value = val;
      activeTab.set(val);
      dispatch('change', val);
    }
  });
</script>

<div class="bits-tabs w-full {className}">
  <slot />
</div>
