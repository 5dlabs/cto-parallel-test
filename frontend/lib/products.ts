// Live product accessors (no mocks)
// All endpoints are parameterized via environment
import { API_BASE_URL, apiUrl, safeId } from './config'

export interface Product {
  id: string | number
  title?: string
  name?: string
  price?: number
  category?: string
  inStock?: boolean
  image?: string
  description?: string
  rating?: number
  reviews?: number
}

export async function getAllProducts(): Promise<Product[]> {
  if (!API_BASE_URL) return []
  const url = apiUrl('products')
  const res = await fetch(url, { cache: 'no-store' })
  if (!res.ok) throw new Error(`Failed to load products: ${res.status}`)
  const data = await res.json()
  return Array.isArray(data) ? data : []
}

export async function getProductById(idRaw: unknown): Promise<Product | null> {
  if (!API_BASE_URL) return null
  const id = safeId(idRaw)
  if (!id) return null
  const url = apiUrl(`products/${id}`)
  const res = await fetch(url, { cache: 'no-store' })
  if (!res.ok) return null
  return await res.json()
}
