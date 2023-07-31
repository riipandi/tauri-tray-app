import type { Config } from 'tailwindcss'
import colors from 'tailwindcss/colors'

export default {
  content: ['src/**/*.{ts,tsx}', 'index.html'],
  darkMode: ['class', '[data-theme="dark"]'],
  theme: {
    extend: {
      colors: {
        'gray': colors.neutral,
        'background-light': colors.neutral[50],
        'background-dark': colors.neutral[900],
        'foreground-light': colors.neutral[950],
        'foreground-dark': colors.neutral[100],
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
