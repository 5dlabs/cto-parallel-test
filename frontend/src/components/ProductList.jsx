import React, { useState, useEffect } from 'react';
import { Link } from 'react-router-dom';
import { Card, CardHeader, CardTitle, CardDescription, CardContent, CardFooter } from './ui/card';
import { Button } from './ui/button';
import { Badge } from './ui/badge';

function ProductList() {
  const [products, setProducts] = useState([]);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    // In a real app, this would fetch from the API
    // Simulating API call with timeout
    const mockProducts = [
      { id: 1, name: 'Laptop Pro', description: 'High-performance laptop for professionals', price: 1299.99, inventory_count: 15 },
      { id: 2, name: 'Wireless Mouse', description: 'Ergonomic wireless mouse with precision tracking', price: 29.99, inventory_count: 50 },
      { id: 3, name: 'Mechanical Keyboard', description: 'RGB mechanical keyboard with tactile switches', price: 89.99, inventory_count: 30 },
      { id: 4, name: 'USB-C Hub', description: '7-in-1 USB-C hub with HDMI and card reader', price: 49.99, inventory_count: 25 },
      { id: 5, name: 'Webcam HD', description: '1080p HD webcam with built-in microphone', price: 79.99, inventory_count: 20 },
      { id: 6, name: 'Monitor 27"', description: '4K UHD monitor with IPS panel', price: 399.99, inventory_count: 10 },
    ];

    setTimeout(() => {
      setProducts(mockProducts);
      setLoading(false);
    }, 500);
  }, []);

  if (loading) {
    return (
      <div className="text-center py-12">
        <p className="text-muted-foreground">Loading products...</p>
      </div>
    );
  }

  return (
    <div className="space-y-6">
      <div>
        <h1 className="text-3xl font-bold tracking-tight">Products</h1>
        <p className="text-muted-foreground mt-2">Browse our collection of quality products</p>
      </div>

      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
        {products.map((product) => (
          <Card key={product.id} className="flex flex-col">
            <CardHeader>
              <div className="flex justify-between items-start">
                <CardTitle className="text-xl">{product.name}</CardTitle>
                {product.inventory_count < 15 && (
                  <Badge variant="secondary">Low Stock</Badge>
                )}
              </div>
              <CardDescription>{product.description}</CardDescription>
            </CardHeader>
            <CardContent className="flex-grow">
              <p className="text-2xl font-bold text-primary">${product.price.toFixed(2)}</p>
              <p className="text-sm text-muted-foreground mt-1">
                {product.inventory_count} in stock
              </p>
            </CardContent>
            <CardFooter className="flex gap-2">
              <Link to={`/products/${product.id}`} className="flex-1">
                <Button variant="outline" className="w-full">View Details</Button>
              </Link>
              <Button className="flex-1">Add to Cart</Button>
            </CardFooter>
          </Card>
        ))}
      </div>
    </div>
  );
}

export default ProductList;
