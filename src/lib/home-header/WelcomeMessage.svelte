<script lang="ts">
  import { state } from '../../stores';
  import LL from '$i18n/i18n-svelte';
</script>

<p class="text-3xl font-semibold text-black">
    {$LL.WELCOME()}, <span class="text-indigo-500">{$state?.active_profile?.display_name}.</span>
</p>
