<script lang="ts">
  export let active: boolean = false;
  export let accentColor: string = '#3fb950';
  export let className: string = '';
</script>

<div class="relative bg-[#0d141e] border border-[#1b2636] rounded-xl overflow-hidden transition-all duration-200 {active ? 'shadow-[0_4px_24px_rgba(0,0,0,0.5)] border-[#2a3c50]' : 'hover:border-[#243548]'} {className}">
  {#if active}
    <div 
      class="absolute left-0 top-0 bottom-0 w-1 shadow-[0_0_12px_rgba(63,185,80,0.65)]"
      style="background-color: {accentColor};"
    ></div>
  {/if}
  <slot />
</div>
