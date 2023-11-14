<script lang="ts">
  import TopNavigation from '$src/lib/components/molecules/navigation/TopNavigation.svelte';
  import EmojiAvatarSelect from '$src/lib/customize/EmojiAvatarSelect.svelte';
  import { dispatch } from '$src/lib/dispatcher';
  import { state } from '$src/stores';
</script>

<TopNavigation on:back={() => history.back()} title="Select display picture" />
<div class="h-full bg-silver p-4 dark:bg-navy">
  <div class="flex justify-center pt-12">
    <EmojiAvatarSelect
      selected={$state?.active_profile?.picture ?? undefined}
      on:change={(e) => dispatch({ type: '[Settings] Update profile', payload: { picture: e.detail } })}
    />
  </div>
</div>
