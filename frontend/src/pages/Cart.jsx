import { Link } from 'react-router-dom'
import { Button } from '@/components/ui/button'
import { Card, CardContent, CardFooter, CardHeader, CardTitle } from '@/components/ui/card'
import { Input } from '@/components/ui/input'

function Cart() {
  // Mock data - in a real app, this would come from state/context
  const cartItems = [
    { id: 1, name: 'Premium Headphones', price: 299.99, quantity: 1 },
    { id: 2, name: 'Smart Watch', price: 199.99, quantity: 2 },
  ]

  const subtotal = cartItems.reduce((sum, item) => sum + item.price * item.quantity, 0)
  const tax = subtotal * 0.1
  const total = subtotal + tax

  return (
    <div className="space-y-6">
      <div>
        <h1 className="text-3xl font-bold tracking-tight">Shopping Cart</h1>
        <p className="text-muted-foreground mt-2">
          {cartItems.length} items in your cart
        </p>
      </div>

      {cartItems.length === 0 ? (
        <Card>
          <CardContent className="p-12 text-center">
            <p className="text-muted-foreground mb-4">Your cart is empty</p>
            <Link to="/products">
              <Button>Continue Shopping</Button>
            </Link>
          </CardContent>
        </Card>
      ) : (
        <div className="grid lg:grid-cols-3 gap-6">
          <div className="lg:col-span-2 space-y-4">
            {cartItems.map((item) => (
              <Card key={item.id}>
                <CardContent className="p-6">
                  <div className="flex items-center justify-between">
                    <div className="flex-grow">
                      <h3 className="font-semibold">{item.name}</h3>
                      <p className="text-sm text-muted-foreground">${item.price}</p>
                    </div>
                    <div className="flex items-center gap-4">
                      <div className="flex items-center gap-2">
                        <Button size="sm" variant="outline">-</Button>
                        <Input
                          type="number"
                          value={item.quantity}
                          className="w-16 text-center"
                          readOnly
                        />
                        <Button size="sm" variant="outline">+</Button>
                      </div>
                      <p className="font-semibold w-24 text-right">
                        ${(item.price * item.quantity).toFixed(2)}
                      </p>
                      <Button size="sm" variant="destructive">
                        Remove
                      </Button>
                    </div>
                  </div>
                </CardContent>
              </Card>
            ))}
          </div>

          <div>
            <Card>
              <CardHeader>
                <CardTitle>Order Summary</CardTitle>
              </CardHeader>
              <CardContent className="space-y-4">
                <div className="flex justify-between">
                  <span>Subtotal</span>
                  <span>${subtotal.toFixed(2)}</span>
                </div>
                <div className="flex justify-between">
                  <span>Tax</span>
                  <span>${tax.toFixed(2)}</span>
                </div>
                <div className="border-t pt-4">
                  <div className="flex justify-between font-bold text-lg">
                    <span>Total</span>
                    <span>${total.toFixed(2)}</span>
                  </div>
                </div>
              </CardContent>
              <CardFooter className="flex flex-col gap-2">
                <Button className="w-full">
                  Proceed to Checkout
                </Button>
                <Link to="/products" className="w-full">
                  <Button variant="outline" className="w-full">
                    Continue Shopping
                  </Button>
                </Link>
              </CardFooter>
            </Card>
          </div>
        </div>
      )}
    </div>
  )
}

export default Cart
