import React from 'react';
import { Link } from 'react-router-dom';
import { Card, CardContent, CardFooter, CardHeader, CardTitle } from './ui/card';
import { Button } from './ui/button';

export function Cart() {
  const cartItems = [
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
  ];

  const total = cartItems.reduce((sum, item) => sum + (item.price * item.quantity), 0);

  if (cartItems.length === 0) {
    return (
      <div className="container mx-auto px-4 py-8">
        <h1 className="text-3xl font-bold mb-8">Shopping Cart</h1>
        <Card>
          <CardContent className="py-12 text-center">
            <p className="text-muted-foreground mb-4">Your cart is empty</p>
            <Link to="/products">
              <Button>Browse Products</Button>
            </Link>
          </CardContent>
        </Card>
      </div>
    );
  }

  return (
    <div className="container mx-auto px-4 py-8">
      <h1 className="text-3xl font-bold mb-8">Shopping Cart</h1>
      
      <div className="grid grid-cols-1 lg:grid-cols-3 gap-8">
        <div className="lg:col-span-2 space-y-4">
          {cartItems.map((item) => (
            <Card key={item.id}>
              <CardContent className="py-6">
                <div className="flex items-center justify-between">
                  <div className="flex-1">
                    <Link to={"/products/" + item.productId}>
                      <h3 className="font-semibold hover:text-primary">{item.name}</h3>
                    </Link>
                    <p className="text-sm text-muted-foreground">{"$" + item.price}</p>
                  </div>
                  
                  <div className="flex items-center gap-4">
                    <div className="flex items-center gap-2">
                      <Button variant="outline" size="sm">-</Button>
                      <span className="w-8 text-center">{item.quantity}</span>
                      <Button variant="outline" size="sm">+</Button>
                    </div>
                    
                    <p className="font-bold w-24 text-right">{"$" + (item.price * item.quantity).toFixed(2)}</p>
                    
                    <Button variant="destructive" size="sm">Remove</Button>
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
                <span>{"$" + total.toFixed(2)}</span>
              </div>
              <div className="flex justify-between">
                <span>Shipping</span>
                <span>Free</span>
              </div>
              <div className="border-t pt-4">
                <div className="flex justify-between font-bold text-lg">
                  <span>Total</span>
                  <span>{"$" + total.toFixed(2)}</span>
                </div>
              </div>
            </CardContent>
            <CardFooter>
              <Button className="w-full" size="lg">Proceed to Checkout</Button>
            </CardFooter>
          </Card>
        </div>
      </div>
    </div>
  );
}
