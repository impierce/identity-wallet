<script lang="ts">
  import { goto } from '$app/navigation';
  import LL from '../../i18n/i18n-svelte';
  import { BottomNavigation, Button } from '@impierce/ui-components';
  import { Link } from 'svelte-heros-v2';
  import { state } from '../../stores';
  import LocaleSelect from '$lib/LocaleSelect.svelte';
  import { fade, fly } from 'svelte/transition';
</script>

<div class="min-h-screen">
  <div class="flex flex-col space-y-8 p-8" in:fly={{ x: 32, opacity: 1 }}>
    <div class="space-y-4">
      <h2 class="font-semibold text-slate-700">{$LL.YOUR_DIDS()}</h2>
      <div class="flex space-x-2">
        <div
          class="break-all rounded-lg bg-slate-200 px-4 py-2 font-mono text-sm font-medium text-slate-700"
          data-testid="primary-did"
        >
          {$state?.active_profile?.primary_did}
        </div>
        <button class="rounded-lg bg-slate-200 p-4"
          ><Link class="text-violet-700" strokeWidth="2" /></button
        >
      </div>
      <div class="flex space-x-2">
        <div
          class="flex-grow break-all rounded-lg bg-slate-200 px-4 py-2 font-mono text-sm font-medium text-slate-700"
          data-testid="secondary-did"
        >
          did:atoi:1234567890
        </div>
        <button class="rounded-lg bg-slate-200 p-4"
          ><Link class="text-violet-700" strokeWidth="2" /></button
        >
      </div>
    </div>
    <div class="space-y-4">
      <h2 class="font-semibold text-slate-700">{$LL.APP_SETTINGS()}</h2>
      <div class="flex flex-col items-center justify-center space-y-4">
        <LocaleSelect />
        <Button label={$LL.RESET_APP()} type="cancel" />
      </div>
    </div>
    <div class="flex flex-col items-center text-sm font-medium text-slate-400">
      <div>0.1.0</div>
      <div>GPL-3.0 2023 Impierce Technologies</div>
    </div>
  </div>
  <div class="sticky top-[100vh]">
    <BottomNavigation active="settings" on:profile={() => goto('/profile')} />
  </div>
</div>
