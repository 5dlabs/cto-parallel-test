import React, { useState, useEffect } from 'react';
import { useParams, Link } from 'react-router-dom';
import { Card, CardHeader, CardTitle, CardContent } from './ui/card';
import { Button } from './ui/button';
import { Badge } from './ui/badge';
import { ArrowLeft } from 'lucide-react';

const sampleProducts = [
  {
    id: 1,
    name: 'Wireless Headphones',
    description: 'Premium noise-canceling wireless headphones with superior sound quality',
    price: 129.99,
    inventory_count: 15,
    features: ['Active Noise Cancellation', '30-hour battery life', 'Bluetooth 5.0', 'Foldable design']
  },
  {
    id: 2,
    name: 'Smart Watch',
    description: 'Fitness tracking smartwatch with heart rate monitor',
    price: 199.99,
    inventory_count: 8,
    features: ['Heart rate monitor', 'GPS tracking', 'Water resistant', '7-day battery life']
  },
  {
    id: 3,
    name: 'Laptop Stand',
    description: 'Ergonomic aluminum laptop stand for better posture',
    price: 49.99,
    inventory_count: 25,
    features: ['Aluminum construction', 'Adjustable height', 'Cable management', 'Non-slip pads']
  },
  {
    id: 4,
    name: 'Mechanical Keyboard',
    description: 'RGB mechanical gaming keyboard with customizable keys',
    price: 89.99,
    inventory_count: 0,
    features: ['Mechanical switches', 'RGB backlighting', 'Programmable keys', 'USB passthrough']
  },
  {
    id: 5,
    name: 'USB-C Hub',
    description: '7-in-1 USB-C multiport adapter for laptops',
    price: 39.99,
    inventory_count: 30,
    features: ['4K HDMI output', 'USB 3.0 ports', 'SD card reader', 'Power delivery']
  },
  {
    id: 6,
    name: 'Wireless Mouse',
    description: 'Ergonomic wireless mouse with precision tracking',
    price: 29.99,
    inventory_count: 20,
    features: ['Ergonomic design', '2400 DPI sensor', 'Long battery life', 'Silent clicks']
  }
];

function ProductDetail() {
  const { id } = useParams();
  const [product, setProduct] = useState(null);
  const [quantity, setQuantity] = useState(1);

  useEffect(() => {
    const foundProduct = sampleProducts.find(p => p.id === parseInt(id));
    setProduct(foundProduct);
  }, [id]);

  if (!product) {
    return (
      <div className="container mx-auto px-4 py-8">
        <p>Product not found</p>
        <Link to="/products">
          <Button variant="outline" className="mt-4">
            <ArrowLeft className="mr-2 h-4 w-4" />
            Back to Products
          </Button>
        </Link>
      </div>
    );
  }

  const handleAddToCart = () => {
    alert('Product added to cart (functionality to be implemented)');
  };

  return (
    <div className="container mx-auto px-4 py-8">
      <Link to="/products">
        <Button variant="outline" className="mb-6">
          <ArrowLeft className="mr-2 h-4 w-4" />
          Back to Products
        </Button>
      </Link>

      <div className="grid md:grid-cols-2 gap-8">
        <div>
          <Card>
            <CardContent className="p-8">
              <div className="aspect-square bg-muted rounded-lg flex items-center justify-center">
                <p className="text-muted-foreground">Product Image</p>
              </div>
            </CardContent>
          </Card>
        </div>

        <div>
          <div className="mb-4">
            <div className="flex items-start justify-between mb-2">
              <h1 className="text-4xl font-bold">{product.name}</h1>
              {product.inventory_count === 0 && (
                <Badge variant="destructive">Out of Stock</Badge>
              )}
              {product.inventory_count > 0 && product.inventory_count < 10 && (
                <Badge variant="secondary">Low Stock</Badge>
              )}
            </div>
            <p className="text-xl text-muted-foreground mb-4">
              {product.description}
            </p>
            <p className="text-4xl font-bold text-primary mb-4">
              ${product.price.toFixed(2)}
            </p>
            <p className="text-sm text-muted-foreground">
              {product.inventory_count > 0 
                ? product.inventory_count + ' in stock'
                : 'Currently unavailable'
              }
            </p>
          </div>

          {product.features && (
            <Card className="mb-6">
              <CardHeader>
                <CardTitle>Features</CardTitle>
              </CardHeader>
              <CardContent>
                <ul className="list-disc list-inside space-y-2">
                  {product.features.map((feature, index) => (
                    <li key={index} className="text-muted-foreground">{feature}</li>
                  ))}
                </ul>
              </CardContent>
            </Card>
          )}

          <div className="space-y-4">
            <div className="flex items-center space-x-4">
              <label className="text-sm font-medium">Quantity:</label>
              <div className="flex items-center space-x-2">
                <Button
                  variant="outline"
                  size="icon"
                  onClick={() => setQuantity(Math.max(1, quantity - 1))}
                  disabled={product.inventory_count === 0}
                >
                  -
                </Button>
                <span className="w-12 text-center">{quantity}</span>
                <Button
                  variant="outline"
                  size="icon"
                  onClick={() => setQuantity(Math.min(product.inventory_count, quantity + 1))}
                  disabled={product.inventory_count === 0}
                >
                  +
                </Button>
              </div>
            </div>

            <div className="flex space-x-4">
              <Button 
                className="flex-1" 
                size="lg"
                disabled={product.inventory_count === 0}
                onClick={handleAddToCart}
              >
                Add to Cart
              </Button>
              <Button 
                variant="outline" 
                size="lg"
                disabled={product.inventory_count === 0}
              >
                Buy Now
              </Button>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
}

export default ProductDetail;
