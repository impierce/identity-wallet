<script lang="ts">
  import PlusCircle from '~icons/ph/plus-circle';
  import LL from '$i18n/i18n-svelte';
</script>

<button class="flex justify-center rounded-full bg-indigo-500 p-3 dark:bg-slate-800">
  <PlusCircle class="h-6 w-6 text-white" />
  <p class="pl-2 pr-1 text-base font-medium text-white">{$LL.ADD()}</p>
</button>
