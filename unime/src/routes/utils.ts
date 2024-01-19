export function determineTheme(
  systemPrefersDark: boolean,
  userPreference: 'system' | 'light' | 'dark',
): 'light' | 'dark' {
  if (systemPrefersDark) {
    if (userPreference === 'light') {
      // system: dark, user: light
      document.documentElement.classList.remove('dark');
      return 'light';
    } else {
      // system: dark, user: dark or system
      document.documentElement.classList.add('dark');
      return 'dark';
    }
  } else {
    if (userPreference === 'dark') {
      // system: light, user: dark
      document.documentElement.classList.add('dark');
      return 'dark';
    } else {
      // system: light, user: light or system
      document.documentElement.classList.remove('dark');
      return 'light';
    }
  }
}
