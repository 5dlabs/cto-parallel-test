import React, { useState, useEffect } from 'react';
import { Link } from 'react-router-dom';
import { Card, CardContent, CardDescription, CardFooter, CardHeader, CardTitle } from './ui/card';
import { Button } from './ui/button';
import { Badge } from './ui/badge';
import axios from 'axios';

function ProductList() {
  const [products, setProducts] = useState([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState(null);

  useEffect(() => {
    // Fetch products from API
    const fetchProducts = async () => {
      try {
        // In a real app, this would fetch from the backend API
        // For now, using mock data to demonstrate the UI
        const apiUrl = process.env.REACT_APP_API_URL || 'http://localhost:8080/api';

        // Simulate API call with mock data for demonstration
        setTimeout(() => {
          setProducts([
            {
              id: 1,
              name: 'Wireless Headphones',
              description: 'Premium noise-cancelling headphones',
              price: 299.99,
              inventory_count: 15
            },
            {
              id: 2,
              name: 'Smart Watch',
              description: 'Fitness tracking and notifications',
              price: 199.99,
              inventory_count: 8
            },
            {
              id: 3,
              name: 'Laptop Stand',
              description: 'Ergonomic aluminum stand',
              price: 49.99,
              inventory_count: 25
            },
            {
              id: 4,
              name: 'USB-C Hub',
              description: '7-in-1 multiport adapter',
              price: 79.99,
              inventory_count: 0
            },
            {
              id: 5,
              name: 'Mechanical Keyboard',
              description: 'RGB backlit gaming keyboard',
              price: 149.99,
              inventory_count: 12
            },
            {
              id: 6,
              name: 'Wireless Mouse',
              description: 'Ergonomic design with precision tracking',
              price: 59.99,
              inventory_count: 20
            }
          ]);
          setLoading(false);
        }, 500);

        // Real implementation would be:
        // const response = await axios.get(`${apiUrl}/products`);
        // setProducts(response.data);
        // setLoading(false);
      } catch (err) {
        setError('Failed to load products');
        setLoading(false);
      }
    };

    fetchProducts();
  }, []);

  if (loading) {
    return (
      <div className="container mx-auto px-4 py-12">
        <div className="text-center">Loading products...</div>
      </div>
    );
  }

  if (error) {
    return (
      <div className="container mx-auto px-4 py-12">
        <div className="text-center text-destructive">{error}</div>
      </div>
    );
  }

  return (
    <div className="container mx-auto px-4 py-12">
      <h1 className="text-4xl font-bold mb-8">Our Products</h1>

      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
        {products.map((product) => (
          <Card key={product.id} className="flex flex-col">
            <CardHeader>
              <div className="flex items-start justify-between">
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
              <p className="text-3xl font-bold">${product.price.toFixed(2)}</p>
              <p className="text-sm text-muted-foreground mt-2">
                {product.inventory_count > 0
                  ? `${product.inventory_count} in stock`
                  : 'Currently unavailable'}
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
