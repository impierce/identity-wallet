<script lang="ts">
  import { state } from '../../stores';
  import LL from '$i18n/i18n-svelte';

  const greetings = [
    'Welcome to your UniMe',
    'Hey',
    "What's up",
    'How are you',
    'Welcome back',
    'Hello'
  ];
</script>

<p class="text-3xl font-semibold text-slate-800 pt-[10px] pb-4">
  <!-- {$LL.WELCOME()}, <span class="text-indigo-500">{$state?.active_profile?.display_name}.</span> -->
  {greetings[Math.floor(Math.random() * greetings.length)]},
  <span class="text-indigo-500">{$state?.active_profile?.display_name}.</span>
</p>
