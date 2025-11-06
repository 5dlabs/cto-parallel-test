import React, { useState, useEffect } from 'react';
import { Link } from 'react-router-dom';
import { Card, CardContent, CardDescription, CardFooter, CardHeader, CardTitle } from './ui/card';
import { Button } from './ui/button';
import { Badge } from './ui/badge';
import { ShoppingCart } from 'lucide-react';

function ProductList() {
  const [products, setProducts] = useState([]);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    // Simulate API call - in real implementation, this would fetch from /api/products
    setTimeout(() => {
      const mockProducts = [
        {
          id: 1,
          name: 'Wireless Headphones',
          description: 'Premium noise-cancelling headphones with 30-hour battery life',
          price: 199.99,
          inventory_count: 25
        },
        {
          id: 2,
          name: 'Smart Watch',
          description: 'Fitness tracker with heart rate monitor and GPS',
          price: 299.99,
          inventory_count: 15
        },
        {
          id: 3,
          name: 'Laptop Backpack',
          description: 'Durable water-resistant backpack with USB charging port',
          price: 49.99,
          inventory_count: 50
        },
        {
          id: 4,
          name: 'Mechanical Keyboard',
          description: 'RGB backlit mechanical keyboard with blue switches',
          price: 129.99,
          inventory_count: 30
        },
        {
          id: 5,
          name: 'Wireless Mouse',
          description: 'Ergonomic wireless mouse with precision tracking',
          price: 39.99,
          inventory_count: 40
        },
        {
          id: 6,
          name: 'USB-C Hub',
          description: '7-in-1 USB-C hub with HDMI, USB 3.0, and SD card reader',
          price: 59.99,
          inventory_count: 0
        }
      ];
      setProducts(mockProducts);
      setLoading(false);
    }, 500);
  }, []);

  const addToCart = (product) => {
    const cart = JSON.parse(localStorage.getItem('cart') || '[]');
    const existingItem = cart.find(item => item.id === product.id);

    if (existingItem) {
      existingItem.quantity += 1;
    } else {
      cart.push({ ...product, quantity: 1 });
    }

    localStorage.setItem('cart', JSON.stringify(cart));

    // Trigger a custom event to update cart count in header
    window.dispatchEvent(new Event('cartUpdated'));

    alert('Product added to cart!');
  };

  if (loading) {
    return (
      <div className="text-center py-12">
        <p className="text-gray-600">Loading products...</p>
      </div>
    );
  }

  return (
    <div className="space-y-8">
      <div>
        <h1 className="text-3xl font-bold text-gray-900 mb-2">Our Products</h1>
        <p className="text-gray-600">Browse our wide selection of quality products</p>
      </div>

      <div className="grid sm:grid-cols-2 lg:grid-cols-3 gap-6">
        {products.map((product) => (
          <Card key={product.id} className="flex flex-col">
            <CardHeader>
              <div className="flex justify-between items-start">
                <CardTitle className="text-lg">{product.name}</CardTitle>
                {product.inventory_count === 0 && (
                  <Badge variant="destructive">Out of Stock</Badge>
                )}
                {product.inventory_count > 0 && product.inventory_count < 20 && (
                  <Badge variant="secondary">Low Stock</Badge>
                )}
              </div>
              <CardDescription className="line-clamp-2">
                {product.description}
              </CardDescription>
            </CardHeader>
            <CardContent className="flex-grow">
              <div className="text-2xl font-bold text-primary">
                ${product.price.toFixed(2)}
              </div>
              <div className="text-sm text-gray-500 mt-1">
                {product.inventory_count > 0
                  ? `${product.inventory_count} in stock`
                  : 'Out of stock'}
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
                disabled={product.inventory_count === 0}
                onClick={() => addToCart(product)}
              >
                <ShoppingCart className="mr-2 h-4 w-4" />
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
