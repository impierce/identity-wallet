<script lang="ts">
  import LL from '$i18n/i18n-svelte';
  import { get } from 'svelte/store';
  import { z } from 'zod';

  import type { DisplayCredential } from '@bindings/credentials/DisplayCredential';
  import Icon from '@iconify/svelte';

  import * as countriesData from '$lib/components/forms/countries.json';
  import { residence as schema } from '$lib/schemas/pid';

  let { credential }: { credential: DisplayCredential } = $props();

  const residence = schema(get(LL));

  const subject = credential.data.credentialSubject as z.infer<typeof residence>;

  const countries: { code: string; name: string }[] = (countriesData as any).default;
</script>

<div class="flex flex-col gap-4">
  <div class="flex items-center rounded-xl bg-background px-4 py-3 text-[13px]/[24px]">
    {#if subject.resident_country}
      <div class="mr-4 rounded-full p-1 dark:bg-[#f9f9f9]">
        <Icon class="size-5" icon={`circle-flags:${subject.resident_country.toLowerCase()}`} />
      </div>
    {/if}
    <div>
      <p class="overflow-x-auto">{subject.resident_street} {subject.resident_house_number}</p>
      <p class="overflow-x-auto">{subject.resident_postal_code} {subject.resident_city}</p>
      <p class="overflow-x-auto">
        <!-- Only render a ", " when there is a resident_state -->
        {subject.resident_state}{subject.resident_state ? ', ' : ''}{countries.find(
          (c) => c.code === subject.resident_country,
        )?.name}
      </p>
    </div>
  </div>
</div>
