import type { Config } from 'tailwindcss'

export default {
  content: ['src/**/*.{ts,tsx}', 'index.html'],
  darkMode: 'media',
  theme: {
    extend: {
      colors: {
        black: '#1C1E21',
        dark: '#2F2F2F',
      },
    },
    debugScreens: {
      position: ['bottom', 'left'],
      style: {
        backgroundColor: '#C0FFEE',
        color: 'black',
      },
    },
  },
  plugins: [
    require('@tailwindcss/aspect-ratio'),
    require('@tailwindcss/forms'),
    require('@tailwindcss/typography'),
    require('tailwindcss-animate'),
    require('tailwind-scrollbar')({ nocompatible: true }),
  ],
} satisfies Config
