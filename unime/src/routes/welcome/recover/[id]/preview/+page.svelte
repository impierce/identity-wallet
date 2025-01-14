<script lang="ts">
  import { goto } from '$app/navigation';
  import LL from '$i18n/i18n-svelte';

  import { Button, Image, ListItemCard, PaddedIcon, TopNavBar } from '$lib/components';
  import {
    CertificateLightIcon,
    CloudFillIcon,
    CubeLightIcon,
    EyeClosedRegularIcon,
    EyeRegularIcon,
    LinkLightIcon,
    VaultFillIcon,
  } from '$lib/icons';
</script>

<TopNavBar title={$LL.ONBOARDING.WELCOME.RECOVER_PROFILE()} on:back={() => history.back()} />
<div class="mt-8 flex grow flex-col">
  <div class="grow p-4">
    <div class="flex w-full justify-center">
      <PaddedIcon icon={VaultFillIcon} />
    </div>
    <!-- <div class="mt-8 grow p-4" in:fade={{ delay: 200 }} out:fade={{ duration: 200 }}> -->
    <div class="px-2 pb-8 pt-4">
      <p class="pb-4 text-3xl font-semibold text-slate-700 dark:text-grey">
        {"Here's a"} <span class="text-primary">{'preview'}</span>
      </p>
      <p class="text-[14px]/[22px] font-medium text-slate-500 dark:text-slate-300">
        {"You'll recover the following data"}
      </p>
    </div>
    <div class="grow space-y-2">
      <ListItemCard id={''} title={'User profile'} description={'Ferris'} />
      <ListItemCard id={''} title={'Credentials'} description={'12 items'}>
        <div
          slot="image"
          class="mr-4 flex h-12 w-12 min-w-[48px] items-center justify-center overflow-hidden rounded-lg bg-silver p-1 dark:bg-navy"
        >
          <Image id={'connection.id'} iconFallback={CubeLightIcon} imgClass="p-1" />
        </div>
      </ListItemCard>
      <ListItemCard id={''} title={'Badges'} description={'5 items'}>
        <div
          slot="image"
          class="mr-4 flex h-12 w-12 min-w-[48px] items-center justify-center overflow-hidden rounded-lg bg-silver p-1 dark:bg-navy"
        >
          <Image id={'connection.id'} iconFallback={CertificateLightIcon} imgClass="p-1" />
        </div>
      </ListItemCard>
      <ListItemCard id={''} title={'Connections'} description={'7 items'}>
        <div
          slot="image"
          class="mr-4 flex h-12 w-12 min-w-[48px] items-center justify-center overflow-hidden rounded-lg bg-silver p-1 dark:bg-navy"
        >
          <Image id={'connection.id'} iconFallback={LinkLightIcon} imgClass="p-1" />
        </div>
      </ListItemCard>
    </div>
  </div>
  <!-- Bottom actions -->
  <div class="z-10 space-y-3 rounded-t-3xl bg-white p-6 dark:bg-dark">
    <Button
      label={'Restore'}
      on:click={() => {
        // onboarding_state.set({});
        goto('/me');
      }}
    />
    <Button
      label={'Go back'}
      variant="secondary"
      on:click={() => {
        goto('/welcome/recover');
      }}
    />
  </div>
</div>
