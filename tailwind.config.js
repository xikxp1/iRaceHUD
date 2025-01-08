import daisyui from 'daisyui'
import tailwindStripes from './src/plugins/tailwind-stripes.ts'

/** @type {import('tailwindcss').Config} */
export default {
  content: ["./src/**/*.{html,js,svelte,ts}"],
  theme: {
    extend: {},
    fontFamily: {
      iracing: ['"iracing"'],
      square: ['"iracing-square"']
    },
  },
  plugins: [
    daisyui,
    tailwindStripes
  ],
  daisyui: {
    base: false,
    themes: [
      {
        "iracing": {
          "primary": "#ededed",
          "primary-content": "#141414",
          "secondary": "#f1b228",
          "secondary-content": "#282828",
          "accent": "#e79122",
          "accent-content": "#130700",
          "neutral": "#ededed",
          "neutral-content": "#141414",
          "base-100": "#ffffff",
          "base-200": "#dedede",
          "base-300": "#bebebe",
          "base-content": "#161616",
          "info": "#00c7db",
          "info-content": "#000e11",
          "success": "#4def6f",
          "success-content": "#021404",
          "warning": "#f44819",
          "warning-content": "#140200",
          "error": "#dd0720",
          "error-content": "#ffd7d3",
        }
      }
    ],
  },
}
