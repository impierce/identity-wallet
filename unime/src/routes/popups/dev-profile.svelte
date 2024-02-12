<script lang="ts">
  import type { ProfileType } from '@bindings/actions/DevProfileType';

  import { dispatch } from '$src/lib/dispatcher';

  async function loadProfile(profile: ProfileType) {
    await dispatch({ type: '[DEV] Load DEV profile', payload: { profile } });
    onClose();
  }

  export let onClose;
</script>

<div class="flex flex-col">
  <button class="mb-2" on:click={() => loadProfile('Turtle')}>Select Turtle</button>
  <hr class="s-divider svelte-syn7p7 inset" aria-orientation="horizontal" />
  <button class="mt-2" on:click={() => loadProfile('Ferris')}>Select Ferris</button>
</div>
