<script lang="ts">
  import type { InitialConnection } from './types';

  export let data: InitialConnection;
</script>

<div class="flex flex-col bg-blue-100 px-4 py-2 rounded">
    <span class="text-blue-500 text-sm">You have connected with <p class="font-semibold">{data.issuer.domain}</p> for the first time!</span>
    <span class="text-blue-500">{data.issuer.did}</span>
</div>
