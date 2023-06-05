<script lang="ts">
  import { goto } from '$app/navigation';
  import LL from '../../i18n/i18n-svelte';
  import { BottomNavigation, Button } from '@impierce/ui-components';
  import { Link } from 'svelte-heros-v2';
  import { state } from '../../stores';
  import LocaleSelect from '$lib/LocaleSelect.svelte';
  import { fade, fly } from 'svelte/transition';

  let IOTA_DID_SHIMMER_TESTNET =
    'did:iota:rms:0xe5d5a7c02874b0284ccc2f28bada68780cad26702e7e8186d521e9615a73bc45';
</script>

<div class="flex min-h-screen flex-col">
  <div class="flex grow flex-col space-y-8 p-8" in:fly={{ x: 32, opacity: 1 }}>
    <div class="space-y-4">
      <h2 class="font-semibold text-slate-700">{$LL.YOUR_DIDS()}</h2>
      <div class="flex items-center space-x-2">
        <div
          class="break-all rounded-lg bg-slate-200 px-4 py-2 font-mono text-sm font-medium text-slate-500"
          data-testid="primary-did"
        >
          {$state?.active_profile?.primary_did}
        </div>
        <button class="rounded-full p-4 hover:bg-slate-200 active:bg-slate-300"
          ><Link class="text-violet-700" strokeWidth="2" /></button
        >
      </div>
      <div class="flex items-center space-x-2">
        <div
          class="flex-grow break-all rounded-lg bg-slate-200 px-4 py-2 font-mono text-sm font-medium text-slate-500"
          data-testid="secondary-did"
        >
          {IOTA_DID_SHIMMER_TESTNET}
        </div>
        <button class="rounded-full p-4 hover:bg-slate-200 active:bg-slate-300"
          ><Link class="text-violet-700" strokeWidth="2" /></button
        >
      </div>
    </div>
    <div class="grow space-y-4">
      <h2 class="font-semibold text-slate-700">{$LL.APP_SETTINGS()}</h2>
      <div class="flex flex-col items-center justify-center space-y-4">
        <LocaleSelect />
        <Button label={$LL.RESET_APP()} type="cancel" />
      </div>
    </div>
    <div class="flex flex-col items-center text-sm font-medium text-slate-400">
      <div>0.1.0</div>
      <div>GPL-3.0</div>
      <div>2023 Impierce Technologies</div>
    </div>
  </div>
  <div class="sticky top-[100vh]">
    <BottomNavigation
      active="settings"
      on:profile={() => goto('/profile')}
      on:history={() => goto('/history')}
    />
  </div>
</div>
