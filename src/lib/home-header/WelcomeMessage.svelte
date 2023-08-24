<script lang="ts">
  import LL from '$src/i18n/i18n-svelte';
  import { state } from '$src/stores';

  const greetings = [
    'Welcome to your UniMe',
    'Hey',
    "What's up",
    'How are you',
    'Welcome back',
    'Hello'
  ];
</script>

<p class="pb-4 pt-[10px] text-3xl font-semibold text-slate-800">
  <!-- {$LL.WELCOME()}, <span class="text-indigo-500">{$state?.active_profile?.display_name}.</span> -->
  {greetings[Math.floor(Math.random() * greetings.length)]},
  <span class="text-indigo-500">{$state?.active_profile?.name}.</span>
</p>
