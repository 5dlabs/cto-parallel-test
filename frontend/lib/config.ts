// Unified API config for Next.js runtime
// Parameterized via environment variables; no hardcoded endpoints
// Prefer NEXT_PUBLIC_API_BASE_URL, fallback to VITE_API_BASE_URL for consistency

export const API_BASE_URL: string =
  (process.env.NEXT_PUBLIC_API_BASE_URL || process.env.VITE_API_BASE_URL || '').replace(/\/$/, '')

export function apiUrl(path: string): string {
  if (!API_BASE_URL) return ''
  const safe = String(path)
    .split('/')
    .filter(Boolean)
    .map((seg) => encodeURIComponent(seg))
    .join('/')
  return `${API_BASE_URL}/${safe}`
}

export function safeId(raw: unknown): string {
  const id = String(raw || '')
  return /^[-_a-zA-Z0-9]+$/.test(id) ? id : ''
}

