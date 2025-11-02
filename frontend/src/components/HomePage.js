import React from 'react';
import { Link } from 'react-router-dom';
import { Button } from './ui/button';
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from './ui/card';
import { ShoppingBag, Star, Truck } from 'lucide-react';

function HomePage() {
  return (
    <div className="container mx-auto px-4 py-12">
      {/* Hero Section */}
      <div className="text-center mb-16">
        <h1 className="text-4xl md:text-6xl font-bold mb-4">
          Welcome to Our Store
        </h1>
        <p className="text-xl text-muted-foreground mb-8 max-w-2xl mx-auto">
          Discover amazing products at great prices. Shop now and enjoy fast, free shipping on orders over $50.
        </p>
        <Link to="/products">
          <Button size="lg" className="text-lg">
            Shop Now
          </Button>
        </Link>
      </div>

      {/* Features Section */}
      <div className="grid grid-cols-1 md:grid-cols-3 gap-6 mb-16">
        <Card>
          <CardHeader>
            <div className="w-12 h-12 bg-primary/10 rounded-lg flex items-center justify-center mb-4">
              <ShoppingBag className="h-6 w-6 text-primary" />
            </div>
            <CardTitle>Easy Shopping</CardTitle>
            <CardDescription>
              Browse thousands of products with our intuitive interface
            </CardDescription>
          </CardHeader>
        </Card>

        <Card>
          <CardHeader>
            <div className="w-12 h-12 bg-primary/10 rounded-lg flex items-center justify-center mb-4">
              <Truck className="h-6 w-6 text-primary" />
            </div>
            <CardTitle>Fast Delivery</CardTitle>
            <CardDescription>
              Free shipping on orders over $50, delivered to your door
            </CardDescription>
          </CardHeader>
        </Card>

        <Card>
          <CardHeader>
            <div className="w-12 h-12 bg-primary/10 rounded-lg flex items-center justify-center mb-4">
              <Star className="h-6 w-6 text-primary" />
            </div>
            <CardTitle>Quality Products</CardTitle>
            <CardDescription>
              All products are carefully selected for quality and value
            </CardDescription>
          </CardHeader>
        </Card>
      </div>

      {/* CTA Section */}
      <div className="bg-muted rounded-lg p-8 text-center">
        <h2 className="text-3xl font-bold mb-4">Ready to Start Shopping?</h2>
        <p className="text-muted-foreground mb-6 max-w-xl mx-auto">
          Create an account today and get exclusive access to special offers and promotions.
        </p>
        <div className="flex items-center justify-center space-x-4">
          <Link to="/register">
            <Button size="lg">
              Create Account
            </Button>
          </Link>
          <Link to="/products">
            <Button size="lg" variant="outline">
              Browse Products
            </Button>
          </Link>
        </div>
      </div>
    </div>
  );
}

export default HomePage;
