<script lang="ts">
  import { createEventDispatcher, onMount } from 'svelte';

  export let open: boolean = false;
  export let title: string = '';
  export let description: string = '';
  export let maxWidth: string = 'max-w-[680px]';

  const dispatch = createEventDispatcher();

  function close() {
    open = false;
    dispatch('close');
  }

  function handleKeydown(e: KeyboardEvent) {
    if (e.key === 'Escape' && open) {
      close();
    }
  }

  onMount(() => {
    window.addEventListener('keydown', handleKeydown);
    return () => window.removeEventListener('keydown', handleKeydown);
  });
</script>

{#if open}
  <!-- Bits UI Dialog Overlay & Backdrop -->
  <div 
    class="fixed inset-0 z-50 flex items-center justify-center p-3 md:p-6 bg-[#04080dc8] backdrop-blur-md transition-opacity duration-200"
    on:click|self={close}
    on:keydown={(e) => { if (e.key === 'Escape') close(); }}
    tabindex="-1"
    role="dialog"
    aria-modal="true"
    aria-labelledby="dialog-title"
  >
    <!-- Bits UI Dialog Content Window -->
    <div 
      class="w-full {maxWidth} bg-[#0d151e] border border-[#1b2734] rounded-xl shadow-[0_25px_60px_-15px_rgba(0,0,0,0.85),0_0_35px_rgba(63,185,80,0.08)] overflow-hidden flex flex-col relative animate-in fade-in zoom-in-95 duration-150"
    >
      <!-- Top Decorative Glow Stripe -->
      <div class="h-[2px] w-full bg-gradient-to-r from-[#1b2734] via-[#3fb950] to-[#1b2734]"></div>

      <!-- Header -->
      <div class="px-6 py-4 bg-[#0a1119] border-b border-[#1b2734] flex items-center justify-between">
        <div class="flex items-center gap-3">
          <div class="w-8 h-8 rounded-lg bg-[#3fb950]/10 border border-[#3fb950]/30 flex items-center justify-center text-[#3fb950] font-bold text-sm shadow-inner">
            ⚡
          </div>
          <div>
            <h2 id="dialog-title" class="text-sm md:text-base font-semibold tracking-tight text-white flex items-center gap-2">
              {title}
              <slot name="badge" />
            </h2>
            {#if description}
              <p class="text-xs text-slate-400 font-mono mt-0.5">
                {description}
              </p>
            {/if}
          </div>
        </div>

        <button 
          on:click={close}
          class="w-7 h-7 rounded-md border border-[#1b2734] text-slate-400 hover:text-white hover:bg-[#15202c] hover:border-slate-500 flex items-center justify-center transition-colors text-xs font-mono"
          title="Close (Esc)"
        >
          ✕
        </button>
      </div>

      <!-- Modal Body -->
      <div class="p-6 overflow-y-auto max-h-[80vh]">
        <slot />
      </div>

      <!-- Footer slot if any -->
      {#if $$slots.footer}
        <div class="px-6 py-3 bg-[#0a1119] border-t border-[#1b2734] flex items-center justify-end gap-3">
          <slot name="footer" />
        </div>
      {/if}
    </div>
  </div>
{/if}
