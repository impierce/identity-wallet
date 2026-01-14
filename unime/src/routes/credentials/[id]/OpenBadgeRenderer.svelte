<script lang="ts">
  import LL from '$i18n/i18n-svelte';
  import markdownit from 'markdown-it';
  import { fade } from 'svelte/transition';

  import type { DisplayCredential } from '@bindings/credentials/DisplayCredential';
  import { createTooltip, melt } from '@melt-ui/svelte';

  import InfoFillIcon from '~icons/ph/info-fill';

  import InfoTooltip from './InfoTooltip.svelte';
  import TextFieldRenderer from './TextFieldRenderer.svelte';

  export let credential: DisplayCredential;

  const md = markdownit();
</script>

<div class="flex flex-col gap-4">
  <!-- Achievement -->
  {#if credential.data.credentialSubject?.achievement?.description}
    <div class="rounded-xl bg-background p-4 dark:prose-invert">
      <!-- Title & Info Icon -->
      <div class="flex items-center justify-between">
        <h2 class="m-0 text-xl font-bold">{$LL.CREDENTIAL.DETAILS.DESCRIPTION()}</h2>

        <InfoTooltip description="Describes a possible achievement result." />
      </div>

      <div class="prose prose-sm mt-4 max-w-none dark:prose-invert">
        <!-- TODO: Review marked vs. markdown-it and security risks. -->
        <!-- eslint-disable-next-line svelte/no-at-html-tags -->
        {@html md.render(credential.data.credentialSubject.achievement.description)}
      </div>
    </div>
  {/if}

  {#if credential.data.credentialSubject?.achievement?.criteria?.narrative}
    <div class="rounded-xl bg-background p-4 dark:prose-invert">
      <div class="flex items-center justify-between">
        <h2 class="m-0 text-xl font-bold">{$LL.CREDENTIAL.DETAILS.OPEN_BADGES.CRITERIA()}</h2>
        <InfoTooltip description="Criteria describing how to earn the achievement." />
      </div>

      <div class="prose prose-sm mt-4 max-w-none dark:prose-invert">
        <!-- TODO: Review marked vs. markdown-it and security risks. -->
        <!-- eslint-disable-next-line svelte/no-at-html-tags -->
        {@html md.render(credential.data.credentialSubject.achievement.criteria.narrative)}
      </div>
    </div>
  {/if}

  {#if credential.data.credentialSubject?.achievement?.achievementType}
    <TextFieldRenderer
      key={'Achievement type'}
      value={credential.data.credentialSubject?.achievement?.achievementType}
    />
  {/if}

  {#if credential.data.credentialSubject?.achievement?.alignment?.length > 0}
    <div class="rounded-xl bg-background p-4 dark:prose-invert">
      <div class="flex items-center justify-between">
        <h2 class="m-0 text-xl font-bold">{$LL.CREDENTIAL.DETAILS.OPEN_BADGES.ALIGNMENT()}</h2>
        <InfoTooltip
          description="An object describing which objectives or educational standards this achievement aligns to, if any."
        />
      </div>

      <div class="prose prose-sm max-w-none dark:prose-invert">
        {#each credential.data.credentialSubject.achievement.alignment as alignmentItem}
          <h4 class="mt-5">{alignmentItem.targetName}</h4>
          {#if alignmentItem.targetDescription}
            <!-- TODO Review marked vs. markdown-it and security risks. -->
            <!-- eslint-disable-next-line svelte/no-at-html-tags -->
            {@html md.render(alignmentItem.targetDescription)}
          {/if}
        {/each}
      </div>
    </div>
  {/if}

  <!-- Result -->
  {#if credential.data.credentialSubject?.result?.length > 0}
    <div class="rounded-xl bg-background p-4 dark:prose-invert">
      <div class="flex items-center justify-between">
        <h2 class="m-0 text-xl font-bold">{$LL.CREDENTIAL.DETAILS.OPEN_BADGES.RESULT()}</h2>
        <InfoTooltip description="Describes a result that was achieved." />
      </div>
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
              <div class="flex h-16 items-center justify-between">
                <h4 class="text-sm font-bold">{$LL.CREDENTIAL.DETAILS.OPEN_BADGES.VALUE()}</h4>
                <div class="text-2xl font-bold">
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
    </div>
  {/if}

  <!-- "validFrom" is defined as REQUIRED in JSON Schema: https://purl.imsglobal.org/spec/ob/v3p0/schema/json/ob_v3p0_achievementcredential_schema.json -->
  {#if credential.data.validFrom}
    <TextFieldRenderer key={'validFrom'} value={credential.data.validFrom} />
  {/if}

  <!-- TODO: Where should the linked image be rendered? Overlap the one during issuance?  -->
  <!-- <img src={credential.data.credentialSubject?.achievement?.image?.id} alt="achievement" /> -->
</div>
