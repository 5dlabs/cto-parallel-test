// Frontend configuration with secure defaults.
// All endpoints and external URLs must be provided via env.

export const CONFIG = {
  apiBaseUrl: import.meta.env.VITE_API_BASE_URL || "/api",
}

