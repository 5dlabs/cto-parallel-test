import React from 'react';
import { Link } from 'react-router-dom';
import { Card, CardContent, CardDescription, CardFooter, CardHeader, CardTitle } from './ui/card';
import { Button } from './ui/button';
import { Badge } from './ui/badge';

export function ProductList() {
  const products = [
    {
      id: 1,
      name: 'Wireless Headphones',
      description: 'High-quality wireless headphones with noise cancellation',
      price: 99.99,
      inventory: 15,
    },
    {
      id: 2,
      name: 'Smart Watch',
      description: 'Feature-rich smartwatch with fitness tracking',
      price: 199.99,
      inventory: 8,
    },
    {
      id: 3,
      name: 'Laptop Stand',
      description: 'Ergonomic laptop stand for better posture',
      price: 49.99,
      inventory: 25,
    },
  ];

  return (
    <div className="container mx-auto px-4 py-8">
      <h1 className="text-3xl font-bold mb-8">Our Products</h1>
      
      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
        {products.map((product) => (
          <Card key={product.id} className="flex flex-col">
            <CardHeader>
              <div className="flex items-start justify-between">
                <CardTitle className="text-xl">{product.name}</CardTitle>
                {product.inventory === 0 && (
                  <Badge variant="destructive">Out of Stock</Badge>
                )}
                {product.inventory > 0 && product.inventory < 10 && (
                  <Badge variant="secondary">Low Stock</Badge>
                )}
              </div>
              <CardDescription>{product.description}</CardDescription>
            </CardHeader>
            
            <CardContent className="flex-grow">
              <p className="text-2xl font-bold">${product.price}</p>
              <p className="text-sm text-muted-foreground mt-2">
                {product.inventory} in stock
              </p>
            </CardContent>
            
            <CardFooter className="flex gap-2">
              <Link to={"/products/" + product.id} className="flex-1">
                <Button variant="outline" className="w-full">View Details</Button>
              </Link>
              <Button 
                className="flex-1" 
                disabled={product.inventory === 0}
              >
                Add to Cart
              </Button>
            </CardFooter>
          </Card>
        ))}
      </div>
    </div>
  );
}
