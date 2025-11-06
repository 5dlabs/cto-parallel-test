import React, { useState } from 'react';
import { useParams, Link } from 'react-router-dom';
import { Card, CardHeader, CardTitle, CardDescription, CardContent } from './ui/card';
import { Button } from './ui/button';
import { Badge } from './ui/badge';
import { Input } from './ui/input';
import { Label } from './ui/label';

const ProductDetail = () => {
  const { id } = useParams();
  const [quantity, setQuantity] = useState(1);

  // Mock product data - in a real app, this would come from an API
  const products = {
    1: {
      id: 1,
      name: 'Premium Wireless Headphones',
      description: 'High-quality sound with noise cancellation',
      price: 299.99,
      inventory_count: 25,
      image: '🎧',
      fullDescription: 'Experience crystal-clear audio with our premium wireless headphones featuring active noise cancellation, 30-hour battery life, and comfortable over-ear design. Perfect for music lovers and professionals alike.',
      features: [
        'Active Noise Cancellation',
        '30-hour battery life',
        'Bluetooth 5.0 connectivity',
        'Comfortable over-ear design',
        'Built-in microphone',
        'Foldable design with carrying case',
      ],
    },
    2: {
      id: 2,
      name: 'Smart Watch Pro',
      description: 'Track your fitness and stay connected',
      price: 399.99,
      inventory_count: 15,
      image: '⌚',
      fullDescription: 'Stay connected and track your health with our advanced smartwatch featuring heart rate monitoring, GPS, and water resistance up to 50m.',
      features: [
        'Heart rate monitoring',
        'Built-in GPS',
        'Water resistant (50m)',
        'Sleep tracking',
        'Multiple sport modes',
        '7-day battery life',
      ],
    },
  };

  const product = products[id] || products[1];

  const handleQuantityChange = (e) => {
    const value = parseInt(e.target.value);
    if (value > 0 && value <= product.inventory_count) {
      setQuantity(value);
    }
  };

  const handleAddToCart = () => {
    alert(`Added ${quantity} ${product.name}(s) to cart`);
  };

  return (
    <div className="container mx-auto px-4 py-8">
      <Link to="/products" className="inline-block mb-6">
        <Button variant="ghost">← Back to Products</Button>
      </Link>

      <div className="grid grid-cols-1 lg:grid-cols-2 gap-8">
        {/* Product Image */}
        <div className="flex items-center justify-center bg-muted rounded-lg p-12">
          <div className="text-9xl">{product.image}</div>
        </div>

        {/* Product Details */}
        <div>
          <div className="mb-4">
            <Badge variant={product.inventory_count > 10 ? 'secondary' : 'destructive'}>
              {product.inventory_count > 10 ? 'In Stock' : 'Low Stock'}
            </Badge>
          </div>

          <h1 className="text-4xl font-bold mb-4">{product.name}</h1>
          <p className="text-muted-foreground text-lg mb-6">{product.description}</p>

          <div className="mb-6">
            <span className="text-4xl font-bold">${product.price}</span>
          </div>

          <Card className="mb-6">
            <CardHeader>
              <CardTitle>Product Description</CardTitle>
            </CardHeader>
            <CardContent>
              <p className="mb-4">{product.fullDescription}</p>
              <h4 className="font-semibold mb-2">Features:</h4>
              <ul className="list-disc list-inside space-y-1">
                {product.features.map((feature, index) => (
                  <li key={index} className="text-sm text-muted-foreground">{feature}</li>
                ))}
              </ul>
            </CardContent>
          </Card>

          {/* Add to Cart Section */}
          <div className="space-y-4">
            <div className="flex items-end gap-4">
              <div className="flex-1">
                <Label htmlFor="quantity">Quantity</Label>
                <Input
                  id="quantity"
                  type="number"
                  min="1"
                  max={product.inventory_count}
                  value={quantity}
                  onChange={handleQuantityChange}
                  className="mt-1"
                />
              </div>
              <div className="flex-1">
                <p className="text-sm text-muted-foreground mb-1">Available</p>
                <p className="text-lg font-semibold">{product.inventory_count} units</p>
              </div>
            </div>

            <Button
              className="w-full"
              size="lg"
              onClick={handleAddToCart}
              disabled={product.inventory_count === 0}
            >
              {product.inventory_count === 0 ? 'Out of Stock' : 'Add to Cart'}
            </Button>
          </div>
        </div>
      </div>
    </div>
  );
};

export default ProductDetail;
