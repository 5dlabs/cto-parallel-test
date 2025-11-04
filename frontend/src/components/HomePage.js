import React from 'react';
import { Link } from 'react-router-dom';
import { ArrowRight, ShoppingBag, Truck, Shield } from 'lucide-react';
import { Button } from './ui/button';
import { Card, CardContent } from './ui/card';

function HomePage() {
  const features = [
    {
      icon: <ShoppingBag className="h-8 w-8 text-primary" />,
      title: 'Wide Selection',
      description: 'Browse thousands of products across multiple categories',
    },
    {
      icon: <Truck className="h-8 w-8 text-primary" />,
      title: 'Fast Delivery',
      description: 'Get your orders delivered quickly and reliably',
    },
    {
      icon: <Shield className="h-8 w-8 text-primary" />,
      title: 'Secure Shopping',
      description: 'Shop with confidence with our secure payment system',
    },
  ];

  return (
    <div className="space-y-12">
      {/* Hero Section */}
      <section className="text-center space-y-6 py-12">
        <h1 className="text-4xl md:text-6xl font-bold text-foreground">
          Welcome to E-Commerce
        </h1>
        <p className="text-xl text-muted-foreground max-w-2xl mx-auto">
          Discover amazing products at great prices. Shop from our curated collection
          and enjoy a seamless shopping experience.
        </p>
        <div className="flex flex-col sm:flex-row gap-4 justify-center">
          <Link to="/products">
            <Button size="lg" className="w-full sm:w-auto">
              Shop Now <ArrowRight className="ml-2 h-4 w-4" />
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
      <section className="py-12">
        <h2 className="text-3xl font-bold text-center mb-8">Why Shop With Us?</h2>
        <div className="grid grid-cols-1 md:grid-cols-3 gap-6">
          {features.map((feature, index) => (
            <Card key={index} className="text-center">
              <CardContent className="pt-6 space-y-4">
                <div className="flex justify-center">{feature.icon}</div>
                <h3 className="text-xl font-semibold">{feature.title}</h3>
                <p className="text-muted-foreground">{feature.description}</p>
              </CardContent>
            </Card>
          ))}
        </div>
      </section>

      {/* CTA Section */}
      <section className="bg-primary/10 rounded-lg p-8 md:p-12 text-center">
        <h2 className="text-3xl font-bold mb-4">Ready to Start Shopping?</h2>
        <p className="text-muted-foreground mb-6 max-w-2xl mx-auto">
          Join thousands of satisfied customers and start your shopping journey today.
        </p>
        <Link to="/products">
          <Button size="lg">
            Browse Products <ArrowRight className="ml-2 h-4 w-4" />
          </Button>
        </Link>
      </section>
    </div>
  );
}

export default HomePage;
