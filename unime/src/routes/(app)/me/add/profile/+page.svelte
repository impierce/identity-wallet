<script lang="ts">
  import { onMount } from 'svelte';

  import { goto } from '$app/navigation';
  import LL from '$i18n/i18n-svelte';
  import { get } from 'svelte/store';
  import { superForm } from 'sveltekit-superforms';
  import { zod4 } from 'sveltekit-superforms/adapters';
  import { z } from 'zod';

  import { info } from '@tauri-apps/plugin-log';

  import { Button, Switch, TextInput, TopNavBar } from '$lib/components';
  import SelectCountry from '$lib/components/forms/SelectCountry.svelte';
  import { dispatch } from '$lib/dispatcher';
  import { HeartFillIcon, IdentificationBadgeRegularIcon } from '$lib/icons';
  import { naturalPerson as schema } from '$lib/schemas/pid';
  import { state as appState } from '$lib/stores';

  // Create a zod schema with i18n error messages
  const naturalPerson = schema(get(LL));

  // Initialize the form
  const init: z.infer<typeof naturalPerson> = {
    family_name: '',
    given_name: '',
    birth_date: new Date(),
    birth_place: '',
    nationality: [],
  };

  const { form, allErrors } = superForm(init, {
    validators: zod4(naturalPerson),
  });

  // Bottom action: Add to favourites, Create profile
  let checked = $state(true);
  // svelte-ignore non_reactive_update
  let labelInput: HTMLInputElement | undefined = undefined;

  // Loading state while the credential is created
  let loading = $state(false);

  let profileName = $state('');

  let primary_nationality: string | undefined = $state();
  let secondary_nationality: string | undefined = $state();

  let show_secondary_nationality = $state(false);

  $effect(() => {
    let nationalities: string[] = [];
    if (primary_nationality) {
      nationalities.push(primary_nationality);
    } else {
      nationalities = [];
    }
    if (secondary_nationality) {
      nationalities.push(secondary_nationality);
    }
    $form.nationality = nationalities;
  });

  function createProfile() {
    loading = true;

    const credentialSubject = {
      ...$form,
    };

    const validationResult = naturalPerson.safeParse(credentialSubject);
    info(`Validation result: ${JSON.stringify(validationResult)}`);

    let name = profileName;
    if (profileName.trim().length === 0) {
      name = $LL.ADD_CREDENTIALS.PROFILE.ADD.LABEL_PLACEHOLDER();
    }

    dispatch({
      type: '[Credential] Self Issue',
      payload: {
        type: 'profile',
        data: {
          type: ['VerifiableCredential', 'NaturalPersonCredential'],
          issuanceDate: new Date().toISOString(), // TODO: the backend should use `metadata` instead
          name,
          credentialSubject,
        },
        is_favorite: checked,
        icon: 'IdentificationBadge',
      },
    });
  }

  onMount(() => {
    if (!profileName) {
      labelInput?.focus();
    }
    if ($appState.dev_mode !== 'Off') {
      profileName = 'My Profile';
      $form = {
        given_name: 'Ferris',
        family_name: 'Rustacean',
        birth_date: new Date('2023-04-01'),
        birth_place: 'Atlantic Ocean',
        nationality: ['NL', 'BQ-BO'],
      };
      primary_nationality = $form.nationality.at(0);
      // secondary_nationality = $form.nationality.at(1);
    }
  });
</script>

<TopNavBar
  on:back={() => goto('/me/add')}
  title={$LL.ADD_CREDENTIALS.PROFILE.ADD.NAVBAR_TITLE()}
  class="sticky top-0 z-10"
/>

<!-- Extra bottom padding is added to make room for the bottom actions -->
<div class="relative flex flex-col pb-20">
  <div class="flex grow flex-col items-center p-4 pt-0">
    <div class="my-5 flex h-[121px] flex-col items-center space-y-4">
      <!-- PaddedIcon -->
      <div class="flex h-[75px] w-[75px] items-center justify-center rounded-3xl bg-background-alt">
        <IdentificationBadgeRegularIcon class="size-7 text-slate-800 dark:text-grey" />
      </div>
      <div
        class="w-full text-center text-[22px]/[30px] font-semibold tracking-tight text-slate-700 outline-hidden dark:text-grey"
      >
        {profileName}
      </div>
    </div>
    <div class="w-full space-y-4">
      <div>
        <TextInput
          id="label"
          label={$LL.ADD_CREDENTIALS.PROFILE.ADD.LABEL()}
          placeholder={$LL.ADD_CREDENTIALS.PROFILE.ADD.LABEL_PLACEHOLDER()}
          bind:value={profileName}
          bind:ref={labelInput}
        />
        <div class="pt-1 text-[12px]/[14px] font-medium text-primary">
          {$LL.ADD_CREDENTIALS.LABEL_DISCLAIMER()}
        </div>
        <!-- Divider -->
        <div class="my-4 h-px bg-slate-300"></div>
      </div>

      <TextInput
        id="given_name"
        label={$LL.ADD_CREDENTIALS.PROFILE.ADD.FIRST_NAME_LABEL()}
        placeholder={$LL.ADD_CREDENTIALS.PROFILE.ADD.FIRST_NAME_PLACEHOLDER()}
        bind:value={$form.given_name}
      />

      <TextInput
        id="family_name"
        label={$LL.ADD_CREDENTIALS.PROFILE.ADD.LAST_NAME_LABEL()}
        placeholder={$LL.ADD_CREDENTIALS.PROFILE.ADD.LAST_NAME_PLACEHOLDER()}
        bind:value={$form.family_name}
      />

      <!-- birth_date -->
      <!-- TODO: use proper date picker -->
      <!-- <DateField label={$LL.ADD_CREDENTIALS.PROFILE.ADD.BIRTH_DATE_LABEL()} /> -->

      <TextInput
        id="birth_place"
        label={$LL.ADD_CREDENTIALS.PROFILE.ADD.BIRTH_PLACE_LABEL()}
        placeholder={$LL.ADD_CREDENTIALS.PROFILE.ADD.BIRTH_PLACE_PLACEHOLDER()}
        bind:value={$form.birth_place}
      />

      <div class="flex w-full flex-col items-center gap-4">
        <SelectCountry label={$LL.ADD_CREDENTIALS.PROFILE.ADD.NATIONALITY_LABEL()} bind:value={primary_nationality} />

        {#if show_secondary_nationality}
          <SelectCountry
            label={$LL.ADD_CREDENTIALS.PROFILE.ADD.NATIONALITY_LABEL()}
            bind:value={secondary_nationality}
          />
        {:else}
          <!-- TODO: allow multiple nationalities -->
          <!-- <button
            onclick={() => (show_secondary_nationality = true)}
            class="w-fit rounded-sm px-3 py-2 text-sm font-medium text-primary hover:bg-slate-100"
          >
            Add another nationality
          </button> -->
        {/if}
      </div>
    </div>
  </div>
  <!-- DEBUG -->
  <!-- <div class="break-all px-4 text-xs">{JSON.stringify($form)}</div> -->
</div>

<div
  class="fixed bottom-(--safe-area-inset-bottom) left-0 z-10 flex h-36 w-full flex-col gap-5 rounded-t-3xl bg-background-alt p-6 shadow-sm"
>
  <Switch {checked} onCheckedChange={({ next }) => (checked = next)}>
    <div class="flex items-center gap-4 px-4">
      <HeartFillIcon class="size-5 text-primary" />
      <span class="text-[13px]/[24px] font-medium text-slate-800 dark:text-grey">
        {$LL.ADD_CREDENTIALS.FAVORITES_TOGGLE_LABEL()}
      </span>
    </div>
  </Switch>
  <Button
    label={$LL.ADD_CREDENTIALS.PROFILE.ADD.CREATE_BUTTON()}
    disabled={$allErrors.length > 0}
    {loading}
    on:click={() => createProfile()}
  />
</div>
