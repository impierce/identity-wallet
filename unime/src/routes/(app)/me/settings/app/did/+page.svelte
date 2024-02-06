<script>
  import TopNavBar from '$lib/components/molecules/navigation/TopNavBar.svelte';
  import LL from '$src/i18n/i18n-svelte';
  import Button from '$src/lib/components/atoms/Button.svelte';

  import Copy from '~icons/ph/copy-fill';
</script>

<TopNavBar on:back={() => history.back()} title={'Manage identities'} />
<div class="content-height bg-silver dark:bg-navy flex flex-col">
  <div class="flex flex-col space-y-12 break-all px-8 py-8 text-[13px]/[18px] font-medium dark:text-slate-300">
    <div>did:key:z6Mkk7yqnGF3YwTrLpqrW6PGsKci7dNqh1CjnvMbzrMerSeL</div>
    <div>did:web:localhost%3A52132</div>
    <div>
      did:jwk:eyJjcnYiOiJQLTI1NiIsImt0eSI6IkVDIiwieCI6ImFjYklRaXVNczNpOF91c3pFakoydHBUdFJNNEVVM3l6OTFQSDZDZEgyVjAiLCJ5IjoiX0tjeUxqOXZXTXB0bm1LdG00NkdxRHo4d2Y3NEk1TEtncmwyR3pIM25TRSJ9
    </div>
    <div>did:iota:0xe4edef97da1257e83cbeb49159cfdd2da6ac971ac447f233f8439cf29376ebfe</div>
    <div>did:smr:0xcd606a482e58b783ba7d14a1c139028f0a249f1a338028e4b3c844f944e6493e</div>
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
