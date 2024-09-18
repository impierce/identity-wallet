<script lang="ts">
  import { onMount } from 'svelte';

  import type { DisplayCredential } from '@bindings/credentials/DisplayCredential';

  import { dispatch } from '$lib/dispatcher';
  import { CertificateLightIcon, HeartStraightFillIcon, HeartStraightRegularIcon } from '$lib/icons';
  import { getImageAsset } from '$lib/utils';

  export let credential: DisplayCredential;

  // Url to cached credential logo (if available).
  let credentialLogoUrl: string | null = null;

  onMount(async () => {
    credentialLogoUrl = await getImageAsset(credential.id);
  });
</script>

<!-- Stretch over parent horizontal padding with negative margins. -->
<div class="relative -mx-4 flex flex-col items-center gap-4 bg-background py-5">
  <div class="grid h-40 w-40 place-items-center rounded-xl bg-white">
    {#if credentialLogoUrl}
      <!-- Fit image of unknown dimensions into availalbe space with `contain` (not `cover`). -->
      <img src={credentialLogoUrl} alt="Credential logo" class="h-32 w-32 object-contain" />
    {:else}
      <CertificateLightIcon class="h-10 w-10 text-text-alt" />
    {/if}
  </div>

  <slot />

  <button
    class="absolute left-0 top-0 ml-4 mt-4"
    on:click={() =>
      dispatch({
        type: '[Credential Metadata] Update',
        payload: {
          id: credential.id,
          is_favorite: !credential.metadata.is_favorite,
        },
      })}
  >
    {#if credential.metadata.is_favorite}
      <HeartStraightFillIcon class="h-6 w-6 dark:text-white" />
    {:else}
      <HeartStraightRegularIcon class="h-6 w-6 dark:text-white" />
    {/if}
  </button>
</div>
