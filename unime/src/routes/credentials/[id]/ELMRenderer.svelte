<script lang="ts">
  import type { DisplayCredential } from '@bindings/credentials/DisplayCredential';

  import CollapsibleWrapper from './CollapsibleWrapper.svelte';

  export let credential: DisplayCredential;
  const credentialSubject = credential.data.credentialSubject;
  const hasClaim = credentialSubject.hasClaim;

  function isAllowedUrl(text: string): boolean {
    return /^(?:https?:\/\/|www\.)[^\s]+$/.test(text);
  }

  function toHref(text: string): string {
    return text.startsWith('www.') ? `https://${text}` : text;
  }
</script>

{#snippet TextField(title: string, value: string | undefined)}
  {#if value}
    <h4 class="text-text-alt">{title}</h4>
    <p class="overflow-x-auto">{value}</p>
  {/if}
{/snippet}

{#snippet URLField(title: string, value: string | undefined)}
  {#if value}
    <h4 class="text-text-alt">{title}</h4>
    {#if isAllowedUrl(value)}
      <a href={toHref(value)} target="_blank" rel="noopener noreferrer" class="overflow-x-auto break-all underline"
        >{value}</a
      >
    {:else}
      <p class="overflow-x-auto">{value}</p>
    {/if}
  {/if}
{/snippet}

<div class="flex flex-col gap-4">
  <!-- Collapsible Profile section -->
  {#if credentialSubject.familyName || credentialSubject.givenName || credentialSubject.dateOfBirth || credentialSubject.identifier}
    <CollapsibleWrapper defaultOpen={false}>
      <h2 class="text-lg font-bold" slot="title">Profile</h2>
      {@render TextField('Last name(s)', credentialSubject.familyName?.en)}

      {@render TextField('First name(s)', credentialSubject.givenName?.en)}

      {@render TextField('Date of birth', credentialSubject.dateOfBirth)}

      {@render TextField('Student number', credentialSubject.identifier?.notation)}
    </CollapsibleWrapper>
  {/if}

  <!-- Collapsible Qualification section -->
  <CollapsibleWrapper defaultOpen={true}>
    <h2 class="text-lg font-bold" slot="title">Qualification</h2>
    {@render TextField('Title', hasClaim?.title.en)}

    {@render TextField('Thematic Area', hasClaim?.specifiedBy?.thematicArea?.prefLabel?.en)}

    {@render TextField('Awarding institution', hasClaim?.awardedBy?.awardingBody?.legalName?.en)}

    {@render TextField(
      'Institution administering studies',
      hasClaim?.specifiedBy?.accreditation?.accreditingAgent?.legalName?.en,
    )}

    {@render TextField('Language', hasClaim?.specifiedBy?.language?.prefLabel?.en)}
  </CollapsibleWrapper>

  <!-- Collapsible Qualification level section -->
  <CollapsibleWrapper defaultOpen={false}>
    <h2 class="text-lg font-bold" slot="title">Qualification level</h2>
    {@render TextField('NLQF', hasClaim?.specifiedBy?.nqfLevel?.prefLabel?.en)}

    {@render TextField('EQF', hasClaim?.specifiedBy?.eqfLevel?.prefLabel?.en)}

    {@render TextField('Duration', hasClaim?.specifiedBy?.volumeOfLearning)}

    {@render TextField('ECTS', hasClaim?.specifiedBy?.creditPoint?.point)}

    {@render TextField('Access Requirement(s)', hasClaim?.specifiedBy?.entryRequirement?.noteLiteral?.en)}
  </CollapsibleWrapper>

  <!-- Collapsible Table section -->
  <CollapsibleWrapper defaultOpen={false}>
    <h2 class="text-lg font-bold" slot="title">Grades</h2>
    <table class="w-full table-auto text-left">
      <thead>
        <tr class="text-text-alt">
          <th class="py-1">Subject</th>
          <th class="py-1">Grade</th>
          <th class="py-1">Credits</th>
        </tr>
      </thead>
      <tbody>
        {#each hasClaim?.hasPart ?? [] as part}
          <tr>
            <td class="py-1">{part?.title?.en ?? ''}</td>
            <td class="py-1">{part?.provenBy?.[0]?.grade?.noteLiteral?.en ?? ''}</td>
            <td class="py-1">{part?.creditReceived?.[0]?.point ?? ''}</td>
          </tr>
        {/each}
      </tbody>
    </table>
  </CollapsibleWrapper>

  <!-- Collapsible Program information section -->
  <CollapsibleWrapper defaultOpen={false}>
    <h2 class="text-lg font-bold" slot="title">Program information</h2>
    {@render TextField('Mode', hasClaim?.specifiedBy?.mode?.prefLabel?.en)}

    {@render TextField('Learning Outcome', hasClaim?.specifiedBy?.learningOutcomeSummary?.noteLiteral?.en)}

    <!-- This block can't be looped as it's currently hardcoded we expect the 'Classification' claim to be in provenBy[1] as seen below (line 100) -->
    {@render TextField(
      hasClaim?.provenBy?.[0]?.specifiedBy?.gradingScheme?.title?.en,
      hasClaim?.provenBy?.[0]?.specifiedBy?.gradingScheme?.description?.en,
    )}

    {@render TextField('Classification', hasClaim?.provenBy?.[1]?.grade?.noteLiteral?.en)}
  </CollapsibleWrapper>

  <!-- Collapsible Qualification Function section -->
  <CollapsibleWrapper defaultOpen={false}>
    <h2 class="text-lg font-bold" slot="title">Qualification Function</h2>
    <!-- These hard indexing cases can't be refactored yet either as the titles are hardcoded -->
    {@render TextField('Access to further study', hasClaim?.entitlesTo?.[0]?.description?.en)}

    {@render TextField('Access to regulated profession', hasClaim?.entitlesTo?.[1]?.description?.en)}
  </CollapsibleWrapper>

  <!-- Collapsible Further information section -->
  <CollapsibleWrapper defaultOpen={false}>
    <h2 class="text-lg font-bold" slot="title">Further information</h2>
    {#each hasClaim?.supplementaryDocument ?? [] as doc}
      {@render URLField(doc?.title?.en, doc?.contentURL)}
    {/each}
  </CollapsibleWrapper>
</div>
