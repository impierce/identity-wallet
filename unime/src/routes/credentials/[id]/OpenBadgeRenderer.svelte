<script lang="ts">
  import LL from '$i18n/i18n-svelte';
  import markdownit from 'markdown-it';

  import type { DisplayCredential } from '@bindings/credentials/DisplayCredential';

  import TextFieldRenderer from './TextFieldRenderer.svelte';

  export let credential: DisplayCredential;

  const md = markdownit();
</script>

<!-- Render `credentialSubject.achievement`. -->
<div class="flex flex-col gap-4">
  {#if credential.data.credentialSubject?.achievement?.description}
    <div class="prose prose-sm rounded-xl bg-background p-4 dark:prose-invert">
      <h2>{$LL.CREDENTIAL.DETAILS.DESCRIPTION()}</h2>
      <!-- TODO Review marked vs. markdown-it and security risks. -->
      <!-- eslint-disable-next-line svelte/no-at-html-tags -->
      {@html md.render(credential.data.credentialSubject.achievement.description)}
    </div>
  {/if}

  {#if credential.data.credentialSubject?.achievement?.criteria?.narrative}
    <div class="prose prose-sm rounded-xl bg-background p-4 dark:prose-invert">
      <h2>{$LL.CREDENTIAL.DETAILS.CRITERIA()}</h2>
      <!-- TODO Review marked vs. markdown-it and security risks. -->
      <!-- eslint-disable-next-line svelte/no-at-html-tags -->
      {@html md.render(credential.data.credentialSubject.achievement.criteria.narrative)}
    </div>
  {/if}

  {#if credential.data.credentialSubject?.achievement?.alignment?.length > 0}
    <!-- Loop over the alignment array and render each item with a subtle dividing line -->
    {#each credential.data.credentialSubject.achievement.alignment as alignItem}
      <div class="prose prose-sm rounded-xl bg-background p-4 dark:prose-invert">
        <!-- Render targetName in bold -->
        <strong>{@html md.render(alignItem.targetName)}</strong>
        {@html md.render(alignItem.targetDescription)}
      </div>
    {/each}
  {/if}

  <TextFieldRenderer key={'Achievement type'} value={credential.data.credentialSubject?.achievement?.achievementType} />

  <!-- "validFrom" is defined as REQUIRED in JSON Schema: https://purl.imsglobal.org/spec/ob/v3p0/schema/json/ob_v3p0_achievementcredential_schema.json -->
  {#if credential.data.validFrom}
    <TextFieldRenderer key={'validFrom'} value={credential.data.validFrom} />
  {/if}

  <!-- TODO: Where should the linked image be rendered? Overlap the one during issuance?  -->
  <!-- <img src={credential.data.credentialSubject?.achievement?.image?.id} alt="achievement" /> -->
</div>
