<script lang="ts">
  import { onMount } from 'svelte';

  import UniMeLogo from '$src/lib/components/logo/UniMeLogo.svelte';
  import { dispatch } from '$src/lib/dispatcher';

  onMount(() => {
    console.log('Injecting password automatically ...');
    setTimeout(() => {
      dispatch({ type: '[Storage] Unlock', payload: { password: 'sup3rSecr3t' } });
    }, 0);
  });
</script>

<div class="content-height flex items-center justify-center bg-secondary">
  <!-- Placeholder -->
  <!-- <div class="aspect-square w-1/4 rounded-3xl border border-slate-200 bg-slate-100" /> -->
  <div class="flex flex-col items-center justify-center">
    <UniMeLogo />
    <p class="pt-6 font-medium text-white">Injecting password ...</p>
  </div>
</div>

<style>
  .content-height {
    height: calc(100vh - var(--safe-area-inset-top) - var(--safe-area-inset-bottom));
  }
</style>
