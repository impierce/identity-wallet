<script lang="ts">
  // TODO: type properly
  export let icon: any;
</script>

<div class="flex h-[75px] w-[75px] items-center justify-center rounded-3xl bg-primary">
  <svelte:component this={icon} class="h-7 w-7 text-white" />
</div>
