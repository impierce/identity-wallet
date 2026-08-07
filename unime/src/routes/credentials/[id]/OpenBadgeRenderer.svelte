<script lang="ts">
  import LL from '$i18n/i18n-svelte';
  import markdownit from 'markdown-it';

  import type { DisplayCredential } from '@bindings/credentials/DisplayCredential';

  import TextFieldRenderer from './(renderers)/TextFieldRenderer.svelte';
  import CollapsibleWrapper from './CollapsibleWrapper.svelte';
  import InfoTooltip from './InfoTooltip.svelte';

  export let credential: DisplayCredential;

  const md = markdownit();
</script>

<div class="flex flex-col gap-4">
  <!-- Achievement -->
  {#if credential.data.credentialSubject?.achievement?.description}
    <CollapsibleWrapper defaultOpen={true}>
      <h2 class="text-lg font-bold" slot="title">{$LL.CREDENTIAL.DETAILS.DESCRIPTION()}</h2>
      <InfoTooltip description="Describes a possible achievement result." />
      <!-- TODO: Review marked vs. markdown-it and security risks. -->
      <!-- eslint-disable-next-line svelte/no-at-html-tags -->
      {@html md.render(credential.data.credentialSubject.achievement.description)}
    </CollapsibleWrapper>
  {/if}

  {#if credential.data.credentialSubject?.achievement?.criteria?.narrative}
    <CollapsibleWrapper defaultOpen={false}>
      <h2 class="text-lg font-bold" slot="title">{$LL.CREDENTIAL.DETAILS.OPEN_BADGES.CRITERIA()}</h2>
      <InfoTooltip description="Criteria describing how to earn the achievement." />
      <!-- TODO: Review marked vs. markdown-it and security risks. -->
      <!-- eslint-disable-next-line svelte/no-at-html-tags -->
      {@html md.render(credential.data.credentialSubject.achievement.criteria.narrative)}
    </CollapsibleWrapper>
  {/if}

  {#if credential.data.credentialSubject?.achievement?.achievementType}
    <TextFieldRenderer
      key={'Achievement type'}
      value={credential.data.credentialSubject?.achievement?.achievementType
        .replaceAll('ext:', '')
        .replaceAll('_', ' ')}
    />
  {/if}

  {#if credential.data.credentialSubject?.achievement?.alignment?.length > 0}
    <CollapsibleWrapper defaultOpen={false}>
      <h2 class="text-lg font-bold" slot="title">{$LL.CREDENTIAL.DETAILS.OPEN_BADGES.ALIGNMENT()}</h2>
      <InfoTooltip
        description="An object describing which objectives or educational standards this achievement aligns to, if any."
      />
      {#each credential.data.credentialSubject.achievement.alignment as alignmentItem}
        <h4>{alignmentItem.targetName}</h4>
        {#if alignmentItem.targetDescription}
          <!-- TODO: Review marked vs. markdown-it and security risks. -->
          <!-- eslint-disable-next-line svelte/no-at-html-tags -->
          {@html md.render(alignmentItem.targetDescription)}
        {/if}
      {/each}
    </CollapsibleWrapper>
  {/if}

  <!-- Result -->
  {#if credential.data.credentialSubject?.result?.length > 0}
    <CollapsibleWrapper defaultOpen={false}>
      <h2 class="text-lg font-bold" slot="title">
        {$LL.CREDENTIAL.DETAILS.OPEN_BADGES.RESULT()}
      </h2>
      <InfoTooltip description="Describes a result that was achieved." />
      <div class="flex flex-col divide-y divide-slate-300">
        {#each credential.data.credentialSubject.result as resultItem}
          <div class="py-4 first:pt-0 last:pb-0">
            {#if resultItem.alignment?.length > 0}
              {#each resultItem.alignment as resultAlignment}
                <h4 class="font-bold">{resultAlignment.targetName}</h4>
                {#if resultAlignment.targetDescription}
                  <!-- TODO: Review marked vs. markdown-it and security risks. -->
                  <!-- eslint-disable-next-line svelte/no-at-html-tags -->
                  {@html md.render(resultAlignment.targetDescription)}
                {/if}
              {/each}
            {/if}

            {#if resultItem.value}
              <div class="flex h-4 items-center justify-between">
                <h4>{$LL.CREDENTIAL.DETAILS.OPEN_BADGES.VALUE()}</h4>
                <div class="mr-1 text-base font-bold">
                  <!-- TODO: Review marked vs. markdown-it and security risks. -->
                  <!-- eslint-disable-next-line svelte/no-at-html-tags -->
                  {@html md.render(resultItem.value)}
                </div>
              </div>
            {/if}

            {#if resultItem.resultDescription}
              <div class="text-[12px]/[14px] text-text-alt">
                <!-- TODO: Review marked vs. markdown-it and security risks. -->
                <!-- eslint-disable-next-line svelte/no-at-html-tags -->
                {@html md.render(resultItem.resultDescription)}
              </div>
            {/if}
          </div>
        {/each}
      </div>
    </CollapsibleWrapper>
  {/if}

  <!-- "validFrom" is defined as REQUIRED in JSON Schema: https://purl.imsglobal.org/spec/ob/v3p0/schema/json/ob_v3p0_achievementcredential_schema.json -->
  {#if credential.data.validFrom}
    <TextFieldRenderer key={'validFrom'} value={credential.data.validFrom} />
  {/if}

  <!-- TODO: Where should the linked image be rendered? Overlap the one during issuance?  -->
  <!-- <img src={credential.data.credentialSubject?.achievement?.image?.id} alt="achievement" /> -->
</div>
