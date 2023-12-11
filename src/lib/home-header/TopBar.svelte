<script lang="ts">
  import { goto } from '$app/navigation';

  import { state } from '$src/stores';

  import MagnifyingGlass from '~icons/ph/magnifying-glass';
</script>

<div class="flex items-center justify-between">
  <button
    class="flex h-11 w-11 items-center justify-center rounded-2xl bg-primary"
    on:click={() => goto('/me/settings')}
  >
    <span class="text-[28px]/[28px]">
      {@html $state?.active_profile?.picture ?? ''}
    </span>
  </button>
  <button
    on:click={() => goto('/me')}
    class="-mr-3 flex h-11 w-11 items-center justify-center rounded-2xl text-black dark:text-white"
  >
    <MagnifyingGlass class="h-6 w-6" />
  </button>
</div>
