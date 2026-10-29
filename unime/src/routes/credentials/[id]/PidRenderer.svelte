<script lang="ts">
  import LL from '$i18n/i18n-svelte';
  import { get } from 'svelte/store';
  import { z } from 'zod';

  import type { DisplayCredential } from '@bindings/credentials/DisplayCredential';
  import Icon from '@iconify/svelte';

  import { naturalPerson as schema } from '$lib/schemas/pid';
  import { state as appState } from '$lib/stores';

  import TextFieldRenderer from './(renderers)/TextFieldRenderer.svelte';

  let { credential }: { credential: DisplayCredential } = $props();

  // eslint-disable-next-line @typescript-eslint/no-unused-vars
  const residence = schema(get(LL));

  const subject = $derived(credential.data.credentialSubject as z.infer<typeof residence>);

  // A date of birth is a calendar date (older profiles stored it as midnight UTC), so it's shown in UTC to avoid
  // showing the day before in time zones west of UTC.
  const birthDate = $derived.by(() => {
    const date = new Date(subject.birth_date);
    if (isNaN(date.getTime())) return subject.birth_date;
    return new Intl.DateTimeFormat($appState.profile_settings.locale, { dateStyle: 'medium', timeZone: 'UTC' }).format(
      date,
    );
  });
</script>

<div class="grid w-full grid-cols-2 gap-4">
  <TextFieldRenderer
    key={$LL.CREDENTIAL.RENDERER.PID.NAME()}
    value={[subject.given_name, subject.middle_name, subject.family_name].filter(Boolean).join(' ')}
  />
  <div class="flex flex-col rounded-xl bg-background px-4 py-3 text-[13px]/[24px]">
    <h2 class="font-medium text-text-alt">{$LL.CREDENTIAL.RENDERER.PID.NATIONALITY()}</h2>
    <div class="flex grow items-center gap-2">
      {#each subject.nationality as nationality}
        <span class="overflow-hidden rounded-sm">
          <Icon class="size-5" icon={`circle-flags:${nationality.toLowerCase()}`} />
        </span>
      {/each}
    </div>
  </div>
  {#if subject.birth_date}
    <TextFieldRenderer key={$LL.CREDENTIAL.RENDERER.PID.BIRTH_DATE()} value={birthDate} />
  {/if}
  <TextFieldRenderer key={$LL.CREDENTIAL.RENDERER.PID.BIRTH_PLACE()} value={subject.birth_place} />
  <!-- <TextFieldRenderer key={'family_name_birth'} value={subject.family_name_birth ?? '-'} />
  <TextFieldRenderer key={'sex'} value={subject.sex?.toString() ?? '-'} /> -->
</div>
