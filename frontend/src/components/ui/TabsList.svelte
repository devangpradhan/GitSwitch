<script lang="ts">
  export let className: string = '';
</script>

<div class="flex items-center bg-[#070b10] p-1 rounded-xl border border-[#1b2636] {className}" role="tablist">
  <slot />
</div>
