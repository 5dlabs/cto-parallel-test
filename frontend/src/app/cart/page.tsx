"use client";

import Link from "next/link";
import { Button } from "@/components/ui/button";
import { Card, CardContent, CardFooter, CardHeader, CardTitle } from "@/components/ui/card";
import { Badge } from "@/components/ui/badge";
import { ShoppingBag, Trash2 } from "lucide-react";

interface CartItem {
  id: number;
  productId: number;
  name: string;
  price: number;
  quantity: number;
}

export default function CartPage() {
  // In a real app, this would come from global state or context
  const cartItems: CartItem[] = [];
  
  const subtotal = cartItems.reduce(
    (sum, item) => sum + item.price * item.quantity,
    0
  );
  const tax = subtotal * 0.1; // 10% tax
  const total = subtotal + tax;

  if (cartItems.length === 0) {
    return (
      <div className="container mx-auto px-4 py-8 md:px-6">
        <h1 className="mb-8 text-3xl font-bold md:text-4xl">Shopping Cart</h1>
        <Card className="mx-auto max-w-md">
          <CardContent className="flex flex-col items-center justify-center py-12">
            <ShoppingBag className="h-16 w-16 text-muted-foreground" />
            <h2 className="mt-4 text-xl font-semibold">Your cart is empty</h2>
            <p className="mt-2 text-center text-muted-foreground">
              Add some products to your cart to see them here.
            </p>
            <Link href="/products" className="mt-6">
              <Button>Browse Products</Button>
            </Link>
          </CardContent>
        </Card>
      </div>
    );
  }

  return (
    <div className="container mx-auto px-4 py-8 md:px-6">
      <h1 className="mb-8 text-3xl font-bold md:text-4xl">Shopping Cart</h1>

      <div className="grid gap-8 lg:grid-cols-3">
        {/* Cart Items */}
        <div className="space-y-4 lg:col-span-2">
          {cartItems.map((item) => (
            <Card key={item.id}>
              <CardHeader>
                <div className="flex items-start justify-between">
                  <div>
                    <CardTitle className="text-xl">{item.name}</CardTitle>
                    <p className="mt-1 text-sm text-muted-foreground">
                      ${item.price.toFixed(2)} each
                    </p>
                  </div>
                  <Button variant="ghost" size="icon" aria-label="Remove item">
                    <Trash2 className="h-4 w-4 text-destructive" />
                  </Button>
                </div>
              </CardHeader>
              <CardContent>
                <div className="flex items-center justify-between">
                  <div className="flex items-center gap-2">
                    <Button variant="outline" size="sm">
                      -
                    </Button>
                    <Badge variant="secondary" className="px-4 py-1">
                      {item.quantity}
                    </Badge>
                    <Button variant="outline" size="sm">
                      +
                    </Button>
                  </div>
                  <span className="text-xl font-bold">
                    ${(item.price * item.quantity).toFixed(2)}
                  </span>
                </div>
              </CardContent>
            </Card>
          ))}
        </div>

        {/* Order Summary */}
        <div className="lg:col-span-1">
          <Card className="sticky top-20">
            <CardHeader>
              <CardTitle>Order Summary</CardTitle>
            </CardHeader>
            <CardContent className="space-y-4">
              <div className="flex justify-between">
                <span className="text-muted-foreground">Subtotal</span>
                <span className="font-medium">${subtotal.toFixed(2)}</span>
              </div>
              <div className="flex justify-between">
                <span className="text-muted-foreground">Tax (10%)</span>
                <span className="font-medium">${tax.toFixed(2)}</span>
              </div>
              <div className="border-t pt-4">
                <div className="flex justify-between">
                  <span className="text-lg font-semibold">Total</span>
                  <span className="text-lg font-bold">${total.toFixed(2)}</span>
                </div>
              </div>
            </CardContent>
            <CardFooter className="flex-col gap-2">
              <Button className="w-full" size="lg">
                Proceed to Checkout
              </Button>
              <Link href="/products" className="w-full">
                <Button variant="outline" className="w-full">
                  Continue Shopping
                </Button>
              </Link>
            </CardFooter>
          </Card>
        </div>
      </div>
    </div>
  );
}
