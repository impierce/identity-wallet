<script lang="ts">
  import LL from '$i18n/i18n-svelte';
  import { get } from 'svelte/store';
  import { z } from 'zod';

  import type { DisplayCredential } from '@bindings/credentials/DisplayCredential';

  import countries from '$lib/components/forms/countries';
  import { residence as schema } from '$lib/schemas/pid';

  let { credential }: { credential: DisplayCredential } = $props();

  // eslint-disable-next-line @typescript-eslint/no-unused-vars
  const residence = schema(get(LL));

  const subject = credential.data.credentialSubject as z.infer<typeof residence>;
</script>

<div class="flex flex-col gap-4">
  <div class="flex flex-col rounded-xl bg-background px-4 py-3 text-[13px]/[24px]">
    <h2 class="font-medium text-text-alt">{$LL.ADD_CREDENTIALS.ADDRESS.ADD.RESIDENT_ADDRESS_LABEL()}</h2>
    <div class="flex items-center">
      <div>
        <p class="overflow-x-auto">{subject.resident_street} {subject.resident_house_number}</p>
        <p class="overflow-x-auto">{subject.resident_postal_code}, {subject.resident_city}</p>
        <p class="overflow-x-auto">
          <!-- Only render a ", " when a `resident_state` is provided -->
          {subject.resident_state}{subject.resident_state ? ', ' : ''}
          {countries.find((c) => c.code === subject.resident_country)?.name}
        </p>
      </div>
    </div>
  </div>
</div>
