<script lang="ts">
  import { state } from '../stores';
  import LL, { locale } from '../i18n/i18n-svelte';
</script>

<div class="grid place-items-center space-y-8 p-8">
  <h1 class="font-serif text-2xl font-semibold">
    {$LL.WELCOME()}, {$state?.active_profile?.display_name}!
  </h1>
  <p>You have just created a new digital identity!</p>
  <div class="w-fit rounded-lg bg-gray-300 px-4 py-2 font-mono text-sm font-semibold text-gray-600">
    {$state?.active_profile?.primary_did}
  </div>
</div>
