<script lang="ts">
  import { createEventDispatcher } from 'svelte';

  import { goto } from '$app/navigation';

  import LL from '$src/i18n/i18n-svelte';

  import MagnifyingGlass from '~icons/ph/magnifying-glass-bold';
  import Clear from '~icons/ph/x-bold';

  const dispatch = createEventDispatcher();

  export let value = '';
  export let placeholder = $LL.SEARCH.INPUT_PLACEHOLDER();
  export let delay = 500;
  // makes the <input> element available to the parent component (example usage: conditionally put focus it)
  export let ref: HTMLInputElement;

  let debouncedValue: string | undefined;
  let timer: NodeJS.Timeout;

  const debounce = (value: string) => {
    clearTimeout(timer);
    timer = setTimeout(() => {
      debouncedValue = value;
      dispatch('value', debouncedValue);
    }, delay);
  };

  const clear = () => {
    value = '';
    dispatch('value', '');
  };
</script>

<div class="flex">
  <div class="group relative flex grow items-center">
    <MagnifyingGlass class="absolute left-4 h-4 w-4 text-slate-800 dark:text-grey"></MagnifyingGlass>

    <!-- TODO: apply "appearance-none" and style everything manually? -->
    <input
      type="text"
      class="h-11 w-full rounded-full bg-white pl-12 text-sm font-medium text-slate-500 placeholder-slate-500 placeholder-opacity-50 dark:bg-dark dark:text-slate-300"
      {placeholder}
      id="input"
      bind:value
      bind:this={ref}
      on:input={() => debounce(value)}
    />
    {#if value}
      <button
        class="absolute right-2 inline-flex h-4 w-4 items-center justify-center rounded-full p-4"
        on:click={clear}
      >
        <Clear class="absolute h-4 w-4 text-slate-800 dark:text-grey" />
      </button>
    {/if}
  </div>

  <button class="pl-4 text-sm font-medium text-slate-500 dark:text-slate-300" on:click={() => goto('/me')}
    >{$LL.CANCEL()}</button
  >
</div>
