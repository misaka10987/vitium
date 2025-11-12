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
          label: 'Web 应用',
          autogenerate: { directory: 'app' },
        },
        {
          label: '客户端',
          autogenerate: { directory: 'client' },
        },
        {
          label: '服务端',
          autogenerate: { directory: 'server' },
        },
        {
          label: '模组',
          autogenerate: { directory: 'mod' },
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
