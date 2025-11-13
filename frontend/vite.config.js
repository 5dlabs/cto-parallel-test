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
    host: '127.0.0.1',
    strictPort: true,
    port: Number(process.env.PORT || 3000),
  },
  preview: {
    host: '127.0.0.1',
    strictPort: true,
    port: Number(process.env.PORT || 3000),
  }
}))
