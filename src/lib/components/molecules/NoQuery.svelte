<script lang="ts">
  import PaddedIcon from '$lib/components/PaddedIcon.svelte';

  import MagnifyingGlass from '~icons/ph/magnifying-glass-fill';
</script>

<div class="flex grow flex-col items-center justify-center space-y-[15px] text-center">
  <PaddedIcon icon={MagnifyingGlass} />
  <p class="text-[22px]/[30px] font-semibold tracking-tight text-slate-800 dark:text-grey">What shall we search for?</p>
  <p class="w-[280px] font-normal text-slate-500 dark:text-slate-300">
    Search for any of your credentials and badges here.
  </p>
</div>
