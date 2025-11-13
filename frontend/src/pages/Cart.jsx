import { useEffect, useState } from "react"
import { Button } from "@/components/ui/button"
import { Card, CardContent, CardFooter, CardHeader, CardTitle } from "@/components/ui/card"
import { CONFIG } from "@/config"

export default function Cart() {
  const [items, setItems] = useState([])
  const [loading, setLoading] = useState(false)
  const [error, setError] = useState("")

  useEffect(() => {
    const controller = new AbortController()
    async function load() {
      setLoading(true)
      setError("")
      try {
        const res = await fetch(`${CONFIG.apiBaseUrl}/cart`, { credentials: 'include', signal: controller.signal })
        if (!res.ok) throw new Error(`Failed: ${res.status}`)
        const data = await res.json()
        if (Array.isArray(data?.items)) setItems(data.items)
      } catch (e) {
        if (e.name !== 'AbortError') setError("Could not load cart")
      } finally {
        setLoading(false)
      }
    }
    load()
    return () => controller.abort()
  }, [])

  const total = items.reduce((acc, i) => acc + Number(i.price || 0), 0)

  return (
    <div className="container py-8">
      <Card>
        <CardHeader>
          <CardTitle>Your Cart</CardTitle>
        </CardHeader>
        <CardContent>
          {loading && <p>Loading…</p>}
          {error && <p className="text-destructive">{error}</p>}
          {items.length === 0 && !loading && !error ? (
            <p className="text-muted-foreground">Your cart is empty.</p>
          ) : (
            <ul className="space-y-2">
              {items.map((i) => (
                <li key={i.id} className="flex items-center justify-between">
                  <span>{i.name}</span>
                  <span>${i.price}</span>
                </li>
              ))}
            </ul>
          )}
        </CardContent>
        <CardFooter className="flex items-center justify-between">
          <span className="font-semibold">Total: ${total.toFixed(2)}</span>
          <Button disabled={items.length === 0}>Checkout</Button>
        </CardFooter>
      </Card>
    </div>
  )
}
