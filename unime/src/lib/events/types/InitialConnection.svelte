<script lang="ts">
  import type { InitialConnection } from '.';

  export let data: InitialConnection;
</script>

<div class="bg-blue-100 flex flex-col rounded p-4">
  <span class="text-blue-500 text-start text-sm"
    >You have connected with <span class="font-semibold">{data.issuer.domain}</span> for the first time!</span
  >
  <!-- <span class="text-blue-500">{data.issuer.did}</span> -->
</div>
