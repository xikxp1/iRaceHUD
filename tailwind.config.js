/** @type {import('tailwindcss').Config} */
export default {
  content: ["./src/**/*.{html,js,svelte,ts}"],
  theme: {
    extend: {},
  },
  plugins: [
    require("daisyui"),
    require("./src/plugins/tailwind-stripes.ts"),
  ],
  daisyui: {
    base: false,
    themes: ["bumblebee"],
  },
}
