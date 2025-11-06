import React, { useState } from 'react';
import { Link } from 'react-router-dom';
import { Card, CardHeader, CardTitle, CardDescription, CardContent, CardFooter } from './ui/card';
import { Button } from './ui/button';
import { Badge } from './ui/badge';

const ProductList = () => {
  const [products] = useState([
    {
      id: 1,
      name: 'Premium Wireless Headphones',
      description: 'High-quality sound with noise cancellation',
      price: 299.99,
      inventory_count: 25,
      image: '🎧',
    },
    {
      id: 2,
      name: 'Smart Watch Pro',
      description: 'Track your fitness and stay connected',
      price: 399.99,
      inventory_count: 15,
      image: '⌚',
    },
    {
      id: 3,
      name: 'Ultra HD Camera',
      description: 'Capture stunning photos and videos',
      price: 899.99,
      inventory_count: 8,
      image: '📷',
    },
    {
      id: 4,
      name: 'Gaming Laptop',
      description: 'High-performance gaming on the go',
      price: 1499.99,
      inventory_count: 5,
      image: '💻',
    },
    {
      id: 5,
      name: 'Wireless Keyboard',
      description: 'Ergonomic design for comfortable typing',
      price: 79.99,
      inventory_count: 50,
      image: '⌨️',
    },
    {
      id: 6,
      name: 'Bluetooth Speaker',
      description: 'Portable speaker with amazing sound',
      price: 149.99,
      inventory_count: 30,
      image: '🔊',
    },
    {
      id: 7,
      name: 'Fitness Tracker',
      description: 'Monitor your health 24/7',
      price: 99.99,
      inventory_count: 40,
      image: '📱',
    },
    {
      id: 8,
      name: 'USB-C Hub',
      description: 'Expand your connectivity options',
      price: 49.99,
      inventory_count: 100,
      image: '🔌',
    },
  ]);

  return (
    <div className="container mx-auto px-4 py-8">
      <div className="mb-8">
        <h1 className="text-4xl font-bold mb-2">Our Products</h1>
        <p className="text-muted-foreground">Browse our collection of premium products</p>
      </div>

      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-6">
        {products.map((product) => (
          <Card key={product.id} className="flex flex-col">
            <CardHeader>
              <div className="text-6xl mb-4 text-center">{product.image}</div>
              <CardTitle className="text-lg">{product.name}</CardTitle>
              <CardDescription>{product.description}</CardDescription>
            </CardHeader>
            <CardContent className="flex-grow">
              <div className="flex items-center justify-between mb-2">
                <span className="text-2xl font-bold">${product.price}</span>
                <Badge variant={product.inventory_count > 10 ? 'secondary' : 'destructive'}>
                  {product.inventory_count > 10 ? 'In Stock' : 'Low Stock'}
                </Badge>
              </div>
              <p className="text-sm text-muted-foreground">
                {product.inventory_count} available
              </p>
            </CardContent>
            <CardFooter>
              <Link to={`/products/${product.id}`} className="w-full">
                <Button className="w-full">View Details</Button>
              </Link>
            </CardFooter>
          </Card>
        ))}
      </div>
    </div>
  );
};

export default ProductList;
