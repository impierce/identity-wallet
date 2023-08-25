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
        primary: colors.indigo[500],
        secondary: colors.pink[500],
      },
    }
  },
  darkMode: 'class',
  plugins: []
};
