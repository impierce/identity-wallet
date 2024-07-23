<script lang="ts">
  import LL from '$i18n/i18n-svelte';

  import { state } from '$lib/stores';

  const greetings = Array.from([
    $LL.ME.GREETINGS.GREETING_0(),
    $LL.ME.GREETINGS.GREETING_1(),
    $LL.ME.GREETINGS.GREETING_2(),
    $LL.ME.GREETINGS.GREETING_3(),
    $LL.ME.GREETINGS.GREETING_4(),
  ]);
</script>

<p class="text-3xl font-semibold tracking-tight text-slate-700 dark:text-grey">
  {greetings.at(Math.floor(Math.random() * greetings.length))},
  <span class="text-primary">{$state?.profile_settings.profile?.name}.</span>
</p>
