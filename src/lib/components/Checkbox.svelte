<script lang="ts">
  import { melt } from '@melt-ui/svelte';

  import Check from '~icons/ph/check-bold';

  export let root: any;
  export let input: any;
  export let isChecked: boolean;
</script>

<button
  use:melt={root}
  class="flex h-6 w-6 appearance-none items-center justify-center
            rounded-md border-[1.5px] border-[#C5C6CC] p-[6px] text-white
            {isChecked ? 'border-none bg-primary' : 'bg-white'}"
  id="checkbox"
>
  {#if isChecked}
    <Check class="h-3 w-3" />
  {/if}
  {#if input}
    <input use:melt={input} />
  {/if}
</button>

<!-- <button
        use:melt={$item(option)}
        class="grid h-6 w-6 cursor-default place-items-center rounded-full bg-white shadow-sm
      hover:bg-magnum-100"
        id={option}
        aria-labelledby="{option}-label"
      >
        {#if $isChecked(option)}
          <div class="h-3 w-3 rounded-full bg-magnum-500" />
        {/if}
      </button> -->
