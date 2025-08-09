/** @type {import('next').NextConfig} */
const nextConfig = {
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
  basePath: '',
}

module.exports = nextConfig