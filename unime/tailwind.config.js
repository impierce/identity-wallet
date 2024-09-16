import typography from '@tailwindcss/typography';

/** @type {import('tailwindcss').Config} */
export default {
  content: ['./src/**/*.{html,js,svelte,ts}'],
  theme: {
    extend: {
      fontFamily: {
        sans: ['Inter', 'sans-serif'],
      },
      colors: {
        // Semantic color definitions.
        brand: 'rgb(var(--color-brand))',
        text: 'rgb(var(--color-text))',
        'text-alt': 'rgb(var(--color-text-alt))',
        background: 'rgb(var(--color-background))',
        'background-alt': 'rgb(var(--color-background-alt))',

        // Old color definitions.

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
          7: '#c9d9d0', // green
        },
      },
    },
  },
  darkMode: 'class',
  plugins: [typography],
};
