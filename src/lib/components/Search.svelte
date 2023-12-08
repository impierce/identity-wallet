<script lang="ts">
  import { state } from '$src/stores';
  import { createEventDispatcher } from 'svelte';
  import MagnifyingGlass from '~icons/ph/magnifying-glass-bold';
  import Clear from '~icons/ph/x-bold';
  import { goto } from '$app/navigation';

  const dispatch = createEventDispatcher();
  export let delay = 500;
  let inputValue: string | undefined;
  let debouncedValue = '';
  let timer: any;

  export let placeholder = 'Look for something ...';

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

<!-- Search compontent for the credentials-->
<div class="flex flex-row p-4">
  <!--input field-->
  <form class="relative flex w-full">
    <MagnifyingGlass class="absolute m-3 fill-slate-800"></MagnifyingGlass>

    <div class="">
      <input
        type="text"
        class="h-11 rounded-3xl bg-white px-10 text-sm text-slate-500"
        {placeholder}
        id="input"
        bind:value={inputValue}
        on:input={(e) => debounce(inputValue)}
      />
    </div>
    {#if inputValue}
      <!-- <Icons name="x" aria-hidden="true"></Icons> -->
      <button class="absolute right-3 m-3" on:click={clear}>
        <Clear class="text-slate-800" />
      </button>
    {/if}
  </form>
  <button class="pl-2 text-sm text-slate-500 dark:text-grey" on:click={() => goto('/me')}>Cancel</button>
</div>
