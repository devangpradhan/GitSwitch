/** @type {import('tailwindcss').Config} */
export default {
  content: [
    "./index.html",
    "./src/**/*.{svelte,js,ts,jsx,tsx}",
  ],
  darkMode: "class",
  theme: {
    extend: {
      colors: {
        background: '#090e15',
        surface: {
          DEFAULT: '#0b141c',
          dim: '#0b141c',
          bright: '#313a43',
          container: '#182028',
          lowest: '#060f16',
          low: '#141c24',
          high: '#222b33',
          highest: '#2d363e',
        },
        primary: {
          DEFAULT: '#3fb950',
          hover: '#2ea043',
          container: '#3fb950',
          fixed: '#83fc89',
        },
        secondary: {
          DEFAULT: '#a2c9ff',
          container: '#0071c7',
        },
        tertiary: {
          DEFAULT: '#d8baff',
          container: '#bb8bfe',
        },
        terminal: {
          bg: '#080e14',
          surface: '#0d151e',
          border: '#1b2734',
          hover: '#15212e',
          accent: '#3fb950',
          accentHover: '#2ea043',
          cyan: '#38bdf8',
          amber: '#d29922',
          dim: '#637081',
          text: '#e6edf3',
        },
        outline: {
          DEFAULT: '#879484',
          variant: '#3e4a3c',
        },
        error: {
          DEFAULT: '#ffb4ab',
          container: '#93000a',
        }
      },
      fontFamily: {
        sans: ['Geist', 'Inter', 'system-ui', 'sans-serif'],
        mono: ['"JetBrains Mono"', '"Fira Code"', 'ui-monospace', 'monospace'],
      },
      boxShadow: {
        'neon': '0 0 25px -2px rgba(63, 185, 80, 0.55), 0 0 60px -8px rgba(63, 185, 80, 0.25)',
        'indigo-neon': '0 0 25px -2px rgba(99, 102, 241, 0.55), 0 0 60px -8px rgba(99, 102, 241, 0.25)',
      }
    },
  },
  plugins: [],
}
