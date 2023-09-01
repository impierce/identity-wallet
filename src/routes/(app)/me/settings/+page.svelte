<script lang="ts">
  import { Eye, Heart } from 'svelte-heros-v2';
  import { fly } from 'svelte/transition';

  import { BottomDrawer, TopNavigation } from '@impierce/ui-components';
  import { melt } from '@melt-ui/svelte';

  import { dispatch } from '$lib/dispatcher';
  import LL from '$src/i18n/i18n-svelte';
  import { state } from '$src/stores';

  import WarningCircle from '~icons/ph/warning-circle-fill';

  let IOTA_MOCK_DID = 'did:iota:H3C2AVvLMv6gmMNam3uVAjZpfkcJCwDwnZn6z3wXmqPV';
</script>

<TopNavigation on:back={() => history.back()} title="Settings" />
<div class="flex min-h-full flex-col bg-bg-secondary dark:bg-bg-dark-secondary">
  <div class="flex grow flex-col justify-between p-6" in:fly={{ x: 32, opacity: 1 }}>
    <div class="space-y-4">
      <!-- DIDs -->
      <div class="space-y-2">
        <p class="font-medium text-neutral-700 dark:text-neutral-300">{$LL.YOUR_DIDS()}</p>
        <div class="flex items-center space-x-2">
          <div
            class="flex flex-grow items-center justify-center break-all rounded-lg bg-white px-4 py-4"
            data-testid="primary-did"
          >
            <p class="font-mono text-sm text-neutral-500 dark:bg-neutral-800">
              {$state?.active_profile?.primary_did}
            </p>
            <button class="-mr-2 ml-2 rounded-full p-2 hover:bg-slate-200 active:bg-slate-300"
              ><Eye class="text-primary" strokeWidth="2" /></button
            >
          </div>
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

      <!-- Profile -->
      <div class="space-y-2">
        <p class="font-medium text-neutral-700 dark:text-neutral-300">Profile</p>

        <div
          class="flex h-14 items-center justify-between rounded-lg bg-white px-4 py-2 dark:bg-bg-dark-primary"
        >
          <p>Profile name</p>
          <div class="flex flex-col items-center justify-center space-y-4">
            <input
              type="text"
              class="h-[40px] w-[192px] rounded-lg border px-4 py-2 text-right text-base font-medium text-primary dark:bg-slate-800"
              value={$state?.active_profile?.name}
              on:input={(e) => console.log(e.target.value)}
            />
          </div>
        </div>

        <div
          class="flex h-14 items-center justify-between rounded-lg bg-white px-4 py-2 dark:bg-bg-dark-primary"
        >
          <p>Password</p>
          <div class="flex flex-col items-center justify-center space-y-4">
            <button class="rounded-lg px-2 py-1 font-medium text-primary">Change</button>
          </div>
        </div>

        <div
          class="flex h-14 items-center justify-between rounded-lg bg-white px-4 py-2 dark:bg-bg-dark-primary"
        >
          <p>Backup</p>
          <div class="flex items-center justify-center">
            <WarningCircle class="h-6 w-6 text-red-600" />
            <button class="rounded-lg px-2 py-1 font-medium text-red-600">Activate backups</button>
          </div>
        </div>
      </div>

      <!-- App Settings -->
      <div class="space-y-2">
        <p class="font-medium text-neutral-700 dark:text-neutral-300">{$LL.APP_SETTINGS()}</p>

        <!-- <div class="flex items-center justify-between rounded-lg bg-white p-4">
          <p>Language</p>
          <div class="flex flex-col items-center justify-center space-y-4">
            <LanguageSelect
              selected={$state?.locale}
              on:value={(e) =>
                dispatch({ type: '[Settings] Set locale', payload: { locale: e.detail } })}
            />
          </div>
        </div> -->

        <div
          class="flex h-14 items-center justify-between rounded-lg bg-white px-4 py-2 dark:bg-bg-dark-primary"
        >
          <p>Theme</p>
          <div class="flex items-center justify-center">
            <button
              class="rounded-lg px-2 py-1 font-medium text-primary"
              on:click={() =>
                dispatch({
                  type: '[Settings] Update profile',
                  payload: { theme: $state.active_profile.theme === 'light' ? 'dark' : 'light' }
                })}>Toggle dark mode</button
            >
          </div>
        </div>

        <div
          class="flex h-14 items-center justify-between rounded-lg bg-white p-4 dark:bg-bg-dark-primary"
        >
          <p>Onboarding journey</p>
          <div class="flex flex-col items-center justify-center space-y-4">
            <!-- <LanguageSelect
              selected={$state?.locale}
              on:value={(e) =>
                dispatch({ type: '[Settings] Set locale', payload: { locale: e.detail } })}
            /> -->
            <button class="rounded-lg px-2 py-1 font-medium text-primary">Restart</button>
          </div>
        </div>

        <div
          class="flex h-14 items-center justify-between rounded-lg bg-white p-4 dark:bg-bg-dark-primary"
        >
          <p>Dismissed hints & tips</p>
          <div class="flex flex-col items-center justify-center space-y-4">
            <!-- <LanguageSelect
              selected={$state?.locale}
              on:value={(e) =>
                dispatch({ type: '[Settings] Set locale', payload: { locale: e.detail } })}
            /> -->
            <button class="rounded-lg px-2 py-1 font-medium text-primary">Re-enable</button>
          </div>
        </div>
      </div>
    </div>
    <!-- Footer -->
    <div class="flex flex-col items-center pt-4 text-sm font-medium text-slate-400">
      <div class="pb-4">
        <BottomDrawer
          titleText="Reset app"
          descriptionText="Are you sure you want to reset the entire app and lose all unsaved data?"
        >
          <button
            slot="trigger"
            let:trigger
            class="rounded bg-red-100 px-4 py-2 text-red-600"
            use:melt={trigger}>{$LL.RESET_APP()}</button
          >
          <div slot="content" class="flex flex-col">
            <!-- more content -->
            <button
              class="w-full rounded-lg bg-red-600 px-4 py-2 text-white"
              on:click={() => dispatch({ type: '[App] Reset' })}>Yes, delete everything</button
            >
          </div>
          <button
            slot="close"
            let:close
            use:melt={close}
            class="mt-2 w-full rounded-lg border bg-white px-4 py-2 text-neutral-700">Cancel</button
          >
        </BottomDrawer>
      </div>

      <div>0.2.3</div>
      <div class="flex items-center pb-4">
        <p>Built with Tauri</p>
        <Heart variation="solid" size="18" class="pl-1" />
      </div>
      <div>GPL-3.0</div>
      <div>2023 Impierce Technologies</div>
    </div>
  </div>
</div>
