<script lang="ts">
  import { createEventDispatcher } from 'svelte';

  export let value: string = '';
  export let options: { value: string; label: string; description?: string }[] = [];
  export let placeholder: string = 'Select option';
  export let className: string = '';
  export let id: string = '';

  const dispatch = createEventDispatcher();

  function handleChange(e: Event) {
    const target = e.target as HTMLSelectElement;
    value = target.value;
    dispatch('change', value);
  }
</script>

<div class="relative w-full {className}">
  <select
    {id}
    {value}
    on:change={handleChange}
    class="w-full appearance-none px-3.5 py-2.5 rounded-lg bg-[#070d13] border border-[#1b2734] text-slate-100 font-mono text-xs focus:outline-none focus:border-[#3fb950] focus:ring-1 focus:ring-[#3fb950]/40 transition-all cursor-pointer shadow-inner pr-10"
  >
    {#if placeholder}
      <option value="" disabled selected={!value}>{placeholder}</option>
    {/if}
    {#each options as opt}
      <option value={opt.value} class="bg-[#0b141e] text-slate-200 py-1">
        {opt.label} {opt.description ? `(${opt.description})` : ''}
      </option>
    {/each}
  </select>
  <div class="pointer-events-none absolute right-3.5 top-1/2 -translate-y-1/2 text-slate-500 font-mono text-xs">
    ▼
  </div>
</div>
