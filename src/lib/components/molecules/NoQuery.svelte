<script lang="ts">
  import PaddedIcon from '$lib/components/PaddedIcon.svelte';
  import LL from '$src/i18n/i18n-svelte';
  import MagnifyingGlass from '~icons/ph/magnifying-glass-fill';
</script>

<div class="flex grow flex-col items-center justify-center space-y-[15px] text-center">
  <PaddedIcon icon={MagnifyingGlass} />
  <p class="text-[22px]/[30px] font-semibold tracking-tight text-slate-700 dark:text-grey">
    <!--{$LL.EMPTY_CREDENTIALS_LIST_TITLE()}  -->
    What shall we search for?
  </p>

  <!-- TODO: uncomment when hard-coded link is removed -->
  <!--<p class="w-[240px] text-[13px]/[24px] text-slate-500 dark:text-slate-300">
      {$LL.EMPTY_CREDENTIALS_LIST_SUBTITLE()}
    </p> -->

  <!-- Message -->
  <p class="w-[280px] font-normal text-slate-400 dark:text-slate-300">
    Search for any of your credentials and badges here.
  </p>
</div>
