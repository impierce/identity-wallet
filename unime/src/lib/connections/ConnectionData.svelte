<script lang="ts">
  import LL from '$src/i18n/i18n-svelte';
  import Image from '$src/lib/components/atoms/Image.svelte';
  import ListItemCard from '$src/lib/components/molecules/ListItemCard.svelte';

  export let id: string;
  export let data: any = undefined;
</script>

{#if data}
  <div class="flex w-full flex-col items-center justify-center space-y-4 py-6">
    <div class="h-[75px] w-[75px] rounded-3xl bg-white p-4">
      <Image {id} imgClass="h-full w-full rounded-2xl" iconFallback="Bank" iconClass="h-6 w-6 dark:text-slate-800" />
    </div>

    <div class="text-center text-[22px]/[30px] font-semibold text-slate-700 dark:text-grey">Data shared</div>
  </div>
  <div class="flex w-full flex-col rounded-2xl border border-slate-200 bg-white p-1 dark:border-slate-600 dark:bg-dark">
    <ListItemCard id={'_'} title="Email address" description="ferris.rustacean@example.org" />
    <ListItemCard id={'_'} title="Home address" description="Sunset Rd 1337, Iron Islands" />
  </div>
{:else}
  <div class="flex h-full flex-col items-center justify-center">
    <p class="text-[14px]/[22px] font-medium text-slate-500 dark:text-slate-300">{$LL.CONNECTION.DATA.EMPTY()}</p>
  </div>
{/if}
