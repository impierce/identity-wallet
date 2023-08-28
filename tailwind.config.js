const colors = require('tailwindcss/colors');
const defaultTheme = require('tailwindcss/defaultTheme');

/** @type {import('tailwindcss').Config} */
export default {
  content: ['./src/**/*.{html,js,svelte,ts}'],
  theme: {
    extend: {
      fontFamily: {
        // sans: ['Satoshi', ...defaultTheme.fontFamily.sans],
        sans: ['Satoshi-Variable', 'sans-serif']
      },
      colors: {
        primary: '#83CCCC',
        secondary: colors.pink[500],
        'bg-primary': colors.white,
        'bg-secondary': colors.neutral[100],
        'bg-dark-primary': '#13243D',
        'bg-dark-secondary': '#152D49'
      }
    }
  },
  darkMode: 'class',
  plugins: []
};
