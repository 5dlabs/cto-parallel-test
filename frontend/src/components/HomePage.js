import React from 'react';
import { Link } from 'react-router-dom';
import { Button } from './ui/button';
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from './ui/card';
import { ShoppingBag, Truck, Shield, Tag } from 'lucide-react';

function HomePage() {
  return (
    <div className="space-y-12">
      {/* Hero Section */}
      <section className="text-center space-y-6 py-12">
        <h1 className="text-4xl md:text-6xl font-bold text-gray-900">
          Welcome to E-Shop
        </h1>
        <p className="text-xl text-gray-600 max-w-2xl mx-auto">
          Discover amazing products at unbeatable prices. Shop the latest trends and enjoy a seamless shopping experience.
        </p>
        <div className="flex flex-col sm:flex-row gap-4 justify-center mt-8">
          <Link to="/products">
            <Button size="lg" className="w-full sm:w-auto">
              <ShoppingBag className="mr-2 h-5 w-5" />
              Browse Products
            </Button>
          </Link>
          <Link to="/register">
            <Button size="lg" variant="outline" className="w-full sm:w-auto">
              Create Account
            </Button>
          </Link>
        </div>
      </section>

      {/* Features Section */}
      <section className="grid md:grid-cols-2 lg:grid-cols-4 gap-6">
        <Card>
          <CardHeader>
            <div className="w-12 h-12 bg-primary/10 rounded-lg flex items-center justify-center mb-2">
              <ShoppingBag className="h-6 w-6 text-primary" />
            </div>
            <CardTitle className="text-lg">Wide Selection</CardTitle>
            <CardDescription>
              Thousands of products across multiple categories
            </CardDescription>
          </CardHeader>
        </Card>

        <Card>
          <CardHeader>
            <div className="w-12 h-12 bg-primary/10 rounded-lg flex items-center justify-center mb-2">
              <Truck className="h-6 w-6 text-primary" />
            </div>
            <CardTitle className="text-lg">Fast Shipping</CardTitle>
            <CardDescription>
              Free delivery on orders over $50
            </CardDescription>
          </CardHeader>
        </Card>

        <Card>
          <CardHeader>
            <div className="w-12 h-12 bg-primary/10 rounded-lg flex items-center justify-center mb-2">
              <Shield className="h-6 w-6 text-primary" />
            </div>
            <CardTitle className="text-lg">Secure Payment</CardTitle>
            <CardDescription>
              Your payment information is always protected
            </CardDescription>
          </CardHeader>
        </Card>

        <Card>
          <CardHeader>
            <div className="w-12 h-12 bg-primary/10 rounded-lg flex items-center justify-center mb-2">
              <Tag className="h-6 w-6 text-primary" />
            </div>
            <CardTitle className="text-lg">Best Prices</CardTitle>
            <CardDescription>
              Competitive prices with regular discounts
            </CardDescription>
          </CardHeader>
        </Card>
      </section>

      {/* Call to Action */}
      <section className="bg-primary/5 rounded-lg p-8 md:p-12 text-center">
        <h2 className="text-3xl font-bold text-gray-900 mb-4">
          Ready to Start Shopping?
        </h2>
        <p className="text-gray-600 mb-6 max-w-xl mx-auto">
          Join thousands of satisfied customers and experience the best online shopping platform.
        </p>
        <Link to="/products">
          <Button size="lg">
            Explore Products
          </Button>
        </Link>
      </section>
    </div>
  );
}

export default HomePage;
