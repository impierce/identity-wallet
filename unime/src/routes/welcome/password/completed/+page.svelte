<script lang="ts">
  import { goto } from '$app/navigation';
  import { fade } from 'svelte/transition';

  import TopNavBar from '$src/lib/components/molecules/navigation/TopNavBar.svelte';

  import '@lottiefiles/lottie-player';

  import { dispatch } from '$lib/dispatcher';
  import LL from '$src/i18n/i18n-svelte';
  import Button from '$src/lib/components/atoms/Button.svelte';
  import { calculateInitials } from '$src/routes/(app)/me/utils';
  import { onboarding_state } from '$src/stores';

  import Shield from '~icons/ph/shield-fill';
</script>

<!-- TODO: should we show this screen AFTER a successful creation of a stronghold? -->
<TopNavBar on:back={() => history.back()} title={$LL.ONBOARDING.PASSWORD.COMPLETED.NAVBAR_TITLE()} />
<!-- Content -->
<div class="mt-8 grow p-4" in:fade={{ delay: 200 }}>
  <div class="pb-8 pt-4">
    <p class="pb-8 text-3xl font-semibold text-slate-700 dark:text-grey">
      {$LL.ONBOARDING.PASSWORD.COMPLETED.TITLE_1()}
      <span class="text-primary">{$LL.ONBOARDING.PASSWORD.COMPLETED.TITLE_2()}</span>
    </p>
  </div>
  <div class="flex flex-col items-center justify-center space-y-6 rounded-3xl bg-white p-5 dark:bg-dark">
    <p class="text-[22px]/[30px] font-semibold text-primary">{$LL.ONBOARDING.PASSWORD.COMPLETED.MESSAGE_1()}</p>
    <div class="relative">
      <div class="relative z-10">
        <div class="text-[100px]/[100px]"><Shield class="text-primary" /></div>
        <span class="absolute left-1/2 top-1/2 -translate-x-1/2 -translate-y-1/2 text-[36px]/[36px]">
          <p class="font-semibold tracking-tight text-white dark:text-dark">
            {calculateInitials($onboarding_state.name ?? '')}
          </p>
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
      {$LL.ONBOARDING.PASSWORD.COMPLETED.MESSAGE_2()}, {$onboarding_state.name}!
    </p>
    <!-- Hint: backup -->
    <!-- <div class="bg-slate-100 p-4 rounded-2xl w-full">
      <p class="text-sm text-slate-800">Let's create a quick backup.</p>
    </div> -->
  </div>
</div>

<div class="rounded-t-3xl bg-white p-6 dark:bg-dark" in:fade={{ delay: 200 }}>
  <Button
    label={$LL.CONTINUE()}
    on:click={async () => {
      await dispatch({
        type: '[DID] Create new',
        payload: {
          name: $onboarding_state.name ?? '',
          picture: '',
          theme: 'system',
          password: $onboarding_state.password ?? '',
        },
      });
    }}
  />
</div>
