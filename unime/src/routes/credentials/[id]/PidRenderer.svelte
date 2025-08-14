<script lang="ts">
  import LL from '$i18n/i18n-svelte';
  import { get } from 'svelte/store';
  import { z } from 'zod';

  import type { DisplayCredential } from '@bindings/credentials/DisplayCredential';
  import Icon from '@iconify/svelte';

  import { naturalPerson as schema } from '$lib/schemas/pid';
  import { state as appState } from '$lib/stores';
  import { formatDate } from '$lib/utils';

  import TextFieldRenderer from './TextFieldRenderer.svelte';

  let { credential }: { credential: DisplayCredential } = $props();

  const residence = schema(get(LL));

  const subject = credential.data.credentialSubject as z.infer<typeof residence>;
</script>

<div class="grid w-full grid-cols-2 gap-4">
  <TextFieldRenderer key={$LL.CREDENTIAL.RENDERER.PID.NAME()} value={`${subject.given_name} ${subject.family_name}`} />
  <div class="flex flex-col rounded-xl bg-background px-4 py-3 text-[13px]/[24px]">
    <h2 class="font-medium text-text-alt">{$LL.CREDENTIAL.RENDERER.PID.NATIONALITY()}</h2>
    <div class="flex grow items-center gap-2">
      {#each subject.nationality as nationality}
        <span class="overflow-hidden rounded">
          <Icon class="size-5" icon={`circle-flags:${nationality.toLowerCase()}`} />
        </span>
      {/each}
    </div>
  </div>
  <!-- TODO: temporarily hide birth_date field while there is no proper input for it (date picker) -->
  <!-- <TextFieldRenderer
    key={$LL.CREDENTIAL.RENDERER.PID.BIRTH_DATE()}
    value={formatDate(subject.birth_date, $appState.profile_settings.locale)}
  /> -->
  <TextFieldRenderer key={$LL.CREDENTIAL.RENDERER.PID.BIRTH_PLACE()} value={subject.birth_place} />
  <!-- <TextFieldRenderer key={'family_name_birth'} value={subject.family_name_birth ?? '-'} />
  <TextFieldRenderer key={'sex'} value={subject.sex?.toString() ?? '-'} /> -->
</div>
