<script lang="ts">
  import { goto } from '$app/navigation';
  import LL from '$i18n/i18n-svelte';
  import { BottomNavBar, Button, LanguageSelect, TopNavigation } from '@impierce/ui-components';
  import { Link, Eye, Heart } from 'svelte-heros-v2';
  import { state } from '../../../../stores';
  import { fade, fly } from 'svelte/transition';
  import { dispatch } from '$lib/dispatcher';

  let IOTA_MOCK_DID = 'did:iota:H3C2AVvLMv6gmMNam3uVAjZpfkcJCwDwnZn6z3wXmqPV';
</script>

<TopNavigation on:back={() => history.back()} title="Settings" />
<div class="bg-white dark:bg-slate-700">
  <div class="flex flex-col space-y-8 p-8" in:fly={{ x: 32, opacity: 1 }}>
    <div class="space-y-4">
      <h2 class="font-semibold text-slate-700 dark:text-neutral-300">{$LL.YOUR_DIDS()}</h2>
      <div class="flex items-center space-x-2">
        <div
          class="flex-grow break-all rounded-lg bg-slate-200 px-4 py-2 font-mono text-sm font-medium text-slate-500 dark:bg-slate-800"
          data-testid="primary-did"
        >
          {$state?.active_profile?.primary_did}
        </div>
        <button class="rounded-full p-4 hover:bg-slate-200 active:bg-slate-300"
          ><Eye class="text-violet-700" strokeWidth="2" /></button
        >
      </div>
      <!-- <div class="flex items-center space-x-2">
        <div
          class="flex-grow break-all rounded-lg bg-slate-200 px-4 py-2 font-mono text-sm font-medium text-slate-500 dark:bg-slate-800"
          data-testid="secondary-did"
        >
          {IOTA_MOCK_DID}
        </div>
        <button class="rounded-full p-4 hover:bg-slate-200 active:bg-slate-300"
          ><Link class="text-violet-700" strokeWidth="2" /></button
        >
      </div> -->
    </div>
    <div class="space-y-4">
      <h2 class="font-semibold text-slate-700 dark:text-neutral-300">{$LL.APP_SETTINGS()}</h2>
      <div class="flex flex-col items-center justify-center space-y-4">
        <LanguageSelect
          selected={$state.locale}
          on:value={(e) =>
            dispatch({ type: '[Settings] Set locale', payload: { locale: e.detail } })}
        />
        <Button variant="destructive">{$LL.RESET_APP()}</Button>
      </div>
    </div>
    <div class="flex flex-col items-center text-sm font-medium text-slate-400">
      <div>0.2.0</div>
      <div class="flex items-center pb-4">
        <p>Built with Tauri</p>
        <Heart variation="solid" size="18" class="pl-1" />
      </div>
      <div>GPL-3.0</div>
      <div>2023 Impierce Technologies</div>
    </div>
  </div>
</div>
