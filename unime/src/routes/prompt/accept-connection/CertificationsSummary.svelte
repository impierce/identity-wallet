<script lang="ts">
  import LL from '$i18n/i18n-svelte';

  export let count: number;
</script>

<!--
@component
Collapsed stand-in for the certification cards, shown on a known connection.
### Props
- count
-->
<div class="w-full rounded-xl bg-white p-4 dark:bg-dark">
  <div class="flex flex-wrap justify-center gap-2">
    <span
      class="rounded-full bg-slate-100 px-3 py-1.5 text-[12px]/[20px] font-medium text-slate-700 dark:bg-slate-700 dark:text-grey"
    >
      {$LL.SCAN.CONNECTION_REQUEST.CERTIFICATION_COUNT({ count })}
    </span>
  </div>
</div>
