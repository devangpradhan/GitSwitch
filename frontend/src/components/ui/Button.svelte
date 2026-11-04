<script lang="ts">
  import { createEventDispatcher } from 'svelte';

  export let variant: 'primary' | 'secondary' | 'ghost' | 'destructive' | 'outline' = 'primary';
  export let size: 'sm' | 'md' | 'lg' = 'md';
  export let disabled: boolean = false;
  export let loading: boolean = false;
  export let type: 'button' | 'submit' | 'reset' = 'button';
  export let className: string = '';

  const dispatch = createEventDispatcher();

  const baseStyles = 'inline-flex items-center justify-center font-mono font-medium rounded transition-all duration-150 focus:outline-none focus:ring-1 focus:ring-primary/50 select-none active:scale-[0.98] disabled:opacity-50 disabled:pointer-events-none disabled:active:scale-100';

  const variants = {
    primary: 'bg-[#238636] hover:bg-[#2ea043] text-white border border-white/10 shadow-[0_0_12px_rgba(46,160,67,0.35)]',
    secondary: 'bg-[#16202c] hover:bg-[#202d3e] text-slate-200 border border-[#233547] hover:text-white',
    ghost: 'bg-transparent hover:bg-[#162230] text-slate-400 hover:text-slate-200',
    destructive: 'bg-[#21262d] hover:bg-[#da3633]/20 text-[#f85149] border border-[#f85149]/30 hover:border-[#f85149]',
    outline: 'bg-transparent border border-[#30363d] hover:border-slate-400 text-slate-300 hover:text-white',
  };

  const sizes = {
    sm: 'text-[11px] px-2.5 py-1 gap-1.5',
    md: 'text-xs px-3.5 py-1.5 gap-2',
    lg: 'text-sm px-4 py-2 gap-2.5',
  };
</script>

<button
  {type}
  {disabled}
  class="{baseStyles} {variants[variant]} {sizes[size]} {className}"
  on:click={(e) => dispatch('click', e)}
>
  {#if loading}
    <svg class="animate-spin -ml-1 mr-1.5 h-3.5 w-3.5 text-current" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
      <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
      <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
    </svg>
  {/if}
  <slot />
</button>
