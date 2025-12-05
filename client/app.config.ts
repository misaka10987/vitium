import { defineConfig } from '@solidjs/start/config'
import tailwindcss from '@tailwindcss/vite'

export default defineConfig({
  server: {
    preset: 'cloudflare-module',
    cloudflare: {
      deployConfig: true,
    },
  },
  vite: {
    resolve: {
      alias: {
        '~': '/src',
      },
    },
    plugins: [tailwindcss()],
  },
})
