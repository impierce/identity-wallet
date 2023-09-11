<script lang="ts">
  import type { DisplayCredential } from 'src-tauri/bindings/display-credential/DisplayCredential';

  import type { Connection } from '$lib/connections/types';
  import { state } from '$src/stores';

  import EnvelopeSimple from '~icons/ph/envelope-simple-light';

  import CredentialListEntry from '../components/CredentialListEntry.svelte';
  import { colors } from '../credentials/customization/utils';

  export let connection: Connection;
  export let title: string;
  export let timestamp: string;
  export let credentials: DisplayCredential[];
</script>

<div class="flex flex-col">
  <p class="text-[14px]/[22px] font-medium text-slate-800">{title}</p>
  <p class="text-[12px]/[20px] font-medium text-slate-500">
    {new Date(timestamp).toLocaleString('en-US', {
      dateStyle: 'medium',
      timeStyle: 'medium'
    })}
  </p>
  {#if credentials.length > 0}
    <div class="mt-[12px] w-fit rounded-xl border border-slate-200 dark:border-slate-600">
      {#each credentials as credential}
        <CredentialListEntry
          title={credential.metadata.display.name ?? credential.data.type.at(-1)}
          color={colors.at(0)}
        >
          <!-- <span slot="icon"><EnvelopeSimple class="h-6 w-6" /></span> -->
        </CredentialListEntry>
      {/each}
    </div>
  {/if}
</div>
