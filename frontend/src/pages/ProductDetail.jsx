import { useParams } from 'react-router-dom'
import { useEffect, useMemo, useState } from 'react'
import { Card, CardHeader, CardTitle, CardContent } from '@/components/ui/card'
import { Button } from '@/components/ui/button'
import { useCart } from '@/context/CartContext'
import { API_BASE_URL, apiUrl } from '@/config'
import { safeImageSrc } from '@/lib/utils'

function safeId(raw) {
  // Allow simple ids: alphanum, dash, underscore. Prevent path traversal.
  const id = String(raw || '')
  if (!/^[-_a-zA-Z0-9]+$/.test(id)) return ''
  return id
}

export default function ProductDetail() {
  const params = useParams()
  const id = useMemo(() => safeId(params.id), [params.id])
  const [product, setProduct] = useState(null)
  const [loading, setLoading] = useState(!!(API_BASE_URL && id))
  const [error, setError] = useState('')
  const { addItem } = useCart()

  useEffect(() => {
    let abort = new AbortController()
    async function load() {
      if (!API_BASE_URL || !id) return
      setLoading(true)
      setError('')
      try {
        const res = await fetch(apiUrl(`products/${id}`), { signal: abort.signal })
        if (!res.ok) throw new Error(`HTTP ${res.status}`)
        setProduct(await res.json())
      } catch (e) {
        if (e.name !== 'AbortError') setError('Failed to load product')
      } finally {
        setLoading(false)
      }
    }
    load()
    return () => abort.abort()
  }, [id])

  if (!id) {
    return <p className="text-red-600">Invalid product id.</p>
  }

  if (!API_BASE_URL) {
    return <p className="text-muted-foreground">Set VITE_API_BASE_URL to view product details.</p>
  }

  if (loading) return <p>Loading...</p>
  if (error) return <p className="text-red-600">{error}</p>

  if (!product) return <p>Product not found.</p>

  return (
    <Card>
      <CardHeader>
        <CardTitle>{product.title}</CardTitle>
      </CardHeader>
      <CardContent className="grid md:grid-cols-2 gap-6">
        {product.image && (
          <img src={safeImageSrc(product.image)} alt={String(product.title || 'Product')} className="w-full max-h-96 object-contain" loading="lazy" />
        )}
        <div className="space-y-4">
          <p className="text-lg font-semibold">${product.price}</p>
          <p className="text-muted-foreground">{product.description}</p>
          <Button onClick={() => addItem({ id: product.id, title: product.title, price: product.price })}>Add to Cart</Button>
        </div>
      </CardContent>
    </Card>
  )
}
