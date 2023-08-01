<script lang="ts">
  import { page } from '$app/stores';
  import onboardingJourney from '../../../journey-definition.json';
  import type { Goal } from '../../types';

  /* TODO: extract to ui-component */
  import {
    Accordion,
    AccordionContent,
    AccordionItem,
    AccordionTrigger,
    Button
  } from '@impierce/ui-components';

  let goal: Goal = onboardingJourney.goals.find((g) => g.id === parseInt($page.params.id))!!;
</script>

<div class="flex h-full flex-col items-center justify-evenly p-8 space-y-8">
  <p class="text-2xl font-semibold text-slate-600">{goal?.label}</p>
  <Accordion type="single" collapsible class="w-full">
    {#each goal?.faqs as faq}
      <AccordionItem value="item-{faq.id}">
        <AccordionTrigger>{faq.title}</AccordionTrigger>
        <AccordionContent>
          {faq.content}
        </AccordionContent>
      </AccordionItem>
    {/each}
  </Accordion>
  <Button href="/goals/2/step/0">Continue</Button>
</div>
