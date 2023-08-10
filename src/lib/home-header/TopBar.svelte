<script>
  import { Avatar, ProgressBar } from '@impierce/ui-components';
  import { state } from '../../stores';
  import { goto } from '$app/navigation';
  import Settings from '~icons/lucide/settings';
  import DotsThreeVertical from '~icons/ph/dots-three-vertical-bold';

  // import DotsThreeVertical from '~icons/ph/dots-three-outline-vertical-fill';
</script>

<div class="flex items-center justify-between bg-white p-6">
  <!-- <Avatar size="small" initials="FR" /> -->
  <div class="h-12 w-12 rounded-2xl bg-indigo-600 p-2 ">
    <div class="flex h-full w-full items-center justify-center font-medium text-neutral-100">
      <!-- TODO: placeholder for avatar or emoji -->
    </div>
  </div>
  <button
    on:click={() => goto('/me/settings')}
    class="-mx-4 rounded-full p-4 text-black hover:bg-white hover:text-black"
  >
    <DotsThreeVertical class="h-8 w-8" />
    <!-- <Cog class="h-6 w-6" /> -->
  </button>
</div>
<!-- <div class="h-6 bg-gradient-to-b from-white to-transparent">

</div> -->

