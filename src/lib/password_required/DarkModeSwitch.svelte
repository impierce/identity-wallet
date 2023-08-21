<!-- source: https://dev.to/willkre/persistent-theme-switch-dark-mode-with-svelte-sveltekit-tailwind-1b9g -->
<script lang="ts">
    import { browser } from '$app/environment';
  
    let darkMode = true;
  
    function handleSwitchDarkMode() {
      darkMode = !darkMode;
  
      localStorage.setItem('theme', darkMode ? 'dark' : 'light');
  
      darkMode
        ? document.documentElement.classList.add('dark')
        : document.documentElement.classList.remove('dark');
    }
  
    if (browser) {
      if (
        localStorage.theme === 'dark' ||
        (!('theme' in localStorage) && window.matchMedia('(prefers-color-scheme: dark)').matches)
      ) {
        document.documentElement.classList.add('dark');
        darkMode = true;
      } else {
        document.documentElement.classList.remove('dark');
        darkMode = false;
      }
    }
  </script>
  
  <!-- this belongs in app.css -->
  <style>
    body {
      transition: background-color 0.2s ease-in-out, color 0.2s ease-in-out;
      @apply bg-white text-center text-black dark:bg-black dark:text-white;
    }
  </style>
  