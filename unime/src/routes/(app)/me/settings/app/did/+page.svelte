<script lang="ts">
  import type { SvelteComponent } from 'svelte';

  import type { SvelteHTMLElements } from 'svelte/elements';

  import TopNavBar from '$lib/components/molecules/navigation/TopNavBar.svelte';
  import { dispatch } from '$lib/dispatcher';
  import IotaLogo from '$lib/static/svg/Iota.svelte';
  import ShimmerLogo from '$lib/static/svg/Shimmer.svelte';
  import { state } from '$lib/stores';

  import CheckCircle from '~icons/ph/check-circle-fill';
  import Code from '~icons/ph/code';

  $: preferred_did_method = $state.profile_settings.preferred_did_method;

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
    // TODO: enable IOTA-based methods once this is implemented: https://github.com/impierce/did-manager/issues/7
    // {
    //   alias: 'Shimmer Testnet',
    //   method: 'did:iota:rms',
    //   did: $state.dids['did:iota:rms'],
    //   enabled: false,
    //   logo: ShimmerLogo,
    // },
    // {
    //   alias: 'Shimmer',
    //   method: 'did:iota:smr',
    //   did: $state.dids['did:iota:smr'],
    //   enabled: false,
    //   logo: ShimmerLogo,
    // },
    // {
    //   alias: 'IOTA',
    //   method: 'did:iota',
    //   did: $state.dids['did:iota'],
    //   enabled: false,
    //   logo: IotaLogo,
    // },
    // {
    //   alias: 'Custom',
    //   method: '',
    //   enabled: false,
    //   logo: Code,
    // },
  ];

  // Capabilities of `did-manager`
  const verifiableMethods: string[] = ['did:jwk', 'did:key', 'did:web', 'did:iota', 'did:iota:rms', 'did:iota:smr'];

  const handleClick = (method: Method) => {
    if (method.did) {
      dispatch({ type: '[DID] Set preferred method', payload: { method: method.method } });
    } else {
      // TODO: start the setup process for the method
    }
  };
</script>

<TopNavBar on:back={() => history.back()} title={'Manage identities'} />
<div class="content-height flex flex-col bg-silver dark:bg-navy">
  <div class="space-y-[15px] px-4 py-5">
    <!-- Produce -->
    <div class="flex flex-col space-y-[10px]">
      <p class="text-[14px]/[22px] font-medium text-slate-500 dark:text-slate-300">Produce</p>
      <div class="flex flex-col space-y-4">
        {#each methods as method (method.method)}
          <button
            class={`rounded-xl border bg-white p-4 disabled:opacity-30 dark:bg-dark ${method.method === preferred_did_method ? 'border-primary ring-1 ring-primary' : 'border-slate-200 dark:border-slate-600'}`}
            on:click={() => handleClick(method)}
            disabled={!method.enabled}
          >
            <div class="flex h-7 items-center justify-between">
              <div class="flex items-center">
                {#if method.logo}
                  <svelte:component this={method.logo} class="mr-3 h-6 w-6" />
                {/if}
                <p class="text-base font-semibold text-slate-800 dark:text-grey">{method.alias ?? method.method}</p>
              </div>
              {#if method.method === preferred_did_method}
                <div class="flex items-center space-x-1 rounded-full bg-ex-blue-2 px-2 py-1 dark:bg-primary">
                  <p class="text-[12px]/[20px] font-medium text-teal dark:text-dark">preferred</p>
                </div>
              {/if}
              {#if !method.did}
                <button class="-mr-4 px-4 py-2 text-[13px]/[24px] font-medium text-primary disabled:text-slate-300">
                  <span class="flex items-center">Set up</span>
                </button>
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
          <div class="flex items-center space-x-1 rounded-full bg-ex-blue-2 px-2 py-1 dark:bg-primary">
            <CheckCircle class="h-4 w-4 text-primary dark:text-navy" />
            <p class="text-[12px]/[20px] font-medium text-teal dark:text-dark">
              {method}
            </p>
          </div>
        {/each}
      </div>
    </div>
  </div>
</div>

<style>
  .content-height {
    /* bottom-navigation: 64px, top-navigation: 50px */
    height: calc(100vh - var(--safe-area-inset-top) - var(--safe-area-inset-bottom) - 64px - 50px);
  }
</style>
