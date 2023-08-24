<script>
  import { state } from '$src/stores';
  import { goto } from '$app/navigation';
  import Settings from '~icons/lucide/settings';
  import DotsThreeVertical from '~icons/ph/dots-three-vertical-bold';
  // import DotsThreeVertical from '~icons/ph/dots-three-outline-vertical-fill';
</script>

<div class="flex items-center justify-between bg-white px-[18px]">
  <div class="flex h-[44px] w-[44px] items-center justify-center rounded-2xl bg-indigo-500">
    <span class="text-[28px]/[28px]">
      {@html $state?.active_profile?.profile_picture ?? ''}
    </span>
  </div>
  <button
    on:click={() => goto('/me/settings')}
    class="-mx-4 rounded-full p-3 pr-0 text-black hover:bg-white hover:text-black"
  >
    <DotsThreeVertical class="h-8 w-8" />
  </button>
</div>
