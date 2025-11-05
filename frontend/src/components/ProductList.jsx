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
    fetchProducts();
  }, []);

  const fetchProducts = async () => {
    try {
      setLoading(true);
      // Mock data for now - will be replaced with API call
      const mockProducts = [
        {
          id: 1,
          name: 'Wireless Headphones',
          description: 'High-quality wireless headphones with noise cancellation',
          price: 99.99,
          inventory_count: 50
        },
        {
          id: 2,
          name: 'Smart Watch',
          description: 'Feature-rich smartwatch with health tracking',
          price: 199.99,
          inventory_count: 30
        },
        {
          id: 3,
          name: 'Laptop Stand',
          description: 'Ergonomic aluminum laptop stand',
          price: 49.99,
          inventory_count: 100
        },
        {
          id: 4,
          name: 'USB-C Hub',
          description: 'Multi-port USB-C hub with HDMI and USB 3.0',
          price: 39.99,
          inventory_count: 75
        },
        {
          id: 5,
          name: 'Mechanical Keyboard',
          description: 'RGB mechanical keyboard with blue switches',
          price: 129.99,
          inventory_count: 25
        },
        {
          id: 6,
          name: 'Wireless Mouse',
          description: 'Ergonomic wireless mouse with precision tracking',
          price: 59.99,
          inventory_count: 60
        }
      ];

      setProducts(mockProducts);
      setLoading(false);
    } catch (err) {
      setError('Failed to load products');
      setLoading(false);
    }
  };

  const addToCart = (product) => {
    const cart = JSON.parse(localStorage.getItem('cart') || '[]');
    const existingItemIndex = cart.findIndex(item => item.id === product.id);

    if (existingItemIndex >= 0) {
      cart[existingItemIndex].quantity += 1;
    } else {
      cart.push({ ...product, quantity: 1 });
    }

    localStorage.setItem('cart', JSON.stringify(cart));

    // Trigger a custom event to update cart count in header
    window.dispatchEvent(new Event('cartUpdated'));
  };

  if (loading) {
    return (
      <div className="text-center py-12">
        <p className="text-lg text-muted-foreground">Loading products...</p>
      </div>
    );
  }

  if (error) {
    return (
      <div className="text-center py-12">
        <p className="text-lg text-destructive">{error}</p>
      </div>
    );
  }

  return (
    <div className="space-y-6">
      <div className="flex justify-between items-center">
        <h1 className="text-3xl font-bold">Products</h1>
        <p className="text-muted-foreground">{products.length} products available</p>
      </div>

      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
        {products.map((product) => (
          <Card key={product.id} className="flex flex-col">
            <CardHeader>
              <CardTitle>{product.name}</CardTitle>
              <CardDescription>{product.description}</CardDescription>
            </CardHeader>
            <CardContent className="flex-grow">
              <div className="space-y-2">
                <p className="text-2xl font-bold">${product.price.toFixed(2)}</p>
                {product.inventory_count > 0 ? (
                  <Badge variant="secondary">
                    {product.inventory_count} in stock
                  </Badge>
                ) : (
                  <Badge variant="destructive">Out of stock</Badge>
                )}
              </div>
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
