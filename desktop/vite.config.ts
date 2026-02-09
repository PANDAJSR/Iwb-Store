import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react'

// https://vite.dev/config/
export default defineConfig({
  plugins: [react()],
  // Tauri 需要配置清空 outDir
  build: {
    outDir: 'dist',
    emptyOutDir: true,
  },
  server: {
    port: 1420,
    strictPort: false, // 如果端口被占用，自动选择其他端口
  },
  envPrefix: ['VITE_', 'TAURI_'],
})
