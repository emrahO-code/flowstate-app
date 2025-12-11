/** @type {import('tailwindcss').Config} */
module.exports = {
  // This tells Tailwind to scan files in the public folder for classes
  content: [
    "./public/index.html",
    "./src/**/*.js",
  ],
  theme: {
    extend: {},
  },
  plugins: [],
}