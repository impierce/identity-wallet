<script lang="ts">
  import { goto } from '$app/navigation';
  import { page } from '$app/stores';
  import Button from '$lib/components/Button.svelte';
  import { state } from '../../../../../stores';
  import type { Goal } from '../../types';

  /* TODO: extract to ui-component */
  import {
    Accordion,
    AccordionContent,
    AccordionItem,
    AccordionTrigger
  } from '@impierce/ui-components';

  let goal: Goal = $state?.user_journey?.goals.find((g) => g.id === parseInt($page.params.id));
</script>

<div class="flex h-full flex-col items-center justify-evenly space-y-8 bg-neutral-100 p-6">
  <p class="text-center text-2xl font-semibold text-slate-600">{goal?.label}</p>
  <div class="flex w-full grow flex-col px-6">
    <Accordion type="single" collapsible class="">
      {#each goal?.faqs as faq}
        <AccordionItem value="item-{faq.id}">
          <AccordionTrigger>{faq.title}</AccordionTrigger>
          <AccordionContent>
            {faq.content}
          </AccordionContent>
        </AccordionItem>
      {/each}
    </Accordion>
  </div>
  <Button label="Continue" on:click={() => goto('/goals/2/step/0')} />
</div>
