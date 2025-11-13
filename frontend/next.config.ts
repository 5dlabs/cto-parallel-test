import type { NextConfig } from "next";

const nextConfig: NextConfig = {
  // Remove the X-Powered-By header to avoid information disclosure.
  poweredByHeader: false,
  // Add security headers for all routes in production builds.
  // Note: Some headers (e.g., HSTS) are most effective when set at the edge/proxy.
  async headers() {
    return [
      {
        source: "/:path*",
        headers: [
          { key: "X-Frame-Options", value: "DENY" },
          { key: "X-Content-Type-Options", value: "nosniff" },
          { key: "Referrer-Policy", value: "no-referrer" },
          // HSTS is safe on HTTPS deployments; proxies may also set this.
          { key: "Strict-Transport-Security", value: "max-age=63072000; includeSubDomains; preload" },
          // Limit powerful features by default; expand as needed.
          { key: "Permissions-Policy", value: "geolocation=(), camera=(), microphone=()" },
          // Basic CSP to mitigate XSS and related attacks.
          // Note: Adjust if you add third-party resources.
          {
            key: "Content-Security-Policy",
            value: [
              "default-src 'self'",
              "base-uri 'self'",
              "form-action 'self'",
              "frame-ancestors 'none'",
              "object-src 'none'",
              "script-src 'self'",
              "style-src 'self' 'unsafe-inline'",
              "img-src 'self' data:",
              "font-src 'self' data:",
              // Allow websockets for Next.js dev server/HMR
              "connect-src 'self' ws: wss:",
              "upgrade-insecure-requests",
            ].join('; '),
          },
        ],
      },
    ];
  },
};

export default nextConfig;
