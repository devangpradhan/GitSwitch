<script lang="ts">
  export let variant: 'active' | 'standby' | 'bot' | 'cyan' | 'neutral' = 'neutral';
  export let size: 'sm' | 'md' = 'sm';
  export let pulse: boolean = false;
  export let className: string = '';

  const variants = {
    active: 'bg-[#3fb950]/15 text-[#3fb950] border-[#3fb950]/30',
    standby: 'bg-[#182028] text-slate-400 border-[#2d363e]',
    bot: 'bg-[#bc8cff]/15 text-[#bc8cff] border-[#bc8cff]/30',
    cyan: 'bg-[#38bdf8]/15 text-[#38bdf8] border-[#38bdf8]/30',
    neutral: 'bg-[#16202c] text-slate-300 border-[#233547]',
  };

  const sizes = {
    sm: 'text-[10px] px-2 py-0.5',
    md: 'text-xs px-2.5 py-1',
  };
</script>

<span class="inline-flex items-center gap-1.5 font-mono font-medium rounded border tracking-wide select-none {variants[variant]} {sizes[size]} {className}">
  {#if pulse}
    <span class="w-1.5 h-1.5 rounded-full bg-current animate-pulse"></span>
  {/if}
  <slot />
</span>
