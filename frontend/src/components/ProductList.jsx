import React from 'react';
import { Link } from 'react-router-dom';
import { Card, CardContent, CardDescription, CardFooter, CardHeader, CardTitle } from './ui/card';
import { Button } from './ui/button';
import { Badge } from './ui/badge';

const ProductList = () => {
  // Mock product data - will be replaced with API calls later
  const products = [
    { id: 1, name: 'Premium Headphones', description: 'High-quality wireless headphones', price: 199.99, inventory_count: 15 },
    { id: 2, name: 'Smart Watch', description: 'Feature-rich smartwatch', price: 299.99, inventory_count: 8 },
    { id: 3, name: 'Laptop Stand', description: 'Ergonomic aluminum stand', price: 49.99, inventory_count: 25 },
    { id: 4, name: 'Wireless Mouse', description: 'Precision wireless mouse', price: 39.99, inventory_count: 30 },
    { id: 5, name: 'USB-C Hub', description: 'Multi-port USB-C adapter', price: 59.99, inventory_count: 12 },
    { id: 6, name: 'Mechanical Keyboard', description: 'RGB backlit keyboard', price: 149.99, inventory_count: 0 },
  ];

  return (
    <div className="container mx-auto px-4 py-8">
      <div className="mb-8">
        <h1 className="text-4xl font-bold mb-2">Products</h1>
        <p className="text-muted-foreground">Browse our collection of quality products</p>
      </div>

      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
        {products.map((product) => (
          <Card key={product.id} className="flex flex-col">
            <CardHeader>
              <div className="flex justify-between items-start">
                <CardTitle>{product.name}</CardTitle>
                {product.inventory_count === 0 && (
                  <Badge variant="destructive">Out of Stock</Badge>
                )}
                {product.inventory_count > 0 && product.inventory_count < 10 && (
                  <Badge variant="secondary">Low Stock</Badge>
                )}
              </div>
              <CardDescription>{product.description}</CardDescription>
            </CardHeader>
            <CardContent className="flex-grow">
              <p className="text-3xl font-bold text-primary">${product.price}</p>
              <p className="text-sm text-muted-foreground mt-2">
                {product.inventory_count > 0 ? `${product.inventory_count} in stock` : 'Out of stock'}
              </p>
            </CardContent>
            <CardFooter className="flex gap-2">
              <Link to={`/products/${product.id}`} className="flex-1">
                <Button variant="outline" className="w-full">View Details</Button>
              </Link>
              <Button 
                className="flex-1" 
                disabled={product.inventory_count === 0}
              >
                Add to Cart
              </Button>
            </CardFooter>
          </Card>
        ))}
      </div>
    </div>
  );
};

export default ProductList;
