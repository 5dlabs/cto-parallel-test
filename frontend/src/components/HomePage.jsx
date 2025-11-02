import React from 'react';
import { Link } from 'react-router-dom';
import { Button } from './ui/button';
import { Card, CardHeader, CardTitle, CardDescription } from './ui/card';
import { ShoppingBag, TrendingUp, Shield } from 'lucide-react';

function HomePage() {
  return (
    <div className="space-y-12">
      {/* Hero Section */}
      <section className="text-center space-y-6 py-12">
        <h1 className="text-4xl md:text-6xl font-bold tracking-tight">
          Welcome to Our Store
        </h1>
        <p className="text-xl text-muted-foreground max-w-2xl mx-auto">
          Discover amazing products at great prices. Shop with confidence and enjoy fast delivery.
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
            <ShoppingBag className="h-10 w-10 text-primary mb-2" />
            <CardTitle>Wide Selection</CardTitle>
            <CardDescription>
              Browse thousands of products across multiple categories
            </CardDescription>
          </CardHeader>
        </Card>

        <Card>
          <CardHeader>
            <TrendingUp className="h-10 w-10 text-primary mb-2" />
            <CardTitle>Best Prices</CardTitle>
            <CardDescription>
              Competitive pricing and regular deals on your favorite items
            </CardDescription>
          </CardHeader>
        </Card>

        <Card>
          <CardHeader>
            <Shield className="h-10 w-10 text-primary mb-2" />
            <CardTitle>Secure Shopping</CardTitle>
            <CardDescription>
              Safe and secure checkout with multiple payment options
            </CardDescription>
          </CardHeader>
        </Card>
      </section>

      {/* Call to Action */}
      <section className="bg-muted rounded-lg p-8 text-center space-y-4">
        <h2 className="text-3xl font-bold">Ready to Start Shopping?</h2>
        <p className="text-muted-foreground">
          Create an account today and get access to exclusive deals.
        </p>
        <Link to="/register">
          <Button size="lg">
            Get Started
          </Button>
        </Link>
      </section>
    </div>
  );
}

export default HomePage;
