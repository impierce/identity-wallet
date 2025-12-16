<script lang="ts">
  import LL from '$i18n/i18n-svelte';

  //import markdownit from 'markdown-it';

  import type { DisplayCredential } from '@bindings/credentials/DisplayCredential';

  import CollapsibleRenderer from '$lib/components/CollapsibleRenderer.svelte';

  import TextFieldRenderer from './TextFieldRenderer.svelte';

  export let credential: DisplayCredential;

  //const md = markdownit();
</script>

<div class="flex flex-col gap-4">
  <!-- Achievement -->
  {#if credential.data.credentialSubject?.achievement?.description}
    <div class="rounded-xl bg-background p-3">
      <CollapsibleRenderer
        items={[
          {
            id: 'description',
            title: $LL.CREDENTIAL.DETAILS.DESCRIPTION(),
            description: credential.data.credentialSubject.achievement.description,
          },
        ]}
      />
    </div>
    <!-- TODO: Review marked vs. markdown-it and security risks. -->
    <!-- eslint-disable-next-line svelte/no-at-html-tags -->
  {/if}

  {#if credential.data.credentialSubject?.achievement?.criteria?.narrative}
    <div class="rounded-xl bg-background p-3">
      <CollapsibleRenderer
        items={[
          {
            id: 'criteria',
            title: $LL.CREDENTIAL.DETAILS.OPEN_BADGES.CRITERIA(),
            description: credential.data.credentialSubject.achievement.criteria.narrative,
          },
        ]}
      />
      <!-- TODO: Review marked vs. markdown-it and security risks. -->
      <!-- eslint-disable-next-line svelte/no-at-html-tags -->
    </div>
  {/if}

  {#if credential.data.credentialSubject?.achievement?.achievementType}
    <TextFieldRenderer
      key={'Achievement type'}
      value={credential.data.credentialSubject?.achievement?.achievementType}
    />
  {/if}

  <!-- {#if credential.data.credentialSubject?.achievement?.alignment?.length > 0}
    <div class="prose prose-sm rounded-xl bg-background p-4 dark:prose-invert">
      <h2>{$LL.CREDENTIAL.DETAILS.OPEN_BADGES.ALIGNMENT()}</h2>
      {#each credential.data.credentialSubject.achievement.alignment as alignmentItem}
        <h4>{alignmentItem.targetName}</h4>
        {#if alignmentItem.targetDescription} -->
  <!-- TODO Review marked vs. markdown-it and security risks. -->
  <!-- eslint-disable-next-line svelte/no-at-html-tags -->
  <!--   {@html md.render(alignmentItem.targetDescription)}
        {/if}
      {/each}
    </div>
  {/if} -->

  {#if credential.data.credentialSubject?.achievement?.alignment?.length > 0}
    <div class="rounded-xl bg-background p-3">
      <CollapsibleRenderer
        items={credential.data.credentialSubject.achievement.alignment.map((item, index) => ({
          id: `alignment-${index}`,
          title: $LL.CREDENTIAL.DETAILS.OPEN_BADGES.ALIGNMENT(),
          description: `#### ${item.targetName}\n\n${item.targetDescription ?? ''}`,
        }))}
      />
    </div>
  {/if}

  <!-- Result -->
  {#if credential.data.credentialSubject?.result?.length > 0}
    <div class="rounded-xl bg-background p-3">
      <CollapsibleRenderer
        items={credential.data.credentialSubject.result.map((resultItem, index) => {
          let description = '';

          if (resultItem.alignment?.length) {
            description += resultItem.alignment
              .map((a) => `#### ${a.targetName}\n\n${a.targetDescription ?? ''}`)
              .join('\n\n');
          }

          if (resultItem.value) {
            description += `\n\n#### ${$LL.CREDENTIAL.DETAILS.OPEN_BADGES.VALUE()}\n\n${resultItem.value}`;
          }

          if (resultItem.resultDescription) {
            description += `\n\n${resultItem.resultDescription}`;
          }

          return {
            id: `result-${index}`,
            title: $LL.CREDENTIAL.DETAILS.OPEN_BADGES.RESULT(),
            description,
          };
        })}
      />
    </div>
  {/if}

  <!-- Result 
  {#if credential.data.credentialSubject?.result?.length > 0}
    <div class="prose prose-sm rounded-xl bg-background p-4 dark:prose-invert">
      <h2>{$LL.CREDENTIAL.DETAILS.OPEN_BADGES.RESULT()}</h2>

      <div class="flex flex-col divide-y divide-slate-300">
        {#each credential.data.credentialSubject.result as resultItem}
          <div class="py-4 first:pt-0 last:pb-0">
            {#if resultItem.alignment?.length > 0}
              {#each resultItem.alignment as resultAlignment}
                <h4>{resultAlignment.targetName}</h4>
                {#if resultAlignment.targetDescription} -->
  <!-- TODO: Review marked vs. markdown-it and security risks. -->
  <!-- eslint-disable-next-line svelte/no-at-html-tags -->
  <!--          {@html md.render(resultAlignment.targetDescription)}
                {/if}
              {/each}
            {/if}

            {#if resultItem.value}
              <div class="flex h-16 items-center justify-between">
                <h4 class="mt-2">{$LL.CREDENTIAL.DETAILS.OPEN_BADGES.VALUE()}</h4>
                <div class="text-2xl font-bold">  -->
  <!-- TODO: Review marked vs. markdown-it and security risks. -->
  <!-- eslint-disable-next-line svelte/no-at-html-tags -->
  <!--     {@html md.render(resultItem.value)}
                </div>
              </div>
            {/if}

            {#if resultItem.resultDescription}
              <div class="text-[12px]/[14px] text-text-alt"> -->
  <!-- TODO: Review marked vs. markdown-it and security risks. -->
  <!-- eslint-disable-next-line svelte/no-at-html-tags -->
  <!--    {@html md.render(resultItem.resultDescription)}
              </div>
            {/if}
          </div> -
        {/each}
      </div> 
    </div>
  {/if} -->

  <!-- "validFrom" is defined as REQUIRED in JSON Schema: https://purl.imsglobal.org/spec/ob/v3p0/schema/json/ob_v3p0_achievementcredential_schema.json -->
  {#if credential.data.validFrom}
    <TextFieldRenderer key={'validFrom'} value={credential.data.validFrom} />
  {/if}

  <!-- TODO: Where should the linked image be rendered? Overlap the one during issuance?  -->
  <!-- <img src={credential.data.credentialSubject?.achievement?.image?.id} alt="achievement" /> -->
</div>
