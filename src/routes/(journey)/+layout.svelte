<script>
  import { TopNavigation } from '@impierce/ui-components';
  import journeyDefinition from './journey-definition.json';
  import { page } from '$app/stores';
</script>

<TopNavigation on:back={() => history.back()} title={journeyDefinition.title} />
{$page.url.pathname}
<div class="h-[calc(100vh-56px-1px)]">
  <!-- TopNavigation: height 56px, border 1px -->
  <slot />
</div>
