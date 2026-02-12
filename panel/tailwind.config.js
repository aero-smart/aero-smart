/** @type {import('tailwindcss').Config} */
export default {
  content: ['./index.html', './src/**/*.{vue,js,ts,jsx,tsx}'],
  theme: {
    extend: {
      colors: {
        // Semantic Colors mapped to CSS Variables
        primary: {
          DEFAULT: 'var(--color-primary)',
          foreground: 'var(--color-primary-foreground)',
        },
        secondary: {
          DEFAULT: 'var(--color-secondary)',
          foreground: 'var(--color-secondary-foreground)',
        },
        accent: {
          DEFAULT: 'var(--color-accent)',
          foreground: 'var(--color-accent-foreground)',
        },

        // Status Colors
        success: 'var(--color-success)',
        warning: 'var(--color-warning)',
        danger: 'var(--color-danger)',
        info: 'var(--color-info)',

        // Backgrounds
        page: 'var(--bg-page)',
        surface: 'var(--bg-surface)',
        sidebar: 'var(--bg-sidebar)',

        // Text
        'text-primary': 'var(--text-primary)',
        'text-secondary': 'var(--text-secondary)',
        'text-tertiary': 'var(--text-tertiary)',
        'text-inverse': 'var(--text-inverse)',

        // Borders
        border: 'var(--border-color)',
      },
      fontFamily: {
        sans: [
          'Inter',
          '-apple-system',
          'BlinkMacSystemFont',
          'Segoe UI',
          'Roboto',
          'Helvetica Neue',
          'Arial',
          'sans-serif',
        ],
      },
      borderRadius: {
        lg: 'var(--border-radius-lg)',
        xl: 'var(--border-radius-xl)',
      },
      spacing: {
        18: '4.5rem',
        112: '28rem',
        128: '32rem',
      },
      boxShadow: {
        soft: '0 2px 10px rgba(0, 0, 0, 0.03)',
        card: '0 1px 3px rgba(0, 0, 0, 0.05), 0 1px 2px rgba(0, 0, 0, 0.1)',
        float: '0 10px 30px -10px rgba(0, 0, 0, 0.1)',
      },
    },
  },
  plugins: [],
}
