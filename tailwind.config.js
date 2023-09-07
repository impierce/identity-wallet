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
        // accent
        primary: '#5cc7c7',
        teal: '#50b5b2',
        // light
        silver: '#f9f9f9',
        grey: '#efefef',
        // dark
        navy: '#1e3959',
        blue: '#152d49',
        dark: '#13243d',
        // extended
        'ex-blue-1': '#eaf4f4',
        'ex-blue-2': '#d4eded',
        'ex-blue-3': '#b9cfd0',
        'ex-grey-1': '#5b676b',
        'ex-grey-2': '#323b40',
        'ex-grey-3': '#171d23',
        credentials: {
          0: '#d7e0cc', // green
          1: '#e3d1b6', // gold
          2: '#afcbdd', // blue
          3: '#acb7e2', // purple
          4: '#c6d3d8', // gray
          5: '#dddcf1', // purple
          6: '#d9cadd', // red
          7: '#c9d9d0' // green
        }
        // slate (Tailwind default)
        /*
        50: #f8fafc,
        100: #f1f5f9,
        200: #e2e8f0,
        300: #cbd5e1,
        400: #94a3b8,
        500: #64748b,
        600: #475569,
        700: #334155,
        800: #1e293b,
        900: #0f172a
        */
      }
    }
  },
  darkMode: 'class',
  plugins: []
};
