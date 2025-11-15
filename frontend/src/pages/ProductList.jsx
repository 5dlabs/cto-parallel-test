import { useEffect, useState } from 'react'
import { Link } from 'react-router-dom'
import { Card, CardHeader, CardTitle, CardContent } from '@/components/ui/card'
import { Badge } from '@/components/ui/badge'
import { Input } from '@/components/ui/input'
import { API_BASE_URL, apiUrl } from '@/config'
import { safeImageSrc } from '@/lib/utils'

export default function ProductList() {
  const [products, setProducts] = useState([])
  const [loading, setLoading] = useState(!!API_BASE_URL)
  const [error, setError] = useState('')
  const [query, setQuery] = useState('')

  useEffect(() => {
    let abort = new AbortController()
    async function load() {
      if (!API_BASE_URL) return
      setLoading(true)
      setError('')
      try {
        const url = apiUrl('products')
        const res = await fetch(url, { signal: abort.signal })
        if (!res.ok) throw new Error(`HTTP ${res.status}`)
        const data = await res.json()
        // Expect array of {id,title,price,image}
        setProducts(Array.isArray(data) ? data : [])
      } catch (e) {
        if (e.name !== 'AbortError') setError('Failed to load products')
      } finally {
        setLoading(false)
      }
    }
    load()
    return () => abort.abort()
  }, [])

  const filtered = products.filter((p) =>
    p?.title?.toLowerCase().includes(query.toLowerCase())
  )

  return (
    <div className="space-y-4">
      <div className="flex items-center gap-2">
        <Input
          placeholder="Search products..."
          value={query}
          onChange={(e) => setQuery(e.target.value)}
          aria-label="Search products"
        />
        {!API_BASE_URL && (
          <Badge variant="secondary">Set VITE_API_BASE_URL to load products</Badge>
        )}
      </div>
      {loading && <p>Loading...</p>}
      {error && <p className="text-red-600">{error}</p>}
      <div className="grid sm:grid-cols-2 lg:grid-cols-3 gap-4">
        {(API_BASE_URL ? filtered : []).map((p) => (
          <Card key={p.id} className="flex flex-col">
            <CardHeader>
              <CardTitle className="line-clamp-1" title={p.title}>{p.title}</CardTitle>
            </CardHeader>
            <CardContent className="flex-1 flex flex-col">
              {p.image && (
                <img src={safeImageSrc(p.image)} alt={String(p.title || 'Product')} className="h-40 object-contain mx-auto mb-3" loading="lazy" />
              )}
              <div className="mt-auto flex items-center justify-between">
                <span className="font-semibold">${p.price}</span>
                <Link to={`/products/${encodeURIComponent(String(p.id))}`} className="text-primary">
                  View
                </Link>
              </div>
            </CardContent>
          </Card>
        ))}
      </div>
    </div>
  )
}
