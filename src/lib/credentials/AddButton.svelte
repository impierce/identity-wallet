<script lang="ts">
  import PlusCircle from '~icons/ph/plus-circle';
  import LL from '$src/i18n/i18n-svelte';
</script>

<button class="flex justify-center rounded-full bg-indigo-500 px-4 h-12 items-center dark:bg-slate-800">
  <PlusCircle class="h-6 w-6 text-white" />
  <p class="pl-2 text-[13px]/[24px] font-medium text-white">{$LL.ADD()}</p>
</button>
