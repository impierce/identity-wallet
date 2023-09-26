<script lang="ts">
  import PaddedIcon from '$lib/components/PaddedIcon.svelte';
  import LL from '$src/i18n/i18n-svelte';

  import Ghost from '~icons/ph/ghost-fill';
</script>

<div class="flex grow flex-col items-center justify-center space-y-[15px] text-center">
  <PaddedIcon icon={Ghost} />
  <p class="text-[22px]/[30px] font-semibold tracking-tight text-slate-700 dark:text-grey">
    {$LL.EMPTY_CREDENTIALS_LIST_TITLE()}
  </p>
  <!-- TODO: uncomment when hard-coded link is removed -->
  <!-- <p class="w-[240px] text-[13px]/[24px] text-slate-500 dark:text-slate-300">
    {$LL.EMPTY_CREDENTIALS_LIST_SUBTITLE()}
  </p> -->
  <!-- TODO: remove hard-coded link to NGDIL demo -->
  <p class="w-[280px] font-normal text-slate-400 dark:text-slate-300">
    Visit <span class="font-medium text-primary">https://demo.ngdil.com</span> on a desktop computer
    to get started.
  </p>
</div>
