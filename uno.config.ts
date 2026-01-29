import { defineConfig, presetUno } from 'unocss'

export default defineConfig({
  presets: [
    presetUno(),
  ],
  theme: {
    colors: {
      primary: '#11ad32',
      secondary: '#11ad32d9',
      accent: '#11ad32b3',
      background: '#0b0d12',
      surface: '#0b0d12',
      text: '#1F2937',
      muted: '#6B7280',
    },
    borderRadius: {
      DEFAULT: '0.5rem',
      sm: '0.375rem',
      md: '0.75rem',
      lg: '1rem',
      xl: '1.5rem',
      '2xl': '2rem',
    },
    boxShadow: {
      DEFAULT: '0 1px 3px 0 rgba(0, 0, 0, 0.1), 0 1px 2px 0 rgba(0, 0, 0, 0.06)',
      sm: '0 1px 2px 0 rgba(0, 0, 0, 0.05)',
      md: '0 4px 6px -1px rgba(0, 0, 0, 0.1), 0 2px 4px -1px rgba(0, 0, 0, 0.06)',
      lg: '0 10px 15px -3px rgba(0, 0, 0, 0.1), 0 4px 6px -2px rgba(0, 0, 0, 0.05)',
      xl: '0 20px 25px -5px rgba(0, 0, 0, 0.1), 0 10px 10px -5px rgba(0, 0, 0, 0.04)',
      '2xl': '0 25px 50px -12px rgba(0, 0, 0, 0.25)',
      inner: 'inset 0 2px 4px 0 rgba(0, 0, 0, 0.06)',
      'primary': '0 0 0 3px rgba(17, 173, 50, 0.3)',
      'primary-lg': '0 10px 15px -3px rgba(17, 173, 50, 0.2), 0 4px 6px -2px rgba(17, 173, 50, 0.1)',
    },
  },
  shortcuts: {
    'btn-primary': 'bg-primary cursor-pointer h-12 border-0 text-white text-14px box-border px-4 py-2 rounded-md transition-all shadow-md hover:shadow-primary-lg',
    'btn-secondary': 'bg-white cursor-pointer h-12 text-14px box-border text-primary border border-primary px-4 py-2 rounded-md hover:bg-primary/5 transition-all shadow-sm',
    'card': 'bg-white rounded-xl p-6 shadow-md hover:shadow-lg transition-all',
    'input': 'w-full h-12 text-14px box-border px-4 p2-4 rounded-md outline-none focus:outline-none focus:ring-2 focus:ring-primary focus:border-transparent transition-all',
    'gradient-primary': 'bg-gradient-to-r from-primary to-secondary',
    'gradient-card': 'bg-gradient-to-br from-white to-accent/10',
  },
})