<script lang="ts">
  import { state } from '$src/stores';
  import { page } from '$app/stores';
  import { TopNavigation } from '@impierce/ui-components';
  import type { Goal } from '../types';

  let goal: Goal = $state?.user_journey?.goals.find((g) => g.id === parseInt($page.params.id));
</script>

<TopNavigation on:back={() => history.back()} title={goal.label}>
  <!-- <button
      class="w-full py-4 text-left text-indigo-500"
      on:click={() => dispatch({ type: '[User Journey] Cancel' })}>Skip</button
    > -->
</TopNavigation>
<slot />
