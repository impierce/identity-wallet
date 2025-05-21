<script lang="ts">
  import { onMount } from 'svelte';

  import { goto } from '$app/navigation';
  import LL from '$i18n/i18n-svelte';

  import { Button, Switch, TopNavBar } from '$lib/components';
  import { HeartFillIcon, IdentificationBadgeRegularIcon } from '$lib/icons';

  // Bottom action: Add to favourites, Create profile
  let checked = $state(true);
  let labelInput: HTMLInputElement;

  // Loading state while the credential is created
  let loading = $state(false);

  let profileName = $state('');

  interface Field {
    id: string;
    label: string;
    placeholder: string;
    value: string;
    required?: boolean;
  }

  const fields: Field[] = $state([
    {
      id: 'first-name',
      label: $LL.ADD_CREDENTIALS.PROFILE.ADD.FIRST_NAME_LABEL(),
      placeholder: $LL.ADD_CREDENTIALS.PROFILE.ADD.FIRST_NAME_PLACEHOLDER(),
      value: '',
      required: true,
    },
    {
      id: 'middle-name',
      label: $LL.ADD_CREDENTIALS.PROFILE.ADD.MIDDLE_NAME_LABEL(),
      placeholder: $LL.ADD_CREDENTIALS.PROFILE.ADD.MIDDLE_NAME_PLACEHOLDER(),
      value: '',
    },
    {
      id: 'last-name',
      label: $LL.ADD_CREDENTIALS.PROFILE.ADD.LAST_NAME_LABEL(),
      placeholder: $LL.ADD_CREDENTIALS.PROFILE.ADD.LAST_NAME_PLACEHOLDER(),
      value: '',
      required: true,
    },
  ]);

  // TODO: do we even need this?
  function createProfile() {
    loading = true;
    // TODO: dispatch action
    setTimeout(() => {
      loading = false;
    }, 2000);
  }

  let valid = $derived(() => {
    return fields.every((field) => {
      if (field.required) {
        return field.value.length > 0;
      } else {
        return true;
      }
    });
  });

  onMount(() => {
    if (!profileName) {
      labelInput.focus();
    }
  });
</script>

<TopNavBar
  on:back={() => goto('/me/add')}
  title={$LL.ADD_CREDENTIALS.PROFILE.ADD.NAVBAR_TITLE()}
  class="sticky top-0 z-10"
/>

<!-- The 50px height of the TopNavBar are manually subtracted -->
<div class="relative flex h-[calc(100%_-_50px)] flex-col">
  <div class="flex grow flex-col items-center p-4 pt-0">
    <div class="my-5 flex h-[120px] flex-col items-center space-y-4">
      <!-- PaddedIcon -->
      <div class="flex h-[75px] w-[75px] items-center justify-center rounded-3xl bg-background-alt">
        <IdentificationBadgeRegularIcon class="dark:text-gray size-7 text-slate-800" />
      </div>
      <input
        type="text"
        class="w-full bg-background text-center text-[22px]/[30px] font-semibold tracking-tight text-slate-700 outline-none dark:text-grey"
        placeholder={$LL.ADD_CREDENTIALS.PROFILE.ADD.LABEL_PLACEHOLDER()}
        bind:this={labelInput}
      />
    </div>
    <div class="w-full space-y-4">
      {#each fields as field}
        <div class="flex flex-col gap-1">
          <div class="flex items-center justify-between">
            <label for={field.id} class="text-[14px]/[22px] font-medium text-slate-800 dark:text-grey">
              {field.label}
            </label>
            {#if field.required}
              <span class="text-[12px]/[14px] font-medium text-primary"
                >{$LL.ADD_CREDENTIALS.PROFILE.ADD.REQUIRED()}</span
              >
            {/if}
          </div>
          <input
            name={field.id}
            type="text"
            class="w-full rounded-xl border border-slate-300 bg-white px-4 py-3 text-[13px]/[24px] font-normal text-slate-800 disabled:cursor-not-allowed disabled:opacity-50 dark:border-slate-600 dark:bg-dark dark:text-slate-300 dark:caret-slate-300"
            placeholder={field.placeholder}
            bind:value={field.value}
          />
        </div>
      {/each}
    </div>
  </div>
</div>

<div class="absolute bottom-0 left-0 z-10 flex w-full flex-col gap-5 rounded-t-3xl bg-background-alt p-6 shadow">
  <Switch {checked} onCheckedChange={({ next }) => (checked = next)}>
    <div class="flex items-center gap-4 px-4">
      <HeartFillIcon class="size-5 text-primary" />
      <span class="text-[13px]/[24px] font-medium text-slate-800 dark:text-grey"
        >{$LL.ADD_CREDENTIALS.PROFILE.ADD.FAVORITES_TOGGLE_LABEL()}</span
      >
    </div>
  </Switch>
  <Button
    label={$LL.ADD_CREDENTIALS.PROFILE.ADD.CREATE_PROFILE_BUTTON()}
    disabled={!valid()}
    {loading}
    on:click={() => {
      createProfile();
    }}
  />
</div>
