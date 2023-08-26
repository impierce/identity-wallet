<script lang="ts">
  import LL from '$src/i18n/i18n-svelte';

  import PlusCircle from '~icons/ph/plus-circle';
</script>

<button class="flex h-12 items-center justify-center rounded-full bg-bg-dark-primary px-4">
  <PlusCircle class="h-6 w-6 text-primary" />
  <p class="pl-2 text-[13px]/[24px] font-medium text-white">{$LL.ADD()}</p>
</button>
