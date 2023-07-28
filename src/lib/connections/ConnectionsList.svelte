<script lang="ts">
  import { goto } from '$app/navigation';
  import { TopNavigation } from '@impierce/ui-components';
  import ChevronRight from '~icons/lucide/chevron-right';

  let mock_connections = [
    {
      id: '0',
      name: 'amazon.com',
      lastConnected: '2 days ago'
    },
    {
      id: '1',
      name: 'bestdex.com',
      lastConnected: '1 hour ago'
    },
    {
      id: '2',
      name: 'groceries-online.co.uk',
      lastConnected: '2 weeks ago'
    }
  ];
</script>

<div class="flex flex-col space-y-4">
  {#each mock_connections as mock_connection}
    <button on:click={() => goto(`/activity/connection/${mock_connection.id}`)}>
      <div class="flex items-center justify-between rounded bg-slate-100 px-4 py-2">
        <div class="flex flex-col items-start">
          <div class="font-semibold text-slate-600">{mock_connection.name}</div>
          <div class="text-sm font-light italic text-slate-400">
            {mock_connection.lastConnected}
          </div>
        </div>
        <ChevronRight />
      </div>
    </button>
  {/each}
</div>
