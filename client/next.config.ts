import { access, symlink } from 'fs/promises'
import type { NextConfig } from 'next'
import { join } from 'path'
import unTypiaNext from '@ryoppippi/unplugin-typia/next'

const nextConfig: NextConfig = {
  crossOrigin: 'anonymous',
  webpack: (config, { isServer }) => {
    config.experiments.asyncWebAssembly = true

    // to make yargs-parser work in browser
    config.resolve.fallback = { ...config.resolve.fallback, fs: false }

    // walkaround for wasm loading
    // https://github.com/vercel/next.js/issues/25852#issuecomment-1057059000
    const plugin = {
      apply: (compiler: any) => {
        compiler.hooks.afterEmit.tapPromise(
          'SymlinkWebpackPlugin',
          async (compiler: any) => {
            if (!isServer) return

            const from = join(compiler.options.output.path, '../static')
            const to = join(compiler.options.output.path, 'static')

            try {
              await access(from)
              // console.debug(`${from} already exists`)
              return
            } catch (error: any) {
              if (error.code !== 'ENOENT') throw error
            }

            await symlink(to, from, 'junction')
            console.debug(
              'Walkaround applied for WASM loading:',
              `ln -s ${from} ${to}`
            )
          }
        )
      },
    }

    config.plugins.push(plugin)
    return config
  },
}

export default unTypiaNext(nextConfig, {
  cache: false,
  // untypia is producing too many logs
  log: false,
})
