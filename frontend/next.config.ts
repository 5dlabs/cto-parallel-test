import type { NextConfig } from "next";

const securityHeaders = [
  { key: "X-Content-Type-Options", value: "nosniff" },
  { key: "Referrer-Policy", value: "no-referrer" },
  { key: "X-Frame-Options", value: "DENY" },
  { key: "Permissions-Policy", value: "geolocation=(), camera=(), microphone=()" },
  // 180 days HSTS; assumes HTTPS in deployment
  { key: "Strict-Transport-Security", value: "max-age=15552000; includeSubDomains" },
  // Mitigate XSS, clickjacking, and data exfiltration risks
  {
    key: "Content-Security-Policy",
    value: [
      "default-src 'self'",
      "script-src 'self'",
      // Disallow inline styles; align with nginx and meta CSP
      "style-src 'self'",
      "img-src 'self' data: blob: https:",
      "font-src 'self' https: data:",
      "connect-src 'self' https:",
      "object-src 'none'",
      "frame-ancestors 'none'",
      "base-uri 'self'",
      "form-action 'self'",
    ].join('; '),
  },
  // Process isolation hardening
  { key: "Cross-Origin-Opener-Policy", value: "same-origin" },
  { key: "Cross-Origin-Resource-Policy", value: "same-origin" },
  { key: "X-Permitted-Cross-Domain-Policies", value: "none" },
];

const nextConfig: NextConfig = {
  // Reduce server fingerprinting
  poweredByHeader: false,
  async headers() {
    return [
      {
        source: "/:path*",
        headers: securityHeaders,
      },
    ];
  },
};

export default nextConfig;
