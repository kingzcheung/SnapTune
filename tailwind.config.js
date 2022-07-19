/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [],
  content: [
    "./index.html",
    "./src/**/*.{vue,js,ts,jsx,tsx}",
  ],
  theme: {
    extend: {},
  },
  plugins: [
    require("daisyui"),
  ],
  daisyui: {
    themes: [
      {
        mytheme: {
          "primary": "#3f2fce",
          "secondary": "#50acd3",
          "accent": "#d1974b",
          "neutral": "#282735",
          "base-100": "#F6F7F8",
          "info": "#46B8E2",
          "success": "#1AAD66",
          "warning": "#BCA015",
          "error": "#FB4C3C",
        },
      },
    ],
  },
  darkMode: 'class',
}
