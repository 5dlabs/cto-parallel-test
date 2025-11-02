import React from 'react';
import { Link } from 'react-router-dom';
import { Button } from './ui/button';
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from './ui/card';
import { ShoppingBag, Truck, Shield, CreditCard } from 'lucide-react';

const HomePage = () => {
  const features = [
    {
      icon: <ShoppingBag className="h-10 w-10 text-primary" />,
      title: 'Wide Selection',
      description: 'Browse through our extensive catalog of quality products',
    },
    {
      icon: <Truck className="h-10 w-10 text-primary" />,
      title: 'Fast Shipping',
      description: 'Quick and reliable delivery to your doorstep',
    },
    {
      icon: <Shield className="h-10 w-10 text-primary" />,
      title: 'Secure Shopping',
      description: 'Your data and transactions are safe with us',
    },
    {
      icon: <CreditCard className="h-10 w-10 text-primary" />,
      title: 'Easy Payments',
      description: 'Multiple payment options for your convenience',
    },
  ];

  return (
    <div className="min-h-screen">
      {/* Hero Section */}
      <section className="bg-gradient-to-r from-primary to-blue-600 text-white py-20">
        <div className="container mx-auto px-4 text-center">
          <h1 className="text-4xl md:text-6xl font-bold mb-6">
            Welcome to E-Shop
          </h1>
          <p className="text-xl md:text-2xl mb-8 text-blue-100">
            Your one-stop destination for quality products
          </p>
          <Link to="/products">
            <Button size="lg" variant="secondary" className="text-lg px-8">
              Shop Now
            </Button>
          </Link>
        </div>
      </section>

      {/* Features Section */}
      <section className="py-16 bg-gray-50">
        <div className="container mx-auto px-4">
          <h2 className="text-3xl font-bold text-center mb-12">
            Why Choose E-Shop?
          </h2>
          <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6">
            {features.map((feature, index) => (
              <Card key={index} className="text-center hover:shadow-lg transition-shadow">
                <CardHeader>
                  <div className="flex justify-center mb-4">
                    {feature.icon}
                  </div>
                  <CardTitle className="text-xl">{feature.title}</CardTitle>
                </CardHeader>
                <CardContent>
                  <CardDescription>{feature.description}</CardDescription>
                </CardContent>
              </Card>
            ))}
          </div>
        </div>
      </section>

      {/* CTA Section */}
      <section className="py-16 bg-white">
        <div className="container mx-auto px-4 text-center">
          <h2 className="text-3xl font-bold mb-4">
            Ready to Start Shopping?
          </h2>
          <p className="text-gray-600 mb-8 text-lg">
            Explore our collection and find your perfect products
          </p>
          <Link to="/products">
            <Button size="lg" className="text-lg px-8">
              Browse Products
            </Button>
          </Link>
        </div>
      </section>
    </div>
  );
};

export default HomePage;
