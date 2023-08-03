<script lang="ts">
  import type { InitialConnection } from './types';

  export let data: InitialConnection;
</script>

<div class="flex flex-col rounded bg-blue-100 p-4">
  <span class="text-start text-sm text-blue-500"
    >You have connected with <span class="font-semibold">{data.issuer.domain}</span> for the first time!</span
  >
  <!-- <span class="text-blue-500">{data.issuer.did}</span> -->
</div>
