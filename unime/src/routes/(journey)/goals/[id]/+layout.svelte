<script lang="ts">
  import { page } from '$app/stores';

  import { TopNavBar } from '$lib/components';
  import { state } from '$lib/stores';

  import type { Goal } from '../types';

  let goal: Goal = $state?.user_journey?.goals.find((g) => g.id === parseInt($page.params.id));
</script>

<TopNavBar on:back={() => history.back()} title={goal.label}>
  <!-- <button
      class="w-full py-4 text-left text-primary"
      on:click={() => dispatch({ type: '[User Journey] Cancel' })}>Skip</button
    > -->
</TopNavBar>
<slot />
