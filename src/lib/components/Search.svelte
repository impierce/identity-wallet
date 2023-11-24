<script lang="ts">
  import { state } from '$src/stores';
  import { createEventDispatcher } from 'svelte';
  import MagnifyingGlass from '~icons/ph/magnifying-glass-bold';
  import Clear from '~icons/ph/x-bold';

  const dispatch = createEventDispatcher();
  export let delay = 500;
  let inputValue: string | undefined;
  let debouncedValue = '';
  let timer: any;

  export let placeholder = 'Type something here ...';

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

<!-- The HTML and TailwindCSS of the search component-->
<form class="wrapper_search">
  <div class="relative my-4 flex">
    <!--This is the magnifying icon-->
    <div>
      <MagnifyingGlass class="absolute m-3 fill-slate-800"></MagnifyingGlass>
    </div>
    <!--This is the input field-->
    <div class="w-full">
      <input
        type="text"
        class="h-11 w-full rounded-full bg-white pl-10 text-sm text-slate-600"
        {placeholder}
        id="input"
        bind:value={inputValue}
        on:input={(e) => debounce(inputValue)}
      />
    </div>
    {#if inputValue}
      <div on:click={clear}>
        <!-- <Icons name="x" aria-hidden="true"></Icons> -->
        <Clear
          class="pointer-events-auto absolute right-3 top-1/2 -mt-2.5 h-5 w-5 fill-current text-slate-800 hover:cursor-pointer"
        />
      </div>
    {/if}
  </div>
</form>
