<script lang="ts">
  import { dispatch } from '$lib/dispatcher';
  import EmojiAvatarSelect from '$src/lib/app/settings/EmojiAvatarSelect.svelte';
  import TopNavBar from '$src/lib/components/molecules/navigation/TopNavBar.svelte';
  import { state } from '$src/stores';
</script>

<TopNavBar on:back={() => history.back()} title="Select display picture" />
<div class="h-full bg-silver p-4 dark:bg-navy">
  <div class="flex justify-center pt-12">
    <EmojiAvatarSelect
      selected={$state?.active_profile?.picture ?? undefined}
      on:change={(e) =>
        dispatch({
          type: '[Settings] Update profile',
          payload: { picture: e.detail },
        })}
    />
  </div>
</div>
