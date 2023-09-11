<script lang="ts">
  import { goto } from '$app/navigation';
  import { fade } from 'svelte/transition';

  import { TopNavigation } from '@impierce/ui-components';
  import '@lottiefiles/lottie-player';

  import LL from '$src/i18n/i18n-svelte';
  import Button from '$src/lib/components/Button.svelte';
  import { dispatch } from '$src/lib/dispatcher';
  import { onboarding_state } from '$src/stores';

  import Shield from '~icons/ph/shield-fill';
</script>

<!-- TODO: should we show this screen AFTER a successful creation of a stronghold? -->
<TopNavigation on:back={() => history.back()} title="Confirm Password" />
<!-- Content -->
<div class="mt-8 grow p-4" in:fade={{ delay: 200 }}>
  <div class="pb-8 pt-4">
    <p class="pb-8 text-3xl font-semibold text-slate-700 dark:text-grey">
      Your UniMe profile is now <span class="text-primary">protected</span>
    </p>
  </div>
  <div
    class="flex flex-col items-center justify-center space-y-6 rounded-3xl bg-white p-5 dark:bg-dark"
  >
    <p class="text-[22px]/[30px] font-semibold text-primary">Safe & Secure.</p>
    <div class="relative">
      <div class="relative z-10">
        <div class="text-[100px]/[100px]"><Shield class="text-primary" /></div>
        <span class="absolute left-[calc(50%_-_22px)] top-[calc(50%_-_22px)] text-[44px]/[44px]">
          {@html $onboarding_state.picture}
        </span>
      </div>
      <div class="absolute left-1/2 top-1/2 z-0 -translate-x-1/2 -translate-y-1/2">
        <lottie-player
          src="/lottiefiles/bubble-burst-confetti-ajgRKUnNJ7.json"
          autoplay
          loop
          speed={0.25}
          mode="normal"
          style="width: 320px"
        />
      </div>
    </div>
    <p class="text-[22px]/[30px] font-semibold text-primary">
      Nice Job, {$onboarding_state.name}!
    </p>
    <!-- Hint: backup -->
    <!-- <div class="bg-slate-100 p-4 rounded-2xl w-full">
      <p class="text-sm text-slate-800">Let's create a quick backup.</p>
    </div> -->
  </div>
</div>

<div class="rounded-t-3xl bg-white p-6 dark:bg-dark" in:fade={{ delay: 200 }}>
  <Button
    label="Continue"
    on:click={async () => {
      await dispatch({
        type: '[DID] Create new',
        payload: {
          name: $onboarding_state.name,
          picture: $onboarding_state.picture,
          theme: $onboarding_state.theme,
          password: 'sup3rSecr3t'
        }
      });
    }}
  />
</div>
