import React, { useState, useEffect } from 'react';
import { Link } from 'react-router-dom';
import { Card, CardHeader, CardTitle, CardDescription, CardContent, CardFooter } from './ui/card';
import { Button } from './ui/button';
import { Badge } from './ui/badge';

const sampleProducts = [
  {
    id: 1,
    name: 'Wireless Headphones',
    description: 'Premium noise-canceling wireless headphones',
    price: 129.99,
    inventory_count: 15
  },
  {
    id: 2,
    name: 'Smart Watch',
    description: 'Fitness tracking smartwatch with heart rate monitor',
    price: 199.99,
    inventory_count: 8
  },
  {
    id: 3,
    name: 'Laptop Stand',
    description: 'Ergonomic aluminum laptop stand',
    price: 49.99,
    inventory_count: 25
  },
  {
    id: 4,
    name: 'Mechanical Keyboard',
    description: 'RGB mechanical gaming keyboard',
    price: 89.99,
    inventory_count: 0
  },
  {
    id: 5,
    name: 'USB-C Hub',
    description: '7-in-1 USB-C multiport adapter',
    price: 39.99,
    inventory_count: 30
  },
  {
    id: 6,
    name: 'Wireless Mouse',
    description: 'Ergonomic wireless mouse with precision tracking',
    price: 29.99,
    inventory_count: 20
  }
];

function ProductList() {
  const [products, setProducts] = useState([]);

  useEffect(() => {
    setProducts(sampleProducts);
  }, []);

  const formatPrice = (price) => price.toFixed(2);

  return (
    <div className="container mx-auto px-4 py-8">
      <h1 className="text-4xl font-bold mb-8">Our Products</h1>
      
      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
        {products.map((product) => (
          <Card key={product.id} className="flex flex-col">
            <CardHeader>
              <div className="flex justify-between items-start">
                <CardTitle className="text-xl">{product.name}</CardTitle>
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
              <p className="text-3xl font-bold text-primary">
                ${formatPrice(product.price)}
              </p>
              <p className="text-sm text-muted-foreground mt-2">
                {product.inventory_count > 0 
                  ? product.inventory_count + ' in stock'
                  : 'Currently unavailable'
                }
              </p>
            </CardContent>
            <CardFooter className="flex gap-2">
              <Link to={'/products/' + product.id} className="flex-1">
                <Button variant="outline" className="w-full">
                  View Details
                </Button>
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
}

export default ProductList;
