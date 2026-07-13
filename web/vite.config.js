import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig } from 'vite';

const API_PROXY = { target: 'http://localhost:3000', changeOrigin: true };

export default defineConfig({
  plugins: [sveltekit()],
  server: {
    proxy: {
      '/auth': API_PROXY,
      '/categories': API_PROXY,
      '/orders': API_PROXY,
      '/products': API_PROXY,
      '/users': API_PROXY,
    },
  },
});
