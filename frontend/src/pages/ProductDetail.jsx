import { useEffect, useState } from "react"
import { useParams, Link } from "react-router-dom"
import { Card, CardHeader, CardTitle, CardContent, CardFooter } from "@/components/ui/card"
import { Button } from "@/components/ui/button"
import { CONFIG } from "@/config"

export default function ProductDetail() {
  const { id } = useParams()
  const [product, setProduct] = useState(null)
  const [error, setError] = useState("")

  useEffect(() => {
    const controller = new AbortController()
    async function load() {
      setError("")
      try {
        const res = await fetch(`${CONFIG.apiBaseUrl}/products/${encodeURIComponent(id)}`, { signal: controller.signal })
        if (!res.ok) throw new Error(`Failed: ${res.status}`)
        const data = await res.json()
        setProduct(data)
      } catch (e) {
        if (e.name !== 'AbortError') setError("Could not load product")
      }
    }
    load()
    return () => controller.abort()
  }, [id])

  if (error) return <div className="container py-8"><p className="text-destructive">{error}</p></div>
  if (!product) return <div className="container py-8">Loading…</div>

  return (
    <div className="container py-8">
      <Card>
        <CardHeader>
          <CardTitle>{product.name}</CardTitle>
        </CardHeader>
        <CardContent>
          <p>{product.description}</p>
          <p className="mt-4 text-xl font-semibold">${product.price}</p>
        </CardContent>
        <CardFooter className="gap-2">
          <Button variant="secondary">Add to Cart</Button>
          <Button asChild>
            <Link to="/products">Back</Link>
          </Button>
        </CardFooter>
      </Card>
    </div>
  )
}

