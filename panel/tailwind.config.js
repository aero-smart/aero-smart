/** @type {import('tailwindcss').Config} */
export default {
  content: ['./index.html', './src/**/*.{vue,js,ts,jsx,tsx}'],
  theme: {
    extend: {
      colors: {
        // App Surface & Layout
        background: '#ffffff', // App Surface Background
        surface: '#ececec', // Sidebar & Base Shadow Background

        // Navigation & Interactive
        'nav-group': '#f9f9f9', // Navigation Group Background
        'nav-active': '#efefef', // Active Nav Item Background

        // Text Colors
        'text-primary': '#423d3c',
        'text-secondary': '#666666',

        // Brand & Status
        primary: '#3b82f6',
        secondary: '#64748b',
        success: '#22c55e',
        warning: '#eab308',
        danger: '#ef4444',
      },
    },
  },
  plugins: [],
}
