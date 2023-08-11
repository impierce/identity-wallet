<script lang="ts">
  import LL from '$i18n/i18n-svelte';
  import PaddedIcon from '$lib/components/PaddedIcon.svelte';
  import Ghost from '~icons/ph/ghost-fill';
</script>

<div class="flex grow flex-col items-center justify-center text-center">
  <PaddedIcon icon={Ghost} />
  <div class="select-none p-6">
    <p class="pb-4 text-lg font-semibold text-slate-500">
      {$LL.EMPTY_CREDENTIALS_LIST_TITLE()}
    </p>
    <p class="text-slate-400">{$LL.EMPTY_CREDENTIALS_LIST_SUBTITLE()}</p>
  </div>
</div>
