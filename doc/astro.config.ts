// @ts-check
import { defineConfig } from 'astro/config'
import starlight from '@astrojs/starlight'
import type { ViteUserConfig } from 'astro'

const vitePatch: ViteUserConfig = (() => {
  if (process.env.NODE_ENV != 'development') return {}
  console.debug('Enforce Vite polling as HMR walkaround for bun')
  return {
    server: {
      watch: {
        usePolling: true,
      },
    },
  }
})()

// https://astro.build/config
export default defineConfig({
  integrations: [
    starlight({
      title: 'Vitium Docs',
      locales: {
        root: {
          label: '汉语',
          lang: 'zh-CN',
        },
        en: {
          label: 'English',
          lang: 'en',
        },
      },
      social: [
        {
          icon: 'github',
          label: 'GitHub',
          href: 'https://github.com/withastro/starlight',
        },
      ],
      sidebar: [
        {
          label: '开始',
          autogenerate: { directory: 'start' },
        },
        {
          label: '元文档',
          autogenerate: { directory: 'meta' },
        },
      ],
    }),
  ],
  vite: {
    ...{
      resolve: {
        alias: {
          '@': '/src',
        },
      },
    },
    ...vitePatch,
  },
})
