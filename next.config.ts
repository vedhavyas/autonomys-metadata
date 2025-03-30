/** @type {import('next').NextConfig} */
const isProd = process.env.NODE_ENV === 'production';

const nextConfig = {
  output: 'export',
  images: {
    unoptimized: true,
  },
  basePath: isProd ? '/autonomys-metadata' : '',
  assetPrefix: isProd ? '/autonomys-metadata/' : '',
  trailingSlash: true,
};

module.exports = nextConfig;