<script lang="ts">
  import type { SvelteComponent } from 'svelte';

  import type { SvelteHTMLElements } from 'svelte/elements';

  import { TopNavBar } from '$lib/components';
  import { dispatch } from '$lib/dispatcher';
  import { CheckCircleFillIcon } from '$lib/icons';
  import { state } from '$lib/stores';

  $: preferred_did_method = $state.profile_settings.preferred_did_methods.at(0);

  interface Method {
    alias?: string;
    method: string;
    did?: string;
    enabled: boolean;
    logo?: typeof SvelteComponent<SvelteHTMLElements['svg']>;
  }

  const methods: Method[] = [
    {
      method: 'did:jwk',
      did: $state.dids['did:jwk'],
      enabled: true,
    },
    {
      method: `did:key`,
      did: $state.dids['did:key'],
      enabled: true,
    },
  ];

  // Capabilities of `did-manager`
  const verifiableMethods: string[] = ['did:iota', 'did:jwk', 'did:key', 'did:web'];

  const handleClick = (method: Method) => {
    if (method.did) {
      dispatch({ type: '[DID] Set preferred method', payload: { method: method.method } });
    } else {
      // TODO: start the setup process for the method
    }
  };
</script>

<TopNavBar on:back={() => history.back()} title={'Manage identities'} class="sticky top-0 z-10" />

<div class="bg-silver dark:bg-navy flex flex-col space-y-[15px] px-4 py-5">
  <!-- Produce -->
  <div class="flex flex-col space-y-[10px]">
    <p class="text-[14px]/[22px] font-medium text-slate-500 dark:text-slate-300">Produce</p>
    <div class="flex flex-col space-y-4">
      {#each methods as method (method.method)}
        <button
          class={`dark:bg-dark rounded-xl border bg-white p-4 disabled:opacity-30 ${method.method === preferred_did_method ? 'border-primary ring-primary ring-1' : 'border-slate-200 dark:border-slate-600'}`}
          on:click={() => handleClick(method)}
          disabled={!method.enabled}
        >
          <div class="flex h-7 items-center justify-between">
            <div class="flex items-center">
              {#if method.logo}
                <svelte:component this={method.logo} class="mr-3 h-6 w-6" />
              {/if}
              <p class="dark:text-grey text-base font-semibold text-slate-800">{method.alias ?? method.method}</p>
              <div class="ml-2 flex items-center space-x-1 rounded-full bg-slate-200 px-2 py-1 dark:bg-slate-600">
                <p class="text-[12px]/[20px] font-medium text-slate-400 dark:text-slate-300">
                  {$state.profile_settings.preferred_key_types.at(0)}
                </p>
              </div>
            </div>
            {#if method.method === preferred_did_method}
              <div class="bg-ex-blue-2 dark:bg-primary flex items-center space-x-1 rounded-full px-2 py-1">
                <p class="text-secondary dark:text-dark text-[12px]/[20px] font-medium">preferred</p>
              </div>
            {/if}
            {#if !method.did}
              <!-- TODO: Clean up. Button inside button not permitted. Changed to `div`. -->
              <div class="text-primary -mr-4 px-4 py-2 text-[13px]/[24px] font-medium disabled:text-slate-300">
                <span class="flex items-center">Set up</span>
              </div>
            {/if}
          </div>
          {#if method.did}
            <div class="flex items-center justify-between space-x-4 pt-4">
              <p
                class="break-all text-left font-mono text-[11px]/[14px] font-medium text-slate-500 dark:text-slate-300"
              >
                {method.did}
              </p>
            </div>
          {/if}
        </button>
      {/each}
    </div>
  </div>

  <!-- Verify -->
  <div class="flex flex-col space-y-[10px]">
    <p class="text-[14px]/[22px] font-medium text-slate-500 dark:text-slate-300">Verify</p>
    <div class="flex flex-wrap gap-2">
      {#each verifiableMethods as method}
        <div class="bg-ex-blue-2 dark:bg-primary flex items-center space-x-1 rounded-full px-2 py-1">
          <CheckCircleFillIcon class="text-primary dark:text-navy h-4 w-4" />
          <p class="text-secondary dark:text-dark text-[12px]/[20px] font-medium">
            {method}
          </p>
        </div>
      {/each}
    </div>
  </div>
</div>
