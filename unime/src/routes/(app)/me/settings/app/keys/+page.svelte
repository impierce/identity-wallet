<script lang="ts">
  import TopNavBar from '$lib/components/molecules/navigation/TopNavBar.svelte';
  import { dispatch } from '$lib/dispatcher';
  import { state } from '$lib/stores';

  import Info from '~icons/ph/info';

  $: preferred_key_type = $state.profile_settings.preferred_key_types.at(0);

  type KeyType = 'EdDSA' | 'ES256' | 'ES256K';

  interface Key {
    type: KeyType;
    alias?: string;
    key_id: string; // Corresponds to the `JwkStorage` in `did-manager`
    enabled: boolean;
  }

  const keys: Key[] = [
    {
      type: 'EdDSA',
      key_id: 'ed25519-0',
      enabled: true,
    },
    {
      type: 'ES256',
      key_id: 'es256-0',
      enabled: true,
    },
    {
      type: 'ES256K',
      key_id: 'es256k-0',
      enabled: false, // TODO: enable once `identity_manager.subject.identifier` accepts `ES256K`
    },
  ];
</script>

<TopNavBar on:back={() => history.back()} title={'Manage keys'} />
<div class="content-height flex flex-col bg-silver dark:bg-navy">
  <div class="space-y-[15px] px-4 py-5">
    <div class="flex flex-col space-y-[10px]">
      <p class="text-[14px]/[22px] font-medium text-slate-500 dark:text-slate-300">Available keys</p>
      {#each keys as key}
        <button
          class={`rounded-xl border bg-white p-4 disabled:opacity-30 dark:bg-dark ${key.type === preferred_key_type ? 'border-primary ring-1 ring-primary' : 'border-slate-200 dark:border-slate-600'}`}
          on:click={() => dispatch({ type: '[Keys] Set preferred key type', payload: { key_type: key.type } })}
          disabled={!key.enabled}
        >
          <div class="flex h-7 items-center justify-between">
            <div class="flex items-center">
              <p class="text-base font-semibold text-slate-800 dark:text-grey">{key.type}</p>
            </div>
            {#if key.type === preferred_key_type}
              <div class="flex items-center space-x-1 rounded-full bg-ex-blue-2 px-2 py-1 dark:bg-primary">
                <p class="text-[12px]/[20px] font-medium text-teal dark:text-dark">preferred</p>
              </div>
            {/if}
          </div>
          {#if key.key_id}
            <div class="flex items-center justify-between space-x-4 pt-4">
              <p
                class="break-all text-left font-mono text-[11px]/[14px] font-medium text-slate-500 dark:text-slate-300"
              >
                {key.key_id}
              </p>
            </div>
          {/if}
        </button>
        <!-- <button class="rounded-xl border bg-white p-4 disabled:opacity-30 dark:bg-dark border-slate-200 dark:border-slate-600">
            <div class="flex h-7 items-center justify-between">
                <p class="text-[14px -->
      {/each}
    </div>

    <div class="flex w-full items-center rounded-lg bg-white px-4 py-4 dark:bg-dark">
      <span class="mr-4 h-6 w-6">
        <Info class="h-6 w-6 text-primary" />
      </span>
      <div class="flex flex-col">
        <p class="text-[13px]/[24px] font-medium text-slate-800 dark:text-grey">Developer info</p>
        <ul class="ml-3 list-disc text-[12px]/[20px] font-medium text-slate-500 dark:text-slate-300">
          <li>All keys are generated once on profile creation.</li>
          <li>Only one key per type is currently supported.</li>
          <li>
            UniMe will automatically select the key type based on the server capabilities, but respect your preference
            if there's multiple matches.
          </li>
        </ul>
      </div>
    </div>
  </div>
</div>
