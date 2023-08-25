<script lang="ts">
  import LL from '$src/i18n/i18n-svelte';

  import PlusCircle from '~icons/ph/plus-circle';
</script>

<button
  class="flex h-12 items-center justify-center rounded-full bg-primary px-4 dark:bg-slate-800"
>
  <PlusCircle class="h-6 w-6 text-white" />
  <p class="pl-2 text-[13px]/[24px] font-medium text-white">{$LL.ADD()}</p>
</button>
