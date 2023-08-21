<script>
    import { goto } from '$app/navigation';
    import { BottomNavigation } from '@impierce/ui-components';
    import { fly } from 'svelte/transition';
    import LL from '../../i18n/i18n-svelte';
  import PasswordRequired from '$lib/password_required/PasswordRequired.svelte';
  </script>
  
  <div class="flex min-h-screen flex-col bg-slate-100 dark:bg-slate-700">
   <PasswordRequired isOpen=true />
  </div>
  
  <style>
    .safe-bottom {
      bottom: env(safe-area-inset-bottom);
    }
  </style>
  