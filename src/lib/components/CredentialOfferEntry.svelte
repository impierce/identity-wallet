<script lang="ts">
  import { createCheckbox, melt } from '@melt-ui/svelte';

  import Image from '$src/lib/components/Image.svelte';

  import Check from '~icons/ph/check-bold';

  const {
    elements: { root, input },
    helpers: { isChecked },
  } = createCheckbox({
    defaultChecked: true,
    disabled: true,
  });

  export let index: number | string;
  export let title = '';
  export let description = '';
</script>

<div class="flex h-[60px] items-center justify-start rounded-xl bg-white p-[5px] dark:bg-dark">
  <!-- Logo -->
  <div
    class="mr-[15px] flex h-[50px] w-[50px] min-w-[50px] flex-col items-center justify-center overflow-hidden rounded-lg"
  >
    <span class="flex h-full w-full items-center justify-center bg-silver dark:bg-navy">
      <Image id={`credential_${index}`} isTempAsset={true} />
    </span>
  </div>
  <!-- Text -->
  <div class="flex grow flex-col items-start">
    <p class="line-clamp-2 text-[13px]/[18px] font-medium text-slate-800 dark:text-grey">{title}</p>
    <p class="max-w-[180px] truncate text-[12px]/[20px] font-medium text-slate-400 dark:text-slate-300">
      {description}
    </p>
  </div>

  <!-- Checkbox -->
  <button
    use:melt={$root}
    class="mr-2 flex h-6 w-6 appearance-none items-center justify-center rounded-md border-[1.5px] border-slate-300 p-[6px] text-white data-[disabled]:opacity-50 dark:border-slate-600
    {isChecked ? 'border-none bg-primary' : 'bg-white dark:bg-dark'}"
    id="checkbox-{index}"
  >
    {#if $isChecked}
      <Check class="h-3 w-3" />
    {/if}
    <input use:melt={$input} />
  </button>
</div>
