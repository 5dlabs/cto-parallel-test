import { Button } from '@/components/ui/button'
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card'
import { useCart } from '@/context/CartContext'

export default function Cart() {
  const { items, removeItem, clear } = useCart()
  const total = items.reduce((acc, it) => acc + (it.price || 0) * (it.quantity || 1), 0)

  return (
    <div className="space-y-4">
      <Card>
        <CardHeader>
          <CardTitle>Your Cart</CardTitle>
        </CardHeader>
        <CardContent className="space-y-3">
          {items.length === 0 && <p className="text-muted-foreground">Your cart is empty.</p>}
          {items.map((it) => (
            <div key={it.id} className="flex items-center justify-between">
              <div>
                <p className="font-medium">{it.title}</p>
                <p className="text-sm text-muted-foreground">Qty: {it.quantity} • ${(it.price || 0).toFixed(2)}</p>
              </div>
              <Button variant="secondary" onClick={() => removeItem(it.id)}>Remove</Button>
            </div>
          ))}
          {items.length > 0 && (
            <div className="flex items-center justify-between pt-4 border-t">
              <p className="font-semibold">Total: ${total.toFixed(2)}</p>
              <Button onClick={clear}>Checkout</Button>
            </div>
          )}
        </CardContent>
      </Card>
    </div>
  )
}

