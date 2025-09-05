<script lang="ts">
  import { onMount } from 'svelte';

  import { goto } from '$app/navigation';
  import LL from '$i18n/i18n-svelte';
  import { get } from 'svelte/store';
  import { superForm } from 'sveltekit-superforms';
  import { zod } from 'sveltekit-superforms/adapters';
  import { z } from 'zod';

  import { info } from '@tauri-apps/plugin-log';

  import { Button, Switch, TextInput, TopNavBar } from '$lib/components';
  import SelectCountry from '$lib/components/forms/SelectCountry.svelte';
  import { dispatch } from '$lib/dispatcher';
  import { HeartFillIcon, HouseRegularIcon } from '$lib/icons';
  import { residence as schema } from '$lib/schemas/pid';
  import { state as appState } from '$lib/stores';

  // Create a zod schema with i18n error messages
  const residence = schema(get(LL));

  // Initialize the form
  const init: z.infer<typeof residence> = {};

  const { form, enhance, allErrors } = superForm(init, {
    validators: zod(residence),
    validationMethod: 'oninput',
    SPA: true,
  });

  // Bottom action: Add to favourites, Add address
  let checked = $state(true);
  // svelte-ignore non_reactive_update
  let labelInput: HTMLInputElement | undefined = undefined;

  // Loading state while the credential is created
  let loading = $state(false);

  let addressName = $state('');

  function createAddress() {
    loading = true;

    const credentialSubject = {
      ...$form,
    };

    const validationResult = residence.safeParse(credentialSubject);
    info(`Validation result: ${JSON.stringify(validationResult)}`);

    let name = addressName;
    if (addressName.trim().length === 0) {
      name = $LL.ADD_CREDENTIALS.ADDRESS.ADD.LABEL_PLACEHOLDER();
    }

    dispatch({
      type: '[Credential] Self Issue',
      payload: {
        type: 'address',
        data: {
          type: ['VerifiableCredential', 'ResidenceCredential'],
          issuanceDate: new Date().toISOString(), // TODO: this shouldn't be necessary anymore, use `metadata` instead
          name,
          credentialSubject,
        },
        is_favorite: checked,
        icon: 'House',
      },
    });
  }

  onMount(() => {
    if (!addressName) {
      labelInput?.focus();
    }
    if ($appState.dev_mode !== 'Off') {
      addressName = 'Home address';
      $form = {
        resident_country: 'NL',
        resident_state: 'Noord-Holland',
        resident_city: 'Amsterdam',
        resident_postal_code: '1071 XX',
        resident_street: 'Molenstraat',
        resident_house_number: '1A',
      };
    }
  });
</script>

<TopNavBar
  on:back={() => goto('/me/add')}
  title={$LL.ADD_CREDENTIALS.ADDRESS.ADD.NAVBAR_TITLE()}
  class="sticky top-0 z-10"
/>

<!-- Extra bottom padding is added to make room for the bottom actions -->
<div class="relative flex flex-col pb-20">
  <div class="flex grow flex-col items-center p-4 pt-0">
    <div class="my-5 flex h-[121px] flex-col items-center space-y-4">
      <!-- PaddedIcon -->
      <div class="flex h-[75px] w-[75px] items-center justify-center rounded-3xl bg-background-alt">
        <HouseRegularIcon class="size-7 text-slate-800 dark:text-grey" />
      </div>
      <div
        class="w-full text-center text-[22px]/[30px] font-semibold tracking-tight text-slate-700 outline-none dark:text-grey"
      >
        {addressName}
      </div>
    </div>
    <div class="w-full space-y-4">
      <div>
        <TextInput
          id="label"
          label={$LL.ADD_CREDENTIALS.ADDRESS.ADD.LABEL()}
          placeholder={$LL.ADD_CREDENTIALS.ADDRESS.ADD.LABEL_PLACEHOLDER()}
          bind:value={addressName}
          bind:ref={labelInput}
        />
        <div class="pt-1 text-[12px]/[14px] font-medium text-primary">
          {$LL.ADD_CREDENTIALS.LABEL_DISCLAIMER()}
        </div>
        <!-- Divider -->
        <div class="my-4 h-px bg-slate-300"></div>
      </div>
      <!-- This form is actually never submitted as a web-based form, it is only used for validation -->
      <form method="POST" use:enhance>
        <div class="space-y-4">
          <SelectCountry bind:value={$form.resident_country} />
          <!-- Street, House number -->
          <div class="flex gap-4">
            <div class="grow">
              <TextInput
                id="resident_street"
                label={$LL.ADD_CREDENTIALS.ADDRESS.ADD.RESIDENT_STREET_LABEL()}
                bind:value={$form.resident_street}
              />
            </div>
            <div class="w-1/3">
              <TextInput
                id="resident_house_number"
                label={$LL.ADD_CREDENTIALS.ADDRESS.ADD.RESIDENT_HOUSE_NUMBER_LABEL()}
                bind:value={$form.resident_house_number}
              />
            </div>
          </div>
          <!-- Postal code, City -->
          <div class="flex gap-4">
            <div class="w-1/3">
              <TextInput
                id="resident_postal_code"
                label={$LL.ADD_CREDENTIALS.ADDRESS.ADD.RESIDENT_POSTAL_CODE_LABEL()}
                bind:value={$form.resident_postal_code}
              />
            </div>
            <div class="grow">
              <TextInput
                id="resident_city"
                label={$LL.ADD_CREDENTIALS.ADDRESS.ADD.RESIDENT_CITY_LABEL()}
                bind:value={$form.resident_city}
              />
            </div>
          </div>
          <TextInput
            id="resident_state"
            label={$LL.ADD_CREDENTIALS.ADDRESS.ADD.RESIDENT_STATE_LABEL()}
            bind:value={$form.resident_state}
          />
        </div>
      </form>
      <!-- DEBUG -->
      <!-- <pre class="text-xs">{JSON.stringify($form, null, 2)}</pre> -->
      <!-- <div class="break-all text-xs text-rose-700">errors: {JSON.stringify($allErrors, null, 2)}</div> -->
      <!-- <div class="break-all text-xs">constraints: {JSON.stringify($constraints)}</div> -->
      <!-- <div class="text-xs">tainted: {JSON.stringify($tainted, null, 2)}</div> -->
    </div>
  </div>
</div>

<div
  class="fixed bottom-[var(--safe-area-inset-bottom)] left-0 z-10 flex h-36 w-full flex-col gap-5 rounded-t-3xl bg-background-alt p-6 shadow"
>
  <Switch {checked} onCheckedChange={({ next }) => (checked = next)}>
    <div class="flex items-center gap-4 px-4">
      <HeartFillIcon class="size-5 text-primary" />
      <span class="text-[13px]/[24px] font-medium text-slate-800 dark:text-grey"
        >{$LL.ADD_CREDENTIALS.FAVORITES_TOGGLE_LABEL()}</span
      >
    </div>
  </Switch>
  <Button
    label={$LL.ADD_CREDENTIALS.ADDRESS.ADD.CREATE_BUTTON()}
    disabled={$allErrors.length > 0}
    {loading}
    on:click={() => createAddress()}
  />
</div>
