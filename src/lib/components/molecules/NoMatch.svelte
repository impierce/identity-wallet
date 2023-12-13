<script lang="ts">
  import PaddedIcon from '$lib/components/PaddedIcon.svelte';

  import Ghost from '~icons/ph/ghost-fill';
</script>

<div class="flex grow flex-col items-center justify-center space-y-[15px] text-center">
  <PaddedIcon icon={Ghost} />
  <p class="text-[22px]/[30px] font-semibold tracking-tight text-slate-800 dark:text-grey">No results found</p>
  <p class="w-[280px] font-normal text-slate-500 dark:text-slate-300">Try searching for something else.</p>
</div>
