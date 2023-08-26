<script lang="ts">
  import { onMount } from 'svelte';

  import { dispatch } from '$src/lib/dispatcher';

  onMount(() => {
    console.log('Injecting password automatically ...');
    dispatch({ type: '[Storage] Unlock', payload: { password: 'sup3rSecr3t' } });
  });
</script>

<div>prompt/password-required</div>
