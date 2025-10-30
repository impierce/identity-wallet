<script lang="ts">
  import LL from '$i18n/i18n-svelte';

  import { Image, ListItemCard } from '$lib/components';

  export let id: string;
  export let data = undefined;
</script>

{#if data}
  <div class="flex w-full flex-col items-center justify-center space-y-4 py-6">
    <div class="size-[75px] rounded-3xl bg-white p-4">
      <Image
        {id}
        imgClass="h-full w-full rounded-2xl"
        iconFallback="BankLight"
        iconClass="size-6 dark:text-slate-800"
      />
    </div>

    <div class="dark:text-grey text-center text-[22px]/[30px] font-semibold text-slate-700">Data shared</div>
  </div>
  <div class="dark:bg-dark flex w-full flex-col rounded-2xl border border-slate-200 bg-white p-1 dark:border-slate-600">
    <ListItemCard id={'_'} title="Email address" description="ferris.rustacean@example.org" />
    <ListItemCard id={'_'} title="Home address" description="Sunset Rd 1337, Iron Islands" />
  </div>
{:else}
  <div class="flex h-full flex-col items-center justify-center">
    <p class="text-[14px]/[22px] font-medium text-slate-500 dark:text-slate-300">{$LL.CONNECTION.DATA.EMPTY()}</p>
  </div>
{/if}
