import { defineConfig } from 'vite';
import react from '@vitejs/plugin-react';

// The API is the sla_trainer server (Rust). It only accepts requests whose
// Origin is its own address, so the dev proxy presents itself as that origin.
const API = process.env.SLA_TRAINER ?? 'http://localhost:8770';

export default defineConfig({
  plugins: [react()],
  server: {
    port: 5173,
    strictPort: true,
    host: '127.0.0.1',
    proxy: {
      '/api': {
        target: API,
        changeOrigin: true,
        configure: (proxy) => {
          proxy.on('proxyReq', (req) => req.setHeader('origin', API));
        },
      },
    },
  },
});
