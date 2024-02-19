/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
    "./src/**/*.{html,rs}",
    "./index.html"
    ],
  theme: {
    extend: {
      fontFamily: {
        'mplus': ["'M PLUS Rounded 1c'", 'sans-serif'],
        'text': ["-apple-system","BlinkMacSystemFont","Segoe UI","Helvetica","Arial","sans-serif","Apple Color Emoji","Segoe UI Emoji","Segoe UI Symbol"]
      }
    },
  },
  plugins: [],
}

