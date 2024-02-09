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
      '^/(article.*|add|categories|clean|files|load|promote)': {
        target: 'http://localhost:5000/'
      }
    }
  }
})
