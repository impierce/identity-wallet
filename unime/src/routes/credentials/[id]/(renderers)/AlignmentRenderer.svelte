<script lang="ts">
  import LL from '$i18n/i18n-svelte';
  import markdownit from 'markdown-it';

  import { ArrowSquareOutBoldIcon, SealCheckFillIcon } from '$lib/icons';
  import { findOfficialSkill, type Alignment } from '$lib/utils/alignment';

  export let alignment: Alignment;

  const md = markdownit();

  $: skill = findOfficialSkill(alignment);
  $: kind =
    skill?.kind === 'skill'
      ? $LL.CREDENTIAL.DETAILS.OPEN_BADGES.SKILL()
      : skill?.kind === 'occupation'
        ? $LL.CREDENTIAL.DETAILS.OPEN_BADGES.OCCUPATION()
        : null;
</script>

<!--
@component
Renders a single `Alignment` of an achievement or of a result.

An alignment that references an entry of a skills framework (e.g. a skill or an occupation picked from ESCO) is
rendered as an official skill: the framework it was taken from, the code that identifies it there and a link to its
definition are shown alongside its name. An alignment that is written by hand during template creation has none of
these, so it is rendered as its name and description alone.
-->
{#if skill}
  <div class="not-prose flex flex-col gap-2 rounded-xl border border-brand/40 bg-brand/5 px-4 py-3">
    <div class="flex items-center gap-1.5 text-[11px]/[16px] font-semibold tracking-wide text-text-alt uppercase">
      <SealCheckFillIcon class="size-4 shrink-0 text-brand" />
      <span class="truncate">{skill.framework}</span>
      {#if kind}
        <span aria-hidden="true">&middot;</span>
        <span class="truncate">{kind}</span>
      {/if}
    </div>

    <h4 class="text-[15px]/[20px] font-semibold">{skill.name}</h4>

    {#if skill.description}
      <p class="text-[13px]/[20px] text-text-alt">{skill.description}</p>
    {/if}

    {#if skill.code || skill.url}
      <div class="flex items-center gap-3">
        {#if skill.code}
          <span class="truncate rounded-md bg-background-alt px-2 py-0.5 font-mono text-[11px]/[20px] text-text-alt">
            {skill.code}
          </span>
        {/if}
        {#if skill.url}
          <a
            href={skill.url}
            target="_blank"
            rel="noopener noreferrer"
            class="ml-auto flex shrink-0 items-center gap-1 text-[11px]/[20px] font-medium underline"
          >
            {$LL.CREDENTIAL.DETAILS.OPEN_BADGES.FRAMEWORK_LINK()}
            <ArrowSquareOutBoldIcon class="size-3" />
          </a>
        {/if}
      </div>
    {/if}
  </div>
{:else if alignment.targetName}
  <div>
    <h4>{alignment.targetName}</h4>
    {#if alignment.targetDescription}
      <!-- TODO: Review marked vs. markdown-it and security risks. -->
      <!-- eslint-disable-next-line svelte/no-at-html-tags -->
      {@html md.render(alignment.targetDescription)}
    {/if}
  </div>
{/if}
