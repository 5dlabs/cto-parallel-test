import React from 'react';
import { Link } from 'react-router-dom';
import { Button } from './ui/button';
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from './ui/card';
import { ShoppingBag, Shield, Truck } from 'lucide-react';

const HomePage = () => {
  return (
    <div className="container mx-auto px-4 py-8">
      {/* Hero Section */}
      <div className="text-center py-16">
        <h1 className="text-5xl font-bold mb-4">Welcome to E-Commerce</h1>
        <p className="text-xl text-muted-foreground mb-8">
          Discover amazing products at great prices
        </p>
        <Link to="/products">
          <Button size="lg" className="text-lg px-8">
            <ShoppingBag className="mr-2 h-5 w-5" />
            Browse Products
          </Button>
        </Link>
      </div>

      {/* Features Section */}
      <div className="grid grid-cols-1 md:grid-cols-3 gap-6 py-8">
        <Card>
          <CardHeader>
            <div className="flex justify-center mb-4">
              <Truck className="h-12 w-12 text-primary" />
            </div>
            <CardTitle className="text-center">Fast Delivery</CardTitle>
            <CardDescription className="text-center">
              Get your orders delivered quickly and safely to your doorstep
            </CardDescription>
          </CardHeader>
        </Card>

        <Card>
          <CardHeader>
            <div className="flex justify-center mb-4">
              <Shield className="h-12 w-12 text-primary" />
            </div>
            <CardTitle className="text-center">Secure Shopping</CardTitle>
            <CardDescription className="text-center">
              Shop with confidence with our secure payment system
            </CardDescription>
          </CardHeader>
        </Card>

        <Card>
          <CardHeader>
            <div className="flex justify-center mb-4">
              <ShoppingBag className="h-12 w-12 text-primary" />
            </div>
            <CardTitle className="text-center">Quality Products</CardTitle>
            <CardDescription className="text-center">
              Carefully curated products from trusted brands
            </CardDescription>
          </CardHeader>
        </Card>
      </div>
    </div>
  );
};

export default HomePage;
