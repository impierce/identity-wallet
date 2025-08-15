<script lang="ts">
  import { createEventDispatcher, onMount } from 'svelte';

  import type { DisplayCredential } from '@bindings/credentials/DisplayCredential';

  import { Image } from '$lib/components';
  import { dispatch } from '$lib/dispatcher';
  import { CertificateLightIcon, HeartStraightFillIcon, HeartStraightRegularIcon, UserLightIcon } from '$lib/icons';
  import { getImageAsset } from '$lib/utils';

  import CredentialHeaderMenu from './CredentialHeaderMenu.svelte';

  const dispatchEvent = createEventDispatcher();

  export let credential: DisplayCredential;

  // Url to cached credential logo (if available).
  let credentialLogoUrl: string | null = null;

  onMount(async () => {
    credentialLogoUrl = await getImageAsset(credential.id);
  });
</script>

<!-- Stretch over parent horizontal padding with negative margins. -->
<div class="relative -mx-4 flex flex-col items-center gap-4 bg-background py-5">
  <!-- Background is always white since most logos are designed for light backgrounds -->
  {#if credentialLogoUrl}
    <div class="grid size-40 place-items-center rounded-xl bg-white">
      <!-- Fit image of unknown dimensions into available space with `contain` (not `cover`). -->
      <img src={credentialLogoUrl} alt="Credential logo" class="size-32 object-contain" />
    </div>
  {:else}
    <!-- When there's no logo, we adjust the background to the theme -->
    <div class="grid size-40 place-items-center rounded-xl bg-background-alt">
      {#if credential.metadata.icon}
        <Image
          id={`${credential.metadata.icon}Light`}
          iconFallback={`${credential.metadata.icon}Light`}
          iconClass="size-10 dark:text-text-alt"
        />
      {:else if credential.data.type.includes('OpenBadgeCredential')}
        <CertificateLightIcon class="size-10 text-text-alt" />
      {:else}
        <UserLightIcon class="size-10 text-text-alt" />
      {/if}
    </div>
  {/if}

  <slot />

  <button
    class="absolute left-0 top-0 ml-2 mt-4 p-1.5"
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

  <div class="absolute right-0 top-0 mr-2 mt-4">
    <CredentialHeaderMenu id={credential.id} on:edit={() => dispatchEvent('edit')} />
  </div>
</div>
