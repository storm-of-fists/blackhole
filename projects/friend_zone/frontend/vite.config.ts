import { defineConfig } from 'vite';
import solidPlugin from 'vite-plugin-solid';
import suidPlugin from "@suid/vite-plugin";

export default defineConfig({
  plugins: [solidPlugin(), suidPlugin()],
  server: {
    port: 8000,
    host: '127.0.0.1'
  },
  build: {
    target: 'esnext',
  },
});
