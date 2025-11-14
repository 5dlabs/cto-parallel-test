export const API_BASE_URL = import.meta.env.VITE_API_BASE_URL || ''

export function apiUrl(path) {
  if (!API_BASE_URL) return ''
  // Ensure single slash joining and escape path segments prudently
  const safe = path
    .split('/')
    .filter(Boolean)
    .map((seg) => encodeURIComponent(seg))
    .join('/')
  return `${API_BASE_URL.replace(/\/$/, '')}/${safe}`
}

