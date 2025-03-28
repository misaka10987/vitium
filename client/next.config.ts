import type { NextConfig } from "next"
import { add } from 'vitium-api'

const path = require('path')

const nextConfig: NextConfig = {
  // output: "export",
  /* config options here */
  webpack: (config, options) => {
    config.experiments.asyncWebAssembly = true
    return config
  }
}

export default nextConfig
