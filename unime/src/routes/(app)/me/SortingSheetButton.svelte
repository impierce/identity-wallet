<script lang="ts">
  import { createEventDispatcher, type Component } from 'svelte';

  import LL from '$i18n/i18n-svelte';

  import type { CredentialSortMethod } from '@bindings/profile_settings/CredentialSortMethod';

  export let method: CredentialSortMethod;
  // Cannot be typed as Component<SVGAttributes<SVGSVGElement>> because icon `AlphabeticalOrder` has a different type.
  export let icon: Component;
  export let label: string;
  export let active = false;
  export let reversed: boolean;

  const dispatch = createEventDispatcher();

  // Compute sort order based on label and reversal
  let sortOrder: string;
  $: {
    if (method === 'name_az') {
      sortOrder = reversed ? $LL.SORT.ORDER.Z_A() : $LL.SORT.ORDER.A_Z();
    } else if (method === 'issue_date_new_old') {
      sortOrder = reversed ? $LL.SORT.ORDER.OLDEST() : $LL.SORT.ORDER.NEWEST();
    } else if (method === 'added_date_new_old') {
      sortOrder = reversed ? $LL.SORT.ORDER.OLDEST() : $LL.SORT.ORDER.NEWEST();
    }
  }
</script>

<button
  on:click={() => dispatch('click')}
  class={`dark:text-grey my-1 flex w-full gap-2 border p-[10px] text-slate-800 ${
    active ? 'border-grey bg-silver dark:bg-navy rounded-lg dark:border-slate-600' : 'border-transparent'
  }`}
>
  <svelte:component this={icon} />
  <p class="text-sm font-medium">{label}</p>
  {#if active}
    <div class="absolute right-2">
      {#if sortOrder !== ''}
        <p class="text-primary text-sm font-medium">{sortOrder}</p>
      {/if}
    </div>
  {/if}
</button>
