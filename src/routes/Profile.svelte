<script lang="ts">
  import { state } from '../stores';
  import LL from '../i18n/i18n-svelte';
</script>

<div class="space-y-8 p-8">
  <h1 class="font-serif text-2xl font-semibold">
    {$LL.WELCOME()}, {$state?.active_profile?.display_name}!
  </h1>
  <p>{$LL.CREATE_IDENTITY_SUCCESS()}!</p>
  <div
    class="truncate rounded-lg bg-gray-300 px-4 py-2 font-mono text-sm font-semibold text-gray-600"
    data-testid="primary-did"
  >
    {$state?.active_profile?.primary_did}
  </div>
</div>
