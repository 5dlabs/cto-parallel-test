import React, { useState } from 'react'
import { Link } from 'react-router-dom'
import { Button } from './ui/button'
import { Card, CardContent, CardDescription, CardFooter, CardHeader, CardTitle } from './ui/card'
import { Trash2, Plus, Minus, ShoppingBag } from 'lucide-react'

const Cart = () => {
  // Mock cart items for demonstration
  // In a real implementation, this would come from state management or API
  const [cartItems, setCartItems] = useState([
    {
      id: 1,
      productId: 1,
      name: 'Wireless Headphones',
      price: 99.99,
      quantity: 2,
    },
    {
      id: 2,
      productId: 3,
      name: 'Laptop Stand',
      price: 49.99,
      quantity: 1,
    },
  ])

  const formatPrice = (price) => {
    return new Intl.NumberFormat('en-US', {
      style: 'currency',
      currency: 'USD',
    }).format(price)
  }

  const updateQuantity = (itemId, delta) => {
    setCartItems((items) =>
      items.map((item) =>
        item.id === itemId
          ? { ...item, quantity: Math.max(1, item.quantity + delta) }
          : item
      )
    )
  }

  const removeItem = (itemId) => {
    setCartItems((items) => items.filter((item) => item.id !== itemId))
  }

  const clearCart = () => {
    setCartItems([])
  }

  const subtotal = cartItems.reduce((sum, item) => sum + item.price * item.quantity, 0)
  const tax = subtotal * 0.1 // 10% tax
  const total = subtotal + tax

  if (cartItems.length === 0) {
    return (
      <div className="container mx-auto px-4 py-8">
        <Card className="max-w-2xl mx-auto">
          <CardContent className="py-12 text-center">
            <ShoppingBag className="h-16 w-16 mx-auto mb-4 text-muted-foreground" />
            <h2 className="text-2xl font-bold mb-2">Your cart is empty</h2>
            <p className="text-muted-foreground mb-6">
              Add some products to your cart to get started!
            </p>
            <Link to="/products">
              <Button size="lg">Browse Products</Button>
            </Link>
          </CardContent>
        </Card>
      </div>
    )
  }

  return (
    <div className="container mx-auto px-4 py-8">
      <h1 className="text-4xl font-bold tracking-tight mb-8">Shopping Cart</h1>

      <div className="grid grid-cols-1 lg:grid-cols-3 gap-8">
        {/* Cart Items */}
        <div className="lg:col-span-2 space-y-4">
          {cartItems.map((item) => (
            <Card key={item.id}>
              <CardContent className="p-6">
                <div className="flex items-start gap-4">
                  {/* Product Image Placeholder */}
                  <div className="w-24 h-24 bg-muted rounded-md flex items-center justify-center flex-shrink-0">
                    <ShoppingBag className="h-8 w-8 text-muted-foreground" />
                  </div>

                  {/* Product Info */}
                  <div className="flex-grow">
                    <div className="flex justify-between items-start mb-2">
                      <div>
                        <Link
                          to={`/products/${item.productId}`}
                          className="font-semibold text-lg hover:text-primary transition-colors"
                        >
                          {item.name}
                        </Link>
                        <p className="text-muted-foreground">
                          {formatPrice(item.price)} each
                        </p>
                      </div>
                      <Button
                        variant="ghost"
                        size="icon"
                        onClick={() => removeItem(item.id)}
                      >
                        <Trash2 className="h-4 w-4 text-destructive" />
                      </Button>
                    </div>

                    <div className="flex items-center justify-between mt-4">
                      <div className="flex items-center border rounded-md">
                        <Button
                          variant="ghost"
                          size="sm"
                          onClick={() => updateQuantity(item.id, -1)}
                          disabled={item.quantity <= 1}
                        >
                          <Minus className="h-4 w-4" />
                        </Button>
                        <span className="px-4 py-2 min-w-[3rem] text-center">
                          {item.quantity}
                        </span>
                        <Button
                          variant="ghost"
                          size="sm"
                          onClick={() => updateQuantity(item.id, 1)}
                        >
                          <Plus className="h-4 w-4" />
                        </Button>
                      </div>
                      <div className="text-lg font-semibold">
                        {formatPrice(item.price * item.quantity)}
                      </div>
                    </div>
                  </div>
                </div>
              </CardContent>
            </Card>
          ))}

          <Button variant="outline" onClick={clearCart} className="w-full">
            Clear Cart
          </Button>
        </div>

        {/* Order Summary */}
        <div className="lg:col-span-1">
          <Card className="sticky top-20">
            <CardHeader>
              <CardTitle>Order Summary</CardTitle>
              <CardDescription>
                {cartItems.length} {cartItems.length === 1 ? 'item' : 'items'} in cart
              </CardDescription>
            </CardHeader>
            <CardContent className="space-y-4">
              <div className="flex justify-between">
                <span className="text-muted-foreground">Subtotal</span>
                <span className="font-medium">{formatPrice(subtotal)}</span>
              </div>
              <div className="flex justify-between">
                <span className="text-muted-foreground">Tax (10%)</span>
                <span className="font-medium">{formatPrice(tax)}</span>
              </div>
              <div className="border-t pt-4">
                <div className="flex justify-between text-lg font-bold">
                  <span>Total</span>
                  <span>{formatPrice(total)}</span>
                </div>
              </div>
            </CardContent>
            <CardFooter className="flex flex-col gap-2">
              <Button className="w-full" size="lg">
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
    </div>
  )
}

export default Cart
