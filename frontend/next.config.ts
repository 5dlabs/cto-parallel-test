import type { NextConfig } from "next";

// Derive a tight connect-src from configured API base URL when available.
// Falls back to allowing HTTPS anywhere if not set to avoid breaking local usage.
function connectSrc(): string {
  const raw = process.env.NEXT_PUBLIC_API_BASE_URL || process.env.VITE_API_BASE_URL || "";
  try {
    const u = new URL(String(raw));
    if (!/^https?:$/.test(u.protocol)) return "connect-src 'self' https:";
    const origin = `${u.protocol}//${u.host}`;
    return `connect-src 'self' ${origin}`;
  } catch {
    return "connect-src 'self' https:";
  }
}

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
      // Disallow inline styles; all styles must be served from self
      "style-src 'self'",
      "img-src 'self' data: blob: https:",
      "font-src 'self' https: data:",
      connectSrc(),
      "object-src 'none'",
      "frame-ancestors 'none'",
      "base-uri 'self'",
      "form-action 'self'",
    ].join('; '),
  },
  // Process isolation hardening
  { key: "Cross-Origin-Opener-Policy", value: "same-origin" },
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
