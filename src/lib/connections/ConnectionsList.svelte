<script lang="ts">
  import { goto } from '$app/navigation';

  import exampleConnections from '$lib/example/data/connections.json';
  import { state } from '$src/stores';

  import ChevronRight from '~icons/lucide/chevron-right';

  import type { Connection } from './types';
  import { groupConnectionsAlphabetically } from './utils';

  // let connections: Map<string, Connection[]> = groupConnectionsAlphabetically(exampleConnections);
  // // let connections: Map<string, Connection[]> = groupConnectionsAlphabetically($state.connections);
  // let connections: Map<string, Connection[]> = groupConnectionsAlphabetically([
  //   {
  //     client_name: 'NGDIL Demo',
  //     url: 'api.ngdil-demo.tanglelabs.io',
  //     logo_uri: 'https://recursing-feynman.weeir.com/imgs/kw1c-white.png',
  //     verified: false,
  //     first_connected: '2023-09-11T19:53:53.937981+00:00',
  //     last_connected: '2023-09-11T19:53:53.937981+00:00'
  //   },
  //   {
  //     url: 'amazon.com',
  //     client_name: 'Amazon',
  //     // logo_uri: '',
  //     verified: false,
  //     first_connected: '2023-03-02T11:53:53.937981+00:00',
  //     last_connected: '2023-04-05T10:01:53.937981+00:00'
  //   }
  // ]);
  let connections: Map<string, Connection[]> = groupConnectionsAlphabetically($state.connections);
  console.log(Object.fromEntries(connections));
</script>

<div class="flex h-full flex-col space-y-3">
  {#if connections.size === 0}
    <div class="flex h-full flex-col items-center justify-center">
      <p class="text-[14px]/[22px] font-medium text-slate-500 dark:text-white">
        No connections yet.
      </p>
    </div>
  {/if}
  {#each Object.entries(Object.fromEntries(connections)) as entry}
    <p class="w-full px-4 text-[14px]/[22px] font-medium text-slate-600 dark:text-white">
      {entry[0]}
    </p>
    {#each entry[1] as connection, index}
      <button on:click={() => goto(`/activity/connection/${index}`)}>
        <div class="flex h-16 items-center rounded-xl bg-white px-4 dark:bg-dark">
          <!-- Icon -->
          <div
            class="mr-4 flex h-8 w-8 overflow-hidden rounded-full border border-slate-300 dark:border-slate-600"
          >
            <img src={connection.logo_uri} />
          </div>
          <!-- Text -->
          <div class="flex grow flex-col items-start">
            <div class="text-[13px]/[24px] font-medium text-slate-800 dark:text-grey">
              {connection.url}
            </div>
            <div class="text-[12px]/[20px] font-medium text-slate-400 dark:text-slate-300">
              {new Date(connection.last_connected).toLocaleString('en-US', {
                dateStyle: 'long',
                timeStyle: 'medium'
              })}
            </div>
          </div>
        </div>
      </button>
    {/each}
  {/each}
</div>
