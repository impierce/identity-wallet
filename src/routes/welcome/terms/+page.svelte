<script lang="ts">
  import { goto } from '$app/navigation';
  import { fade, fly } from 'svelte/transition';

  import { TopNavigation } from '@impierce/ui-components';
  import { createCheckbox, melt } from '@melt-ui/svelte';

  import Button from '$src/lib/components/Button.svelte';
  import Checkbox from '$src/lib/components/Checkbox.svelte';
  import { dispatch } from '$src/lib/dispatcher';

  import Check from '~icons/ph/check-bold';

  const {
    elements: { root, input },
    helpers: { isChecked }
  } = createCheckbox({});

  const {
    elements: { root: ownership_root, input: ownership_input },
    helpers: { isChecked: ownership_isChecked }
  } = createCheckbox({});
</script>

<!-- <div in:fly={{ x: 128, delay: 400 }} out:fly={{ x: -128, opacity: 1 }}>
  <div class="content-height flex flex-col bg-neutral-100">
    <TopNavigation title="Terms & Conditions" on:back={() => history.back()} />
    <div class="grow">terms</div>
    <div class="space-y-[10px] rounded-t-3xl bg-white p-6">
      <Button label="Continue" on:click={() => goto('/welcome/terms')} disabled />
    </div>
  </div>
</div> -->
<!-- <div in:fade={{ delay: 300 }}> -->
<TopNavigation title="Terms & Conditions" on:back={() => history.back()} />
<!-- </div> -->
<!-- <div class="mt-12 grow p-4" in:fly={{ x: 300, delay: 300 }}> -->
<div class="mt-8 grow p-4" in:fade={{ delay: 200 }} out:fade={{ duration: 200 }}>
  <div class="px-2 pb-8 pt-4">
    <p class="pb-4 text-3xl font-semibold text-slate-800">
      So here's the <br /><span class="text-primary">less interesting stuff ...</span>
    </p>
    <p class="text-[15px]/[24px] font-medium text-slate-500">
      Yeah, we know. We still recommend you read this information carefully.
    </p>
  </div>
  <div class="mt-4 flex flex-col space-y-4">
    <div
      class="flex items-center justify-between space-x-4 rounded-xl border border-slate-200 bg-white p-4"
    >
      <div class="grow">
        <p class="text-[13px]/[24px] font-medium text-slate-800">Data</p>
        <p class="text-[12px]/[20px] font-medium text-slate-500">
          I have read and accept the <span class="text-primary">terms and conditions</span>
        </p>
      </div>
      <div>
        <button
          use:melt={$root}
          class="flex h-6 w-6 appearance-none items-center justify-center
            rounded-md border-[1.5px] border-[#C5C6CC] p-[6px] text-white
            {$isChecked ? 'border-none bg-primary' : 'bg-white'}"
          id="checkbox"
        >
          {#if $isChecked}
            <Check class="h-3 w-3" />
          {/if}
          <input use:melt={$input} />
        </button>
      </div>
    </div>
    <div class="flex items-center justify-between rounded-xl border border-slate-200 bg-white p-4">
      <div class="grow">
        <p class="text-[13px]/[24px] font-medium text-slate-800">Ownership</p>
        <p class="text-[12px]/[20px] font-medium text-slate-500">
          I understand that I am solely responsible for <span class="text-primary">my backups</span>
        </p>
      </div>
      <div>
        <Checkbox
          root={$ownership_root}
          input={$ownership_input}
          isChecked={$ownership_isChecked}
        />
      </div>
    </div>
  </div>
</div>
<!-- Actions -->
<!-- <div class="space-y-[10px] rounded-t-3xl bg-white p-6" in:fly={{ y: 154, delay: 300, opacity: 1 }}> -->
<div class="rounded-t-3xl bg-white p-6" in:fade={{ delay: 200 }} out:fade={{ duration: 200 }}>
  <Button label="Continue" on:click={() => goto('/welcome/customize/name')} />
</div>

<style>
  .content-height {
    height: calc(100vh - var(--safe-area-inset-top));
  }
</style>
