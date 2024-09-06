<script lang="ts">
  import LL from '$i18n/i18n-svelte';
  import markdownit from 'markdown-it';

  import type { DisplayCredential } from '@bindings/credentials/DisplayCredential';

  export let credential: DisplayCredential;

  const md = markdownit();
</script>

<!-- Render `credentialSubject.achievement`. -->
<div class="flex flex-col gap-4">
  {#if credential.data.credentialSubject?.achievement?.description}
    <div class="prose prose-sm dark:prose-invert rounded-xl bg-background-alt p-4">
      <h2>{$LL.CREDENTIAL.DETAILS.DESCRIPTION()}</h2>
      {@html md.render(credential.data.credentialSubject.achievement.description)}
    </div>
  {/if}

  {#if credential.data.credentialSubject?.achievement?.criteria?.narrative}
    <div class="prose prose-sm dark:prose-invert rounded-xl bg-background-alt p-4">
      <h2>{$LL.CREDENTIAL.DETAILS.CONTENTS()}</h2>
      {@html md.render(credential.data.credentialSubject.achievement.criteria.narrative)}
    </div>
  {/if}
</div>
