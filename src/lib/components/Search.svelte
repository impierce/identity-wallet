<script lang="ts">
  import { createEventDispatcher } from 'svelte';

  import { goto } from '$app/navigation';

  import MagnifyingGlass from '~icons/ph/magnifying-glass-bold';
  import Clear from '~icons/ph/x-bold';

  const dispatch = createEventDispatcher();

  export let delay = 500;
  export let placeholder = 'Look for something';

  let inputValue: string | undefined;
  let debouncedValue = '';
  let timer: any;

  const debounce = (value: string) => {
    clearTimeout(timer);
    timer = setTimeout(() => {
      debouncedValue = value;
      dispatch('value', debouncedValue);
    }, delay);
  };

  const clear = () => {
    inputValue = undefined;
    dispatch('value', '');
  };
</script>

<div class="flex">
  <div class="group relative flex grow items-center">
    <MagnifyingGlass class="absolute left-4 h-4 w-4 text-slate-800"></MagnifyingGlass>

    <!-- TODO: apply "appearance-none" and style everything manually? -->
    <input
      type="text"
      class="h-11 w-full rounded-full bg-white pl-12 text-sm leading-6 text-slate-500 placeholder-opacity-50"
      {placeholder}
      id="input"
      bind:value={inputValue}
      on:input={(e) => debounce(inputValue)}
    />
    {#if inputValue}
      <button
        class="absolute right-2 inline-flex h-4 w-4 items-center justify-center rounded-full p-4 active:bg-silver"
        on:click={clear}
      >
        <Clear class="absolute text-slate-800" />
      </button>
    {/if}
  </div>

  <button class="pl-4 text-sm text-slate-500 dark:text-slate-300" on:click={() => goto('/me')}>Cancel</button>
</div>
