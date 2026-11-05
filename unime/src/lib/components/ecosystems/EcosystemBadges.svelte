<script lang="ts">
  import LL from '$i18n/i18n-svelte';

  import type { EcosystemProfile } from '@bindings/user_prompt/EcosystemProfile';

  import EcosystemAvatar from './EcosystemAvatar.svelte';

  export let ecosystems: EcosystemProfile[];

  // A list entry has room for two marks before the title gets squeezed.
  const MAX_SHOWN = 2;

  $: shown = ecosystems.slice(0, MAX_SHOWN);
  $: hidden = ecosystems.length - shown.length;
  $: label = $LL.CREDENTIAL.ECOSYSTEMS.ENDORSED_BY({ names: ecosystems.map((ecosystem) => ecosystem.name).join(', ') });
</script>

<!--
@component
The marks of the ecosystems that endorse a credential's issuer, overlapping, for a credential list entry. Renders
nothing without ecosystems.

### Props
- ecosystems
-->
{#if ecosystems.length > 0}
  <div class="mr-2 flex shrink-0 items-center -space-x-1.5" role="img" aria-label={label} title={label}>
    {#each shown as ecosystem, index (index)}
      <EcosystemAvatar
        name={ecosystem.name}
        logoUri={ecosystem.logo_uri}
        isTempAsset="fallback"
        class="size-6 rounded-full ring-2 ring-white dark:ring-dark"
        textClass="text-[8px]/[10px]"
      />
    {/each}
    {#if hidden > 0}
      <div
        class="flex size-6 shrink-0 items-center justify-center rounded-full bg-grey text-[10px]/[12px] font-semibold text-slate-600 ring-2 ring-white dark:bg-background dark:text-text-alt dark:ring-dark"
      >
        +{hidden}
      </div>
    {/if}
  </div>
{/if}
