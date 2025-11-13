import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react'

export default defineConfig(() => ({
  plugins: [react()],
  resolve: {
    alias: {
      '@': '/src',
    }
  },
  server: {
    port: Number(process.env.PORT || 3000),
  },
  preview: {
    port: Number(process.env.PORT || 3000),
  }
}))
