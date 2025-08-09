/** @type {import('next').NextConfig} */
const nextConfig = {
  experimental: {
    appDir: true,
  },
  images: {
    domains: [
      'raw.githubusercontent.com',
      'github.com',
      'avatars.githubusercontent.com',
      'img.shields.io',
      'docs.rs',
      'crates.io',
      'tokio.rs',
      'axum.rs',
      'serde.rs'
    ],
  },
  trailingSlash: true,
  output: 'export',
  distDir: 'out',
  basePath: process.env.NODE_ENV === 'production' ? '/omnivore' : '',
}

module.exports = nextConfig