import React from 'react';
import { Link } from 'react-router-dom';
import { Button } from './ui/button';
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from './ui/card';
import { ShoppingBag, Package, Zap } from 'lucide-react';

function HomePage() {
  return (
    <div className="space-y-12">
      {/* Hero Section */}
      <section className="text-center space-y-6 py-12">
        <h1 className="text-4xl md:text-6xl font-bold tracking-tight">
          Welcome to E-Store
        </h1>
        <p className="text-xl text-muted-foreground max-w-2xl mx-auto">
          Discover amazing products at unbeatable prices. Your one-stop shop for everything you need.
        </p>
        <div className="flex gap-4 justify-center">
          <Link to="/products">
            <Button size="lg">
              Shop Now
            </Button>
          </Link>
          <Link to="/register">
            <Button size="lg" variant="outline">
              Sign Up
            </Button>
          </Link>
        </div>
      </section>

      {/* Features Section */}
      <section className="grid md:grid-cols-3 gap-6">
        <Card>
          <CardHeader>
            <div className="flex justify-center mb-4">
              <ShoppingBag className="h-12 w-12 text-primary" />
            </div>
            <CardTitle className="text-center">Wide Selection</CardTitle>
          </CardHeader>
          <CardContent>
            <CardDescription className="text-center">
              Browse through thousands of products across multiple categories
            </CardDescription>
          </CardContent>
        </Card>

        <Card>
          <CardHeader>
            <div className="flex justify-center mb-4">
              <Package className="h-12 w-12 text-primary" />
            </div>
            <CardTitle className="text-center">Fast Delivery</CardTitle>
          </CardHeader>
          <CardContent>
            <CardDescription className="text-center">
              Get your orders delivered quickly and securely to your doorstep
            </CardDescription>
          </CardContent>
        </Card>

        <Card>
          <CardHeader>
            <div className="flex justify-center mb-4">
              <Zap className="h-12 w-12 text-primary" />
            </div>
            <CardTitle className="text-center">Easy Checkout</CardTitle>
          </CardHeader>
          <CardContent>
            <CardDescription className="text-center">
              Seamless shopping experience with our simple checkout process
            </CardDescription>
          </CardContent>
        </Card>
      </section>

      {/* CTA Section */}
      <section className="bg-primary text-primary-foreground rounded-lg p-8 md:p-12 text-center">
        <h2 className="text-3xl font-bold mb-4">Ready to Start Shopping?</h2>
        <p className="text-lg mb-6 opacity-90">
          Join thousands of satisfied customers today
        </p>
        <Link to="/products">
          <Button size="lg" variant="secondary">
            Browse Products
          </Button>
        </Link>
      </section>
    </div>
  );
}

export default HomePage;
