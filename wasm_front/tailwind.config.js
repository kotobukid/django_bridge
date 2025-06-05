/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
    "./index.html",
    "./src/**/*.{rs,html}",
  ],
  theme: {
    extend: {
      colors: {
        'wixoss-white': '#FFFFFF',
        'wixoss-blue': '#0080FF',
        'wixoss-black': '#000000',
        'wixoss-red': '#FF0000',
        'wixoss-green': '#00FF00',
        'wixoss-colorless': '#808080',
      }
    },
  },
  plugins: [],
}