<script lang="ts">
  import { onMount } from 'svelte';

  import { goto } from '$app/navigation';
  import { page } from '$app/state';
  import LL from '$i18n/i18n-svelte';

  import { warn } from '@tauri-apps/plugin-log';

  import { TopNavBar } from '$lib/components';
  import { state as appState } from '$lib/stores';

  import { tintFor } from '../../../../../../prompt/accept-connection/ecosystem';
  import EcosystemAvatar from '../../../../../../prompt/accept-connection/EcosystemAvatar.svelte';
  import MemberRow from '../../../../../../prompt/accept-connection/MemberRow.svelte';

  $: connection = $appState.connections.find((item) => item.id === page.params.id);
  $: index = Number(page.params.ecosystemIndex);
  $: ecosystem = (connection?.ecosystems ?? [])[index];
  $: members = ecosystem?.members ?? [];

  onMount(() => {
    if (!connection || !ecosystem) {
      warn(`No ecosystem found at index: \`${page.params.ecosystemIndex}\``);
      goto(connection ? `/activity/connection/${connection.id}/ecosystems` : '/activity');
    }
  });
</script>

<div class="content-height flex hide-scrollbar flex-col items-stretch overflow-y-auto bg-background-alt">
  <TopNavBar title={ecosystem?.name ?? ''} on:back={() => history.back()} class="sticky top-0 z-10" />

  {#if ecosystem}
    <div class="flex min-h-full flex-col px-4 pb-7">
      <div class="flex flex-col items-center gap-3 rounded-2xl {tintFor(ecosystem.name)} px-4 py-6">
        <EcosystemAvatar
          name={ecosystem.name}
          logoUri={ecosystem.logo_uri}
          class="size-[75px] rounded-2xl bg-white/50"
          textClass="text-[22px]/[30px]"
          isTempAsset={false}
        />
        <p class="text-center text-[20px]/[28px] font-semibold">{ecosystem.name}</p>
        <span class="rounded-md bg-white/50 px-2 py-1 text-[12px]/[18px] font-medium">
          {$LL.SCAN.CONNECTION_REQUEST.ECOSYSTEM_MEMBERS({ count: ecosystem.member_count })}
        </span>
      </div>

      {#if ecosystem.description}
        <section class="pt-6">
          <p
            class="border-b border-slate-200 pb-2 text-[15px]/[24px] font-semibold text-slate-800 dark:border-slate-600 dark:text-grey"
          >
            {$LL.SCAN.CONNECTION_REQUEST.ECOSYSTEM_ABOUT()}
          </p>
          <p class="pt-3 text-[13px]/[22px] font-normal text-slate-500 dark:text-slate-300">
            {ecosystem.description}
          </p>
        </section>
      {/if}

      <section class="pt-6">
        <p class="pb-2 text-[15px]/[24px] font-semibold text-slate-800 dark:text-grey">
          {$LL.SCAN.CONNECTION_REQUEST.ECOSYSTEM_OWNER()}
        </p>
        <div class="rounded-xl border border-slate-200 bg-white px-3 dark:border-slate-600 dark:bg-dark">
          <MemberRow member={ecosystem.ecosystem_leader} isTempAsset={false} />
        </div>
      </section>

      {#if members.length > 0}
        <section class="pt-6">
          <div class="flex items-center gap-2 border-b border-slate-200 pb-2 dark:border-slate-600">
            <p class="text-[15px]/[24px] font-semibold text-slate-800 dark:text-grey">
              {$LL.SCAN.CONNECTION_REQUEST.ECOSYSTEM_MEMBERS_HEADING()}
            </p>
            <span
              class="rounded-md bg-slate-100 px-2 py-1 text-[11px]/[16px] font-medium text-slate-600 dark:bg-slate-700 dark:text-slate-200"
            >
              {ecosystem.member_count}
            </span>
          </div>
          <div class="divide-y divide-slate-200 dark:divide-slate-600">
            {#each members as member}
              <MemberRow {member} isTempAsset={false} />
            {/each}
          </div>
        </section>
      {/if}
    </div>
  {/if}
</div>

<style>
  .content-height {
    height: calc(100vh - var(--safe-area-inset-top) - var(--safe-area-inset-bottom));
  }
</style>
