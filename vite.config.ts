import { defineConfig } from 'vite'
import { svelte } from '@sveltejs/vite-plugin-svelte'

// https://vitejs.dev/config/
export default defineConfig({
  build: {
    outDir: "app/static"
  },
  plugins: [svelte()],
  server: {
    proxy: {
      '^/(files|api).*': {
        target: 'http://localhost:5000/'
      }
    }
  }
})
