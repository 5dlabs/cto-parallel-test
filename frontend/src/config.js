function normalizeBaseUrl(raw) {
  const val = String(raw || '').trim()
  if (!val) return ''
  try {
    const u = new URL(val)
    // Allow only http/https to prevent dangerous schemes
    if (!/^https?:$/.test(u.protocol)) return ''
    // Remove trailing slash for consistent joining
    u.pathname = u.pathname.replace(/\/$/, '')
    return u.toString().replace(/\/$/, '')
  } catch {
    return ''
  }
}

export const API_BASE_URL = normalizeBaseUrl(import.meta.env.VITE_API_BASE_URL)

export function apiUrl(path) {
  if (!API_BASE_URL) return ''
  // Ensure single slash joining and escape path segments prudently
  const safe = String(path)
    .split('/')
    .filter(Boolean)
    .map((seg) => encodeURIComponent(seg))
    .join('/')
  return `${API_BASE_URL}/${safe}`
}
