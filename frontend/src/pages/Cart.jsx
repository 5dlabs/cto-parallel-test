import { Button } from "@/components/ui/button"
import { Card, CardContent, CardFooter, CardHeader, CardTitle } from "@/components/ui/card"

export default function Cart() {
  // Placeholder cart; in a real app, connect to state/store.
  const items = []
  const total = 0
  return (
    <div className="container py-8">
      <Card>
        <CardHeader>
          <CardTitle>Your Cart</CardTitle>
        </CardHeader>
        <CardContent>
          {items.length === 0 ? (
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
          <span className="font-semibold">Total: ${total}</span>
          <Button disabled={items.length === 0}>Checkout</Button>
        </CardFooter>
      </Card>
    </div>
  )
}

