import type { NextConfig } from 'next';

const nextConfig: NextConfig = {
  output: 'standalone',
  // Enable experimental features for production optimization
  experimental: {
    // Optimize package imports
    optimizePackageImports: ['lucide-react'],
  },
};

export default nextConfig;
