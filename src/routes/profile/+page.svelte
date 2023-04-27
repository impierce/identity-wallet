<script lang="ts">
  import { state } from '../../stores';
  import LL from '../../i18n/i18n-svelte';
  import { state } from '../stores';
  import LL from '../i18n/i18n-svelte';
  import { Button } from '@impierce/ui-components';
  import { dispatch } from '../lib/dispatcher';
  import { useFocus } from 'svelte-navigator';

  const getRequest = async () =>
    // Dispatches a mock request.
    dispatch({ type: '[Authenticate] Get request', payload: { request_url: "siopv2://idtoken?claims=%7B%22id_token%22%3A%7B%22email%22%3Anull%2C%22name%22%3Anull%7D%2C%22user_claims%22%3Anull%7D&client_id=did%3Akey%3A1&nonce=n-0S6_WzA2Mj&redirect_uri=https%3A%2F%2Fclient.example.org%2Fcb&registration=%7B%22id_token_signing_alg_values_supported%22%3Anull%2C%22subject_syntax_types_supported%22%3A%5B%22did%3Akey%22%5D%7D&response_mode=post&response_type=id_token&scope=openid" } });

  const sendResponse = async () =>
    // Dispatches a mock request.
    dispatch({ type: '[Authenticate] Send response', payload: { user_claims: claims } });

  const registerFocus = useFocus();

  let claims = new Map<string, string>();
  let values = {};

  function updateMap() {
    // Clear the map and add each key-value pair from the object
    claims.clear();
    Object.entries($state?.active_requested_claims).forEach(([key, value]) => {
      if (values[key]) {
        claims.set(key, values[key]);
      }
    });
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
  {#if !$state?.active_requested_claims}
    <Button label={$LL.SCAN_QRCODE()} on:clicked={getRequest} />
  {:else}
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
            use:registerFocus
          />
        </div>
      {/each}
    </div>
    <Button label={$LL.AUTHENTICATE()} on:clicked={sendResponse} />
  {/if}
</div>
