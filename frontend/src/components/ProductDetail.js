import React, { useState, useEffect } from 'react';
import { useParams, Link } from 'react-router-dom';
import { Card, CardContent, CardDescription, CardFooter, CardHeader, CardTitle } from './ui/card';
import { Button } from './ui/button';
import { Badge } from './ui/badge';
import { Input } from './ui/input';
import { Label } from './ui/label';
import { ArrowLeft } from 'lucide-react';

function ProductDetail() {
  const { id } = useParams();
  const [product, setProduct] = useState(null);
  const [quantity, setQuantity] = useState(1);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    // Fetch product details from API
    const fetchProduct = async () => {
      try {
        // Mock data for demonstration
        const mockProducts = {
          '1': {
            id: 1,
            name: 'Wireless Headphones',
            description: 'Premium noise-cancelling headphones with 30-hour battery life',
            price: 299.99,
            inventory_count: 15,
            features: [
              'Active Noise Cancellation',
              '30-hour battery life',
              'Bluetooth 5.0',
              'Comfortable over-ear design',
              'Built-in microphone'
            ]
          },
          '2': {
            id: 2,
            name: 'Smart Watch',
            description: 'Advanced fitness tracking and smart notifications',
            price: 199.99,
            inventory_count: 8,
            features: [
              'Heart rate monitoring',
              'GPS tracking',
              'Water resistant',
              'Sleep tracking',
              '7-day battery life'
            ]
          },
          '3': {
            id: 3,
            name: 'Laptop Stand',
            description: 'Ergonomic aluminum laptop stand for better posture',
            price: 49.99,
            inventory_count: 25,
            features: [
              'Aluminum construction',
              'Adjustable height',
              'Cable management',
              'Supports up to 17" laptops',
              'Non-slip rubber pads'
            ]
          }
        };

        setTimeout(() => {
          setProduct(mockProducts[id] || mockProducts['1']);
          setLoading(false);
        }, 300);
      } catch (err) {
        setLoading(false);
      }
    };

    fetchProduct();
  }, [id]);

  if (loading) {
    return (
      <div className="container mx-auto px-4 py-12">
        <div className="text-center">Loading product details...</div>
      </div>
    );
  }

  if (!product) {
    return (
      <div className="container mx-auto px-4 py-12">
        <div className="text-center">
          <p className="text-muted-foreground mb-4">Product not found</p>
          <Link to="/products">
            <Button>Back to Products</Button>
          </Link>
        </div>
      </div>
    );
  }

  return (
    <div className="container mx-auto px-4 py-12">
      <Link to="/products">
        <Button variant="ghost" className="mb-6">
          <ArrowLeft className="mr-2 h-4 w-4" />
          Back to Products
        </Button>
      </Link>

      <div className="grid grid-cols-1 lg:grid-cols-2 gap-8">
        <div className="bg-muted rounded-lg aspect-square flex items-center justify-center">
          <p className="text-muted-foreground">Product Image</p>
        </div>

        <div>
          <div className="mb-4">
            <h1 className="text-4xl font-bold mb-2">{product.name}</h1>
            {product.inventory_count === 0 && (
              <Badge variant="destructive">Out of Stock</Badge>
            )}
            {product.inventory_count > 0 && product.inventory_count < 10 && (
              <Badge variant="secondary">Low Stock</Badge>
            )}
          </div>

          <p className="text-5xl font-bold mb-6">${product.price.toFixed(2)}</p>

          <Card className="mb-6">
            <CardHeader>
              <CardTitle>Product Description</CardTitle>
            </CardHeader>
            <CardContent>
              <p className="text-muted-foreground">{product.description}</p>
            </CardContent>
          </Card>

          {product.features && (
            <Card className="mb-6">
              <CardHeader>
                <CardTitle>Features</CardTitle>
              </CardHeader>
              <CardContent>
                <ul className="list-disc list-inside space-y-2">
                  {product.features.map((feature, index) => (
                    <li key={index} className="text-muted-foreground">
                      {feature}
                    </li>
                  ))}
                </ul>
              </CardContent>
            </Card>
          )}

          <Card>
            <CardContent className="pt-6">
              <div className="space-y-4">
                <div>
                  <Label htmlFor="quantity">Quantity</Label>
                  <Input
                    id="quantity"
                    type="number"
                    min="1"
                    max={product.inventory_count}
                    value={quantity}
                    onChange={(e) => setQuantity(Math.max(1, parseInt(e.target.value) || 1))}
                    className="mt-2"
                    disabled={product.inventory_count === 0}
                  />
                  <p className="text-sm text-muted-foreground mt-2">
                    {product.inventory_count > 0
                      ? `${product.inventory_count} available`
                      : 'Currently unavailable'}
                  </p>
                </div>

                <Button
                  size="lg"
                  className="w-full"
                  disabled={product.inventory_count === 0}
                >
                  Add to Cart
                </Button>
              </div>
            </CardContent>
          </Card>
        </div>
      </div>
    </div>
  );
}

export default ProductDetail;
