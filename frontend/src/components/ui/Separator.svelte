<script lang="ts">
  export let orientation: 'horizontal' | 'vertical' = 'horizontal';
  export let className: string = '';
</script>

{#if orientation === 'horizontal'}
  <div class="w-full h-[1px] bg-[#182330] my-2 {className}" role="separator"></div>
{:else}
  <div class="h-full w-[1px] bg-[#182330] mx-2 {className}" role="separator"></div>
{/if}
