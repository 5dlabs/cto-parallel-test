import React from 'react';
import { Link } from 'react-router-dom';
import { Button } from './ui/button';
import { Card, CardHeader, CardTitle, CardDescription, CardContent } from './ui/card';
import { ShoppingBag, Truck, Shield } from 'lucide-react';

function HomePage() {
  return (
    <div className="container mx-auto px-4 py-12">
      {/* Hero Section */}
      <div className="text-center mb-16">
        <h1 className="text-5xl font-bold mb-4">Welcome to E-Shop</h1>
        <p className="text-xl text-muted-foreground mb-8">
          Discover amazing products at unbeatable prices
        </p>
        <Link to="/products">
          <Button size="lg">Shop Now</Button>
        </Link>
      </div>

      {/* Features Section */}
      <div className="grid md:grid-cols-3 gap-6 mb-16">
        <Card>
          <CardHeader>
            <div className="flex justify-center mb-4">
              <ShoppingBag className="h-12 w-12 text-primary" />
            </div>
            <CardTitle className="text-center">Wide Selection</CardTitle>
            <CardDescription className="text-center">
              Thousands of products to choose from
            </CardDescription>
          </CardHeader>
        </Card>

        <Card>
          <CardHeader>
            <div className="flex justify-center mb-4">
              <Truck className="h-12 w-12 text-primary" />
            </div>
            <CardTitle className="text-center">Fast Shipping</CardTitle>
            <CardDescription className="text-center">
              Get your orders delivered quickly
            </CardDescription>
          </CardHeader>
        </Card>

        <Card>
          <CardHeader>
            <div className="flex justify-center mb-4">
              <Shield className="h-12 w-12 text-primary" />
            </div>
            <CardTitle className="text-center">Secure Checkout</CardTitle>
            <CardDescription className="text-center">
              Your payment information is safe
            </CardDescription>
          </CardHeader>
        </Card>
      </div>

      {/* Call to Action */}
      <div className="text-center">
        <h2 className="text-3xl font-bold mb-4">Ready to Start Shopping?</h2>
        <p className="text-muted-foreground mb-6">
          Create an account to enjoy exclusive deals and faster checkout
        </p>
        <div className="flex justify-center space-x-4">
          <Link to="/register">
            <Button size="lg">Sign Up</Button>
          </Link>
          <Link to="/products">
            <Button size="lg" variant="outline">Browse Products</Button>
          </Link>
        </div>
      </div>
    </div>
  );
}

export default HomePage;
