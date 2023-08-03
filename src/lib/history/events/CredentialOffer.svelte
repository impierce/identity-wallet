<script lang="ts">
  import type { CredentialOffer } from './types';

  export let data: CredentialOffer;
</script>

<div class="flex flex-col rounded bg-blue-100 px-4 py-2">
  <span class="text-sm text-blue-500"
    >You have received {data.credentials.length} credentials from
    <p class="font-semibold">{data.issuer.id}</p>
    !</span
  >
</div>
