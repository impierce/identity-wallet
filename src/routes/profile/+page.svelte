<script lang="ts">
  import { state } from '../../stores';
  import LL from '../../i18n/i18n-svelte';
  import { Button } from '@impierce/ui-components';
  import { dispatch } from '../../lib/dispatcher';

  const sendResponse = async () =>
    dispatch({ type: '[Authenticate] Send response', payload: { user_claims: claims } });

  let claims = new Map<string, string>();
  let values: { [key: string]: any } = {};
  
  function updateMap() {
    // Clear the map and add each key-value pair from the object
    claims.clear();
    const requested_claims = $state?.active_requested_claims;
    for (const key in requested_claims) {
      const claim = requested_claims[key];
      if (values[key]) {
        claims.set(key, values[key]);
      }
      claims.set(key, claim);
    }

  }
</script>

<div class="space-y-8 p-8">
  <h1 class="font-serif text-2xl font-semibold">
    {$LL.WELCOME()}, {$state?.active_profile?.display_name}!
  </h1>
  <p>{$LL.CREATE_IDENTITY_SUCCESS()}!</p>
  <div
    class="truncate rounded-lg bg-gray-300 px-4 py-2 font-mono text-sm font-semibold text-gray-600"
    data-testid="primary-did"
  >
    {$state?.active_profile?.primary_did}
  </div>

  {#if $state?.active_requested_claims}
    <div
      class="truncate rounded-lg bg-gray-300 px-4 py-2 font-mono text-sm font-semibold text-gray-600"
      data-testid="claims"
    >
      <p data-testid="label-prompt-username" class="text-gray-600">{$LL.ENTER_CLAIMS()}</p>
  <!-- TODO: replace with ui-components/Input -->

      {#each Object.entries($state?.active_requested_claims) as [key, value]}
        <p>{key}: {value}</p>
        <div>
          <input
            type="text"
            data-testid="input-username"
            class="w-full rounded-lg border px-4 py-2 shadow focus:outline-none focus:ring-2 focus:ring-violet-600"
            placeholder=""
            bind:value={values[key]} on:input={updateMap}
          />
        </div>
      {/each}
    </div>
    <Button label={$LL.AUTHENTICATE()} on:clicked={sendResponse} />
  {/if}

</div>