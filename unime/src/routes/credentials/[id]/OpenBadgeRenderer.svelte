<script lang="ts">
  import LL from '$i18n/i18n-svelte';
  import markdownit from 'markdown-it';

  import type { DisplayCredential } from '@bindings/credentials/DisplayCredential';

  import { toAlignments } from '$lib/utils/alignment';

  import AlignmentRenderer from './(renderers)/AlignmentRenderer.svelte';
  import ClaimRenderer from './(renderers)/ClaimRenderer.svelte';
  import InlineClaimRenderer from './(renderers)/InlineClaimRenderer.svelte';
  import TextFieldRenderer from './(renderers)/TextFieldRenderer.svelte';
  import CollapsibleWrapper from './CollapsibleWrapper.svelte';

  export let credential: DisplayCredential;

  const md = markdownit();

  const subject = credential.data.credentialSubject ?? {};

  // The recipient's profile is an optional extension of the `AchievementSubject` (which allows additional properties).
  const profile: Record<string, unknown> | undefined = subject.profile;

  // `AchievementSubject` fields that have a section of their own, or that hold structured data instead of a value
  // that can be displayed as is.
  const renderedFields: string[] = ['id', 'type', 'profile', 'achievement', 'result', 'identifier', 'image', 'source'];

  // The remaining `AchievementSubject` fields, e.g. `activityStartDate` or `creditsEarned`.
  const fields = Object.keys(subject).filter((field) => !renderedFields.includes(field));

  const alignments = toAlignments(subject.achievement?.alignment);
</script>

<div class="flex flex-col gap-4">
  <!-- Achievement -->
  {#if credential.data.credentialSubject?.achievement?.description}
    <CollapsibleWrapper defaultOpen={true}>
      <h2 class="text-lg font-bold" slot="title">{$LL.CREDENTIAL.DETAILS.DESCRIPTION()}</h2>
      <!-- TODO: Review marked vs. markdown-it and security risks. -->
      <!-- eslint-disable-next-line svelte/no-at-html-tags -->
      {@html md.render(credential.data.credentialSubject.achievement.description)}
    </CollapsibleWrapper>
  {/if}

  {#if credential.data.credentialSubject?.achievement?.criteria?.narrative}
    <CollapsibleWrapper defaultOpen={false}>
      <h2 class="text-lg font-bold" slot="title">{$LL.CREDENTIAL.DETAILS.OPEN_BADGES.CRITERIA()}</h2>
      <!-- TODO: Review marked vs. markdown-it and security risks. -->
      <!-- eslint-disable-next-line svelte/no-at-html-tags -->
      {@html md.render(credential.data.credentialSubject.achievement.criteria.narrative)}
    </CollapsibleWrapper>
  {/if}

  <!-- Recipient -->
  {#if profile}
    <CollapsibleWrapper defaultOpen={false}>
      <h2 class="text-lg font-bold" slot="title">{$LL.CREDENTIAL.DETAILS.OPEN_BADGES.RECIPIENT()}</h2>
      <div class="flex flex-col gap-2">
        {#each Object.entries(profile) as [key, value]}
          <InlineClaimRenderer {key} {value} />
        {/each}
      </div>
    </CollapsibleWrapper>
  {/if}

  {#if alignments.length > 0}
    <CollapsibleWrapper defaultOpen={false}>
      <h2 class="text-lg font-bold" slot="title">{$LL.CREDENTIAL.DETAILS.OPEN_BADGES.ALIGNMENT()}</h2>
      <div class="flex flex-col gap-3">
        {#each alignments as alignment}
          <AlignmentRenderer {alignment} />
        {/each}
      </div>
    </CollapsibleWrapper>
  {/if}

  <!-- Result -->
  {#if credential.data.credentialSubject?.result?.length > 0}
    <CollapsibleWrapper defaultOpen={false}>
      <h2 class="text-lg font-bold" slot="title">
        {$LL.CREDENTIAL.DETAILS.OPEN_BADGES.RESULT()}
      </h2>
      <div class="flex flex-col divide-y divide-slate-300">
        {#each credential.data.credentialSubject.result as resultItem}
          <div class="py-4 first:pt-0 last:pb-0">
            <div class="flex flex-col gap-3">
              {#each toAlignments(resultItem.alignment) as resultAlignment}
                <AlignmentRenderer alignment={resultAlignment} />
              {/each}
            </div>

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

  <!-- Fields that are rendered as plain values follow below the collapsible sections. -->
  {#if credential.data.credentialSubject?.achievement?.achievementType}
    <TextFieldRenderer
      key={'Achievement type'}
      value={credential.data.credentialSubject?.achievement?.achievementType
        .replaceAll('ext:', '')
        .replaceAll('_', ' ')}
    />
  {/if}

  {#if credential.data.credentialSubject?.achievement?.fieldOfStudy}
    <ClaimRenderer key={'fieldOfStudy'} value={credential.data.credentialSubject.achievement.fieldOfStudy} />
  {/if}

  {#if credential.data.credentialSubject?.achievement?.specialization}
    <ClaimRenderer key={'specialization'} value={credential.data.credentialSubject.achievement.specialization} />
  {/if}

  <!-- Remaining `AchievementSubject` fields, e.g. `activityStartDate`, `creditsEarned` or `role`. -->
  {#each fields as field}
    <ClaimRenderer key={field} value={subject[field]} />
  {/each}

  <!-- "validFrom" is defined as REQUIRED in JSON Schema: https://purl.imsglobal.org/spec/ob/v3p0/schema/json/ob_v3p0_achievementcredential_schema.json -->
  {#if credential.data.validFrom}
    <ClaimRenderer key={'validFrom'} value={credential.data.validFrom} />
  {/if}

  <!-- TODO: Where should the linked image be rendered? Overlap the one during issuance?  -->
  <!-- <img src={credential.data.credentialSubject?.achievement?.image?.id} alt="achievement" /> -->
</div>
