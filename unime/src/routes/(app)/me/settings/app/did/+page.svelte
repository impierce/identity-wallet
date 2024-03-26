<script lang="ts">
  import { goto } from '$app/navigation';

  import SettingsEntry from '$lib/app/settings/SettingsEntry.svelte';
  import Button from '$lib/components/atoms/Button.svelte';
  import TopNavBar from '$lib/components/molecules/navigation/TopNavBar.svelte';
  import LL from '$src/i18n/i18n-svelte';
  import Switch from '$src/lib/components/atoms/Switch.svelte';

  import '@lottiefiles/lottie-player';

  import Check from '~icons/ph/check-bold';
  import Copy from '~icons/ph/copy-bold';

  interface Method {
    name?: string;
    method: string;
    did?: string;
    enabled: boolean;
  }

  const methods: Method[] = [
    {
      method: 'did:jwk',
      did: 'did:jwk:eyJjcnYiOiJQLTI1NiIsImt0eSI6IkVDIiwieCI6ImFjYklRaXVNczNpOF91c3pFakoydHBUdFJNNEVVM3l6OTFQSDZDZEgyVjAiLCJ5IjoiX0tjeUxqOXZXTXB0bm1LdG00NkdxRHo4d2Y3NEk1TEtncmwyR3pIM25TRSJ9',
      enabled: true,
    },
    { method: 'did:key', did: 'did:key:z6Mkk7yqnGF3YwTrLpqrW6PGsKci7dNqh1CjnvMbzrMerSeL', enabled: true },
    {
      name: 'Shimmer Testnet',
      did: 'did:iota:rms:0xcc26398894d5edb12e7c54ded26b86f3627a11c86bc06a7abe08597905109de1',
      method: 'did:iota:rms',
      enabled: false,
    },
    {
      name: 'IOTA Mainnet',
      method: 'did:iota',
      enabled: false,
    },
  ];
</script>

<TopNavBar on:back={() => history.back()} title={'Manage identities'} />
<div class="content-height flex flex-col bg-silver dark:bg-navy">
  <!-- Style 1: Cards -->
  <div class="space-y-8 p-4">
    {#each methods as method}
      <div class="rounded-xl bg-white p-4 dark:bg-dark">
        <div class="flex items-center justify-between">
          <p class="text-base font-semibold text-slate-800 dark:text-grey">{method.name ?? method.method}</p>
          <!-- <Switch active={method.enabled} /> -->
          {#if method.enabled}
            <Check class="h-6 w-6 text-primary" />
          {:else if method.did}
            <!-- <p class="text-primary text-[13px]/[24px] font-medium">Reactivate</p> -->
            <div class="-mr-1 -mt-1">
              <lottie-player
                src="/lottiefiles/Animation-1704188137306.json"
                autoplay
                loop
                speed={1.25}
                mode="normal"
                style="width: 32px"
              />
            </div>
          {:else}
            <p class="text-[13px]/[24px] font-medium text-primary">Set up</p>
          {/if}
        </div>
        {#if method.did}
          <div class="flex items-center justify-between space-x-4 pt-4">
            <p class="break-all text-[11px]/[14px] font-medium text-slate-500 dark:text-slate-300">{method.did}</p>
            <!-- <button class="bg-primary text-primary rounded-full bg-opacity-0 p-3 hover:bg-opacity-25"
              ><Copy class="h-5 w-5" /></button
            > -->
          </div>
        {/if}
      </div>
    {/each}
  </div>
  <!-- Style 2: Settings entries -->
  <hr />
  <div class="flex flex-col space-y-[10px] px-4 py-5">
    {#each methods as method}
      <SettingsEntry
        icon={undefined}
        title={method.name ?? method.method}
        textRight={'test'}
        hasCaretRight={true}
        on:click={() => goto('/me/settings/app/did')}
        todo={!method.enabled}
      >
        <!-- <Switch active={method.enabled} /> -->
      </SettingsEntry>
    {/each}
  </div>
  <!-- Style 3: Copy-able raw strings -->
  <hr />
  <div class="flex flex-col space-y-12 break-all px-8 py-8 text-[13px]/[18px] font-medium dark:text-slate-300">
    <div class="flex items-center justify-between space-x-2">
      <div>did:key:z6Mkk7yqnGF3YwTrLpqrW6PGsKci7dNqh1CjnvMbzrMerSeL</div>
      <button class="rounded-full bg-primary bg-opacity-10 p-3 text-primary"><Copy class="h-5 w-5" /></button>
    </div>
    <div class="flex items-center justify-between space-x-2">
      <div>did:web:localhost%3A52132</div>
      <button class="rounded-full bg-primary bg-opacity-10 p-3 text-primary"><Copy class="h-5 w-5" /></button>
    </div>
    <!-- <div>
      did:jwk:eyJjcnYiOiJQLTI1NiIsImt0eSI6IkVDIiwieCI6ImFjYklRaXVNczNpOF91c3pFakoydHBUdFJNNEVVM3l6OTFQSDZDZEgyVjAiLCJ5IjoiX0tjeUxqOXZXTXB0bm1LdG00NkdxRHo4d2Y3NEk1TEtncmwyR3pIM25TRSJ9
    </div>
    <div>did:iota:0xe4edef97da1257e83cbeb49159cfdd2da6ac971ac447f233f8439cf29376ebfe</div>
    <div>did:smr:0xcd606a482e58b783ba7d14a1c139028f0a249f1a338028e4b3c844f944e6493e</div> -->
    <!-- <button class="dark:bg-dark flex w-fit items-center rounded-lg p-4"><Copy class="mr-2" />Copy</button> -->
  </div>
  <div class="absolute bottom-8 w-full px-8">
    <Button label="Copy info"></Button>
  </div>
</div>

<style>
  .content-height {
    /* bottom-navigation: 64px, top-navigation: 50px */
    height: calc(100vh - var(--safe-area-inset-top) - var(--safe-area-inset-bottom) - 64px - 50px);
  }
</style>
