<script lang="ts">
  import { goto } from '$app/navigation';
  import { formatRelative, type Locale } from 'date-fns';
  import { de, enGB, enUS, nl } from 'date-fns/locale';

  import type { HistoryCredential } from '@bindings/HistoryCredential';

  import ListItemCard from '$lib/components/molecules/ListItemCard.svelte';
  import { state } from '$src/stores';

  export let title: string;
  export let date: string;
  export let credentials: HistoryCredential[] = [];

  function getRelativeDate() {
    let locale: Locale;

    switch ($state.profile_settings.locale) {
      case 'en-US': {
        locale = enUS;
        break;
      }
      case 'en-GB': {
        locale = enGB;
        break;
      }
      case 'de-DE': {
        locale = de;
        break;
      }
      case 'nl-NL': {
        locale = nl;
        break;
      }
    }

    function capitalizeFirstLetter(str: string) {
      return str.charAt(0).toUpperCase() + str.slice(1);
    }

    const dateStr = formatRelative(date, Date.now(), { locale });
    return capitalizeFirstLetter(dateStr);
  }

  function navigateToCredential(credentialId: string) {
    const credential = $state.credentials.find((cred) => cred.id === credentialId);

    if (credential) {
      if (credential.data.type.includes('OpenBadgeCredential')) {
        goto(`/badges/${credential.id}`);
      } else {
        goto(`/credentials/${credential.id}`);
      }
    }
  }
</script>

<div class="flex flex-col">
  <p class="text-[14px]/[22px] font-medium text-slate-800 dark:text-grey">{title}</p>
  <p class="text-[12px]/[20px] font-medium text-slate-500 dark:text-slate-300">
    {getRelativeDate()}
  </p>
  {#if credentials.length > 0}
    <div class="mt-[12px] rounded-xl border border-slate-200 bg-white p-[3px] dark:border-slate-600 dark:bg-dark">
      {#each credentials as credential}
        <ListItemCard
          id={credential.id}
          title={credential.title}
          description={credential.issuer_name}
          on:click={() => navigateToCredential(credential.id)}
        />
      {/each}
    </div>
  {/if}
</div>
