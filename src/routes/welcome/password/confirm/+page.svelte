<script lang="ts">
  import { goto } from '$app/navigation';
  import { fade } from 'svelte/transition';

  import { BottomDrawer, TopNavigation } from '@impierce/ui-components';
  import { melt } from '@melt-ui/svelte';

  import LL from '$src/i18n/i18n-svelte';
  import Button from '$src/lib/components/Button.svelte';
  import { dispatch } from '$src/lib/dispatcher';
  import { onboarding_state } from '$src/stores';

  import CheckCircle from '~icons/ph/check-circle-fill';
  import Circle from '~icons/ph/circle';
  import Eye from '~icons/ph/eye';
  import EyeClosed from '~icons/ph/eye-closed';
  import Smiley from '~icons/ph/smiley';
  import SmileySad from '~icons/ph/smiley-sad';

  // TODO: remove
  const initialPasswordValue = 'sup3rSecr3';

  let passwordsEqual: boolean | undefined;
  let showPassword = false;
</script>

<TopNavigation on:back={() => history.back()} title="Confirm Password" />
<!-- Content -->
<div class="mt-8 grow p-4" in:fade={{ delay: 200 }} out:fade={{ duration: 200 }}>
  <div class="pb-8 pt-4">
    <p class="pb-8 text-3xl font-semibold text-slate-700">
      Please confirm your new <span class="text-primary">password</span>
    </p>
    <p class="text-[15px]/[24px] font-medium text-slate-500">
      You need to create a strong password to secure your backup.
    </p>
    <!-- <div class="mt-[70px] flex w-full items-center justify-center" /> -->
  </div>
  <div class="relative">
    <input
      type={showPassword ? 'text' : 'password'}
      class="h-12 w-full rounded-xl border border-slate-300 px-4 py-3 text-[13px]/[24px] text-slate-500"
      placeholder="Retype your password"
      value={initialPasswordValue}
      on:input={(e) => {
        console.log(e.target.value, $onboarding_state.password);
        passwordsEqual = e.target.value === $onboarding_state.password;
      }}
    />
    <div class="absolute right-3 top-0 flex h-full items-center">
      <button class="rounded-full p-2" on:click={() => (showPassword = !showPassword)}>
        {#if showPassword}
          <Eye />
        {:else}
          <EyeClosed />
        {/if}
      </button>
    </div>
  </div>
  {#if passwordsEqual !== undefined}
    <div class="mt-8 flex items-center justify-center">
      {#if passwordsEqual}
        <Smiley class="mr-[10px] h-5 w-5 text-primary" />
        <p class="text-[13px]/[24px] font-medium text-primary">Passwords match</p>
      {:else}
        <SmileySad class="mr-[10px] h-5 w-5 text-rose-500" />
        <p class="text-[13px]/[24px] font-medium text-rose-500">Passwords do not match</p>
      {/if}
    </div>
  {/if}
</div>

<div class="rounded-t-3xl bg-white p-6" in:fade={{ delay: 200 }} out:fade={{ duration: 200 }}>
  <Button
    label="Continue"
    on:click={() => goto('/welcome/password/completed')}
    disabled={!passwordsEqual}
  />
</div>
