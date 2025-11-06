import React from 'react';
import { Link } from 'react-router-dom';
import { Button } from './ui/button';
import { Card, CardHeader, CardTitle, CardDescription, CardContent } from './ui/card';

const HomePage = () => {
  const features = [
    {
      title: 'Wide Selection',
      description: 'Browse through thousands of quality products',
      icon: '🛍️',
    },
    {
      title: 'Fast Shipping',
      description: 'Get your orders delivered quickly and safely',
      icon: '🚚',
    },
    {
      title: 'Secure Payment',
      description: 'Shop with confidence using secure payment methods',
      icon: '🔒',
    },
    {
      title: '24/7 Support',
      description: 'Our customer service team is always here to help',
      icon: '💬',
    },
  ];

  return (
    <div className="container mx-auto px-4 py-12">
      {/* Hero Section */}
      <div className="text-center mb-16">
        <h1 className="text-4xl md:text-6xl font-bold mb-6">
          Welcome to Our Store
        </h1>
        <p className="text-xl text-muted-foreground mb-8 max-w-2xl mx-auto">
          Discover amazing products at unbeatable prices. Start shopping today and experience the best in online retail.
        </p>
        <div className="flex gap-4 justify-center">
          <Link to="/products">
            <Button size="lg">
              Shop Now
            </Button>
          </Link>
          <Link to="/register">
            <Button size="lg" variant="outline">
              Create Account
            </Button>
          </Link>
        </div>
      </div>

      {/* Features Section */}
      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6">
        {features.map((feature, index) => (
          <Card key={index}>
            <CardHeader>
              <div className="text-4xl mb-2">{feature.icon}</div>
              <CardTitle className="text-xl">{feature.title}</CardTitle>
              <CardDescription>{feature.description}</CardDescription>
            </CardHeader>
          </Card>
        ))}
      </div>

      {/* Call to Action */}
      <div className="mt-16 text-center">
        <Card className="max-w-2xl mx-auto">
          <CardHeader>
            <CardTitle className="text-3xl">Ready to Start Shopping?</CardTitle>
            <CardDescription className="text-lg">
              Join thousands of satisfied customers and find everything you need in one place.
            </CardDescription>
          </CardHeader>
          <CardContent>
            <Link to="/products">
              <Button size="lg" className="w-full md:w-auto">
                Browse Products
              </Button>
            </Link>
          </CardContent>
        </Card>
      </div>
    </div>
  );
};

export default HomePage;
