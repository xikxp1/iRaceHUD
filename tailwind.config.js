/** @type {import('tailwindcss').Config} */
export default {
  content: ['./src/**/*.{html,js,svelte,ts}'],
  theme: {
    extend: {},
  },
  plugins: [
    require('daisyui'),
    require("@designbycode/tailwindcss-stripes"),
  ],
  daisyui: {
    base: false,
    themes: ["bumblebee"],
  },
}
