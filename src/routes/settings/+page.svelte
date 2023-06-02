<script lang="ts">
  import { goto } from '$app/navigation';
  import { BottomNavigation } from '@impierce/ui-components';
  import { Link } from 'svelte-heros-v2';
  import { state } from '../../stores';
</script>

<div class="min-h-screen">
  <div class="flex p-8">
    <div class="space-y-4">
      <h2 class="font-semibold">Your DIDs</h2>
      <div class="flex space-x-2">
        <div
          class="break-all rounded-lg bg-slate-200 px-4 py-2 font-mono text-sm font-medium text-slate-700"
          data-testid="primary-did"
        >
          {$state?.active_profile?.primary_did}
        </div>
        <button class="rounded-lg bg-slate-200 p-4"
          ><Link class="text-violet-700" strokeWidth="2" /></button
        >
      </div>
    </div>
  </div>
  <div class="sticky top-[100vh]">
    <BottomNavigation active="settings" on:profile={() => goto('/profile')} />
  </div>
</div>
