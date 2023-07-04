const defaultTheme = require('tailwindcss/defaultTheme');

/** @type {import('tailwindcss').Config} */
export default {
  content: ['./src/**/*.{html,js,svelte,ts}'],
  theme: {
    extend: {
      // fontFamily: {
      //   // sans: ['Satoshi', ...defaultTheme.fontFamily.sans],
      //   sans: ['Satoshi-Variable', 'sans-serif']
      // },
      // colors: {
      //   sunrise: '#FDDA0D',
      // },
      boxShadow: {
        'neon': '0 0 5px theme("colors.violet.200"), 0 0 20px theme("colors.violet.700")'
      }
    }
  },
  darkMode: 'class',
  plugins: []
};
