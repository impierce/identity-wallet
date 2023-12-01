<script lang="ts">
  import { goto } from '$app/navigation';
  import { fade, fly } from 'svelte/transition';

  import Checkbox from '$src/lib/components/atoms/Checkbox.svelte';
  import Button from '$src/lib/components/Button.svelte';
  import TopNavigation from '$src/lib/components/molecules/navigation/TopNavigation.svelte';

  let ownership_isChecked = false;
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
    <p class="pb-4 text-3xl font-semibold text-slate-700 dark:text-grey">
      Here's the less<br />interesting <span class="text-primary">stuff</span>
    </p>
    <p class="text-[15px]/[24px] font-medium text-slate-500 dark:text-slate-300">
      Yeah, we know. We still recommend you read this information carefully.
    </p>
  </div>
  <div class="mt-4 flex flex-col space-y-4">
    <!-- TODO: feature disabled: "terms and conditions" -->
    <!-- <div
      class="flex items-center justify-between space-x-4 rounded-xl border border-slate-200 bg-white p-4 dark:border-slate-600 dark:bg-dark"
    >
      <div class="grow">
        <p class="text-[13px]/[24px] font-medium text-slate-800 dark:text-grey">Data</p>
        <p class="text-[12px]/[20px] font-medium text-slate-500 dark:text-slate-300">
          I have read and accept the <span class="text-primary">terms and conditions</span>
        </p>
      </div>
      <div>
        <button
          use:melt={$root}
          class="flex h-6 w-6 appearance-none items-center justify-center
            rounded-md border-[1.5px] border-slate-300 p-[6px] text-white dark:border-slate-600 dark:text-dark
            {$isChecked ? 'border-none bg-primary' : 'bg-white dark:bg-dark'}"
          id="checkbox"
        >
          {#if $isChecked}
            <Check class="h-3 w-3" />
          {/if}
          <input use:melt={$input} />
        </button>
      </div>
    </div> -->
    <div
      class="flex items-center justify-between rounded-xl border border-slate-200 bg-white p-4 dark:border-slate-600 dark:bg-dark"
    >
      <div class="grow">
        <p class="text-[13px]/[24px] font-medium text-slate-800 dark:text-grey">Ownership</p>
        <p class="text-[12px]/[20px] font-medium text-slate-500 dark:text-slate-300">
          <!-- I understand that I am solely responsible for <span class="text-primary">my backups</span> -->
          I understand that I am solely responsible for my data
        </p>
      </div>
      <div class="pl-4">
        <Checkbox
          checked={ownership_isChecked}
          on:change={(e) => {
            ownership_isChecked = e.detail;
          }}
        />
      </div>
    </div>
  </div>
</div>
<!-- Actions -->
<!-- <div class="space-y-[10px] rounded-t-3xl bg-white p-6" in:fly={{ y: 154, delay: 300, opacity: 1 }}> -->
<div class="rounded-t-3xl bg-white p-6 dark:bg-dark" in:fade={{ delay: 200 }} out:fade={{ duration: 200 }}>
  <Button label="Continue" on:click={() => goto('/welcome/customize/name')} disabled={!ownership_isChecked} />
</div>

<style>
  .content-height {
    height: calc(100vh - var(--safe-area-inset-top));
  }
</style>
