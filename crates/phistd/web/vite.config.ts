import { defineConfig } from 'vite';
import react from '@vitejs/plugin-react';
import phidoc from '@phiace/phidoc/vite';
import siteConfig from './phidoc.config';

export default defineConfig({
  plugins: [react(), phidoc(siteConfig)],
  resolve: {
    dedupe: ['react', 'react-dom', '@phiace/puijs'],
  },
  server: {
    host: '127.0.0.1',
    port: 5183,
    fs: {
      allow: ['..', '../..'],
    },
  },
  build: {
    outDir: 'dist',
  },
});
