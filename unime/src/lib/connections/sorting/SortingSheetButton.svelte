<script lang="ts">
  import { createEventDispatcher, SvelteComponent } from 'svelte';

  import LL from '$i18n/i18n-svelte';
  import type { SvelteHTMLElements } from 'svelte/elements';

  import type { CredentialSortMethod } from '@bindings/CredentialSortMethod';

  export let method: CredentialSortMethod;
  export let icon: typeof SvelteComponent<SvelteHTMLElements['svg']>;
  export let label: string;
  export let active = false;
  export let reversed: boolean;

  // Compute sort order based on label and reversal
  let sortOrder: string;
  $: {
    if (method === 'name_az') {
      sortOrder = reversed ? $LL.SORT.ORDER.Z_A() : $LL.SORT.ORDER.A_Z();
    } else if (method === 'issue_date_new_old') {
      sortOrder = reversed ? $LL.SORT.ORDER.NEWEST() : $LL.SORT.ORDER.OLDEST();
    } else if (method === 'added_date_new_old') {
      sortOrder = reversed ? $LL.SORT.ORDER.NEWEST() : $LL.SORT.ORDER.OLDEST();
    }
  }

  // Dispatch component event based on DOM event.
  const dispatch = createEventDispatcher();
  const handleClick = () => {
    dispatch('click');
  };
</script>

<button
  on:click={handleClick}
  class={`my-1 flex w-full gap-2 border p-[10px] text-slate-800 dark:text-grey ${
    active ? 'rounded-lg border-grey bg-silver dark:border-slate-600 dark:bg-navy' : 'border-transparent'
  }`}
>
  <svelte:component this={icon} />
  <p class="text-sm font-medium">{label}</p>
  {#if active}
    <div class="absolute right-2">
      {#if sortOrder !== ''}
        <p class="text-sm font-medium text-primary">{sortOrder}</p>
      {/if}
    </div>
  {/if}
</button>
