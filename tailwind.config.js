const colors = require('tailwindcss/colors');
const defaultTheme = require('tailwindcss/defaultTheme');

/** @type {import('tailwindcss').Config} */
export default {
  content: ['./src/**/*.{html,js,svelte,ts}'],
  theme: {
    // fontFamily: {
    //   sans: [
    //     'Inter var, sans-serif',
    //     {
    //       fontFeatureSettings: '"cv11", "ss01"',
    //       fontVariationSettings: '"opsz" 32'
    //     }
    //   ]
    // },
    extend: {
      fontFamily: {
        // sans: ['Satoshi', ...defaultTheme.fontFamily.sans],
        // sans: ['Satoshi-Variable', 'sans-serif']
        sans: ['Inter', 'sans-serif']
      },
      colors: {
        primary: '#5cc7c7',
        secondary: '#152d49',
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
