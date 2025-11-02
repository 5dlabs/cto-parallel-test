import React from 'react';
import { Link } from 'react-router-dom';
import { Button } from './ui/button';
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from './ui/card';

export function HomePage() {
  return (
    <div className="container mx-auto px-4 py-12">
      <div className="text-center mb-12">
        <h1 className="text-4xl font-bold mb-4">Welcome to Our E-Commerce Store</h1>
        <p className="text-xl text-muted-foreground mb-8">
          Discover amazing products at great prices
        </p>
        <Link to="/products">
          <Button size="lg">Browse Products</Button>
        </Link>
      </div>

      <div className="grid grid-cols-1 md:grid-cols-3 gap-6 mt-16">
        <Card>
          <CardHeader>
            <CardTitle>Quality Products</CardTitle>
            <CardDescription>
              Curated selection of high-quality items
            </CardDescription>
          </CardHeader>
          <CardContent>
            <p className="text-sm text-muted-foreground">
              We source the best products from trusted suppliers to ensure your satisfaction.
            </p>
          </CardContent>
        </Card>

        <Card>
          <CardHeader>
            <CardTitle>Fast Shipping</CardTitle>
            <CardDescription>
              Quick delivery to your doorstep
            </CardDescription>
          </CardHeader>
          <CardContent>
            <p className="text-sm text-muted-foreground">
              Get your orders delivered quickly with our reliable shipping partners.
            </p>
          </CardContent>
        </Card>

        <Card>
          <CardHeader>
            <CardTitle>Secure Checkout</CardTitle>
            <CardDescription>
              Safe and secure payment processing
            </CardDescription>
          </CardHeader>
          <CardContent>
            <p className="text-sm text-muted-foreground">
              Shop with confidence using our secure payment gateway.
            </p>
          </CardContent>
        </Card>
      </div>
    </div>
  );
}
