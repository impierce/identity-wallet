<script>
  import { TopNavigation } from '@impierce/ui-components';
  import exampleJourneyDefinition from '$lib/example/data/journey-definition.json';
  import { page } from '$app/stores';
</script>

<TopNavigation on:back={() => history.back()} title={exampleJourneyDefinition.title} />
<div class="h-[calc(100vh-56px-1px)]">
  <slot />
</div>

<!-- DEBUG: Displays the current path at the bottom -->
<div class="fixed bottom-2 left-2 rounded bg-orange-100 p-1 font-mono text-xs text-orange-600">
  {$page.url.pathname}
</div>
