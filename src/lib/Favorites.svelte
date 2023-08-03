<script lang="ts">
  import { CredentialListEntry } from '@impierce/ui-components';
  import Star from '~icons/lucide/star';
  import Clapperboard from '~icons/lucide/clapperboard';
</script>

<div class="">
  <div class="flex items-center">
    <p class="font-semibold text-slate-500">Favorites</p>
    <!-- <Star class="text-amber-500 ml-1" /> -->
  </div>
  <div class="pt-2">
    <CredentialListEntry title="Avatar: The Way of Water" description="Downtown Cinema">
      <span slot="icon"><Clapperboard class="h-6 w-6 text-violet-500" /></span>
    </CredentialListEntry>
  </div>
  <!-- <p class="text-sm font-medium text-slate-400">No favorites yet.</p> -->
</div>
