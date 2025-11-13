import { useEffect, useState } from "react"
import { Link } from "react-router-dom"
import { Card, CardHeader, CardTitle, CardContent, CardFooter } from "@/components/ui/card"
import { Button } from "@/components/ui/button"
import { CONFIG } from "@/config"

export default function ProductList() {
  const [products, setProducts] = useState([])
  const [loading, setLoading] = useState(false)
  const [error, setError] = useState("")

  useEffect(() => {
    const controller = new AbortController()
    async function load() {
      setLoading(true)
      setError("")
      try {
        const res = await fetch(`${CONFIG.apiBaseUrl}/products`, { signal: controller.signal })
        if (!res.ok) throw new Error(`Failed: ${res.status}`)
        const data = await res.json()
        if (Array.isArray(data)) setProducts(data)
      } catch (e) {
        if (e.name !== 'AbortError') setError("Could not load products")
      } finally {
        setLoading(false)
      }
    }
    load()
    return () => controller.abort()
  }, [])

  return (
    <div className="container py-8">
      <h2 className="mb-6 text-2xl font-semibold">Products</h2>
      {loading && <p>Loading…</p>}
      {error && <p className="text-destructive">{error}</p>}
      <div className="grid gap-6 sm:grid-cols-2 lg:grid-cols-3">
        {products.map((p) => (
          <Card key={p.id}>
            <CardHeader>
              <CardTitle className="line-clamp-1">{p.name}</CardTitle>
            </CardHeader>
            <CardContent>
              <p className="line-clamp-2 text-sm text-muted-foreground">{p.description}</p>
              <p className="mt-2 font-semibold">${p.price}</p>
            </CardContent>
            <CardFooter className="gap-2">
              <Button asChild>
                <Link to={`/products/${encodeURIComponent(p.id)}`}>View</Link>
              </Button>
              <Button variant="secondary">Add to Cart</Button>
            </CardFooter>
          </Card>
        ))}
      </div>
    </div>
  )
}

