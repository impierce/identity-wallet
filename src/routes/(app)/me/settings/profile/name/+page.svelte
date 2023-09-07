<script lang="ts">
  import { fade } from 'svelte/transition';

  import { TopNavigation } from '@impierce/ui-components';

  import Button from '$src/lib/components/Button.svelte';
  import { dispatch } from '$src/lib/dispatcher';
  import { state } from '$src/stores';

  let name = $state.active_profile?.name;
</script>

<TopNavigation title="Change profile name" on:back={() => history.back()} />
<div class="grow bg-silver p-4 dark:bg-navy">
  <input
    class="w-full rounded-xl border border-slate-300 bg-white px-4 py-3 text-[13px]/[24px] font-normal"
    placeholder="Enter a profile name"
    bind:value={name}
  />
</div>
<div class="rounded-t-3xl bg-white p-6">
  <Button
    label="Update"
    on:click={() => dispatch({ type: '[Settings] Update profile', payload: { name } })}
    disabled={!!!name}
  />
</div>
