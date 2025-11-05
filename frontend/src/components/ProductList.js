import React, { useState, useEffect } from 'react';
import { Link } from 'react-router-dom';
import { Card, CardContent, CardFooter, CardHeader, CardTitle } from './ui/card';
import { Button } from './ui/button';
import { Badge } from './ui/badge';
import { ShoppingCart } from 'lucide-react';
import axios from 'axios';

function ProductList() {
  const [products, setProducts] = useState([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState(null);

  useEffect(() => {
    fetchProducts();
  }, []);

  const fetchProducts = async () => {
    try {
      // In production, this would fetch from the actual API
      // For now, we'll use mock data
      const mockProducts = [
        {
          id: 1,
          name: 'Wireless Headphones',
          description: 'High-quality wireless headphones with noise cancellation',
          price: 99.99,
          inventory_count: 15
        },
        {
          id: 2,
          name: 'Smartphone Stand',
          description: 'Adjustable aluminum stand for all smartphones',
          price: 24.99,
          inventory_count: 50
        },
        {
          id: 3,
          name: 'USB-C Cable',
          description: 'Durable braided USB-C cable, 6ft length',
          price: 12.99,
          inventory_count: 100
        },
        {
          id: 4,
          name: 'Laptop Sleeve',
          description: 'Protective sleeve for 13-15 inch laptops',
          price: 29.99,
          inventory_count: 30
        },
        {
          id: 5,
          name: 'Wireless Mouse',
          description: 'Ergonomic wireless mouse with precision tracking',
          price: 34.99,
          inventory_count: 45
        },
        {
          id: 6,
          name: 'Portable Charger',
          description: '10000mAh portable battery pack with fast charging',
          price: 39.99,
          inventory_count: 20
        }
      ];

      // Simulate API call delay
      await new Promise(resolve => setTimeout(resolve, 500));

      setProducts(mockProducts);
      setLoading(false);
    } catch (err) {
      setError('Failed to load products');
      setLoading(false);
    }
  };

  const addToCart = (product) => {
    // In production, this would call the API
    // For now, just show an alert
    alert(`Added ${product.name} to cart!`);

    // Update cart count in localStorage
    const currentCount = parseInt(localStorage.getItem('cartItemCount') || '0');
    localStorage.setItem('cartItemCount', (currentCount + 1).toString());
  };

  if (loading) {
    return (
      <div className="container mx-auto px-4 py-12">
        <div className="text-center">
          <p className="text-lg text-muted-foreground">Loading products...</p>
        </div>
      </div>
    );
  }

  if (error) {
    return (
      <div className="container mx-auto px-4 py-12">
        <div className="text-center">
          <p className="text-lg text-destructive">{error}</p>
        </div>
      </div>
    );
  }

  return (
    <div className="container mx-auto px-4 py-12">
      <div className="mb-8">
        <h1 className="text-4xl font-bold mb-2">Our Products</h1>
        <p className="text-muted-foreground">Browse our collection of quality products</p>
      </div>

      <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-6">
        {products.map((product) => (
          <Card key={product.id} className="flex flex-col">
            <CardHeader>
              <div className="flex justify-between items-start">
                <CardTitle className="text-xl">{product.name}</CardTitle>
                {product.inventory_count < 10 && (
                  <Badge variant="destructive">Low Stock</Badge>
                )}
              </div>
            </CardHeader>
            <CardContent className="flex-grow">
              <p className="text-muted-foreground mb-4">{product.description}</p>
              <p className="text-2xl font-bold text-primary">${product.price.toFixed(2)}</p>
              <p className="text-sm text-muted-foreground mt-2">
                {product.inventory_count} in stock
              </p>
            </CardContent>
            <CardFooter className="flex gap-2">
              <Link to={`/products/${product.id}`} className="flex-1">
                <Button variant="outline" className="w-full">
                  View Details
                </Button>
              </Link>
              <Button
                className="flex-1"
                onClick={() => addToCart(product)}
                disabled={product.inventory_count === 0}
              >
                <ShoppingCart className="h-4 w-4 mr-2" />
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
