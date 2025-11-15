import { clsx, type ClassValue } from "clsx";
import { twMerge } from "tailwind-merge";

export function cn(...inputs: ClassValue[]) {
  return twMerge(clsx(inputs));
}

// Return a safe image src allowing only relative paths, http(s), data, and blob URLs.
// Blocks dangerous schemes like javascript: to prevent XSS via image src attributes.
export function safeImageSrc(raw: unknown): string {
  const s = String(raw || '').trim()
  if (!s) return ''
  if (s.startsWith('/')) return s
  if (s.startsWith('http://') || s.startsWith('https://')) return s
  if (s.startsWith('data:') || s.startsWith('blob:')) return s
  return ''
}
