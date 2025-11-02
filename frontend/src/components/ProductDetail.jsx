import React from 'react';
import { useParams, Link } from 'react-router-dom';
import { Card, CardContent, CardHeader, CardTitle } from './ui/card';
import { Button } from './ui/button';
import { Badge } from './ui/badge';

export function ProductDetail() {
  const { id } = useParams();
  
  const product = {
    id: parseInt(id),
    name: 'Wireless Headphones',
    description: 'High-quality wireless headphones with noise cancellation',
    longDescription: 'These premium wireless headphones feature advanced noise cancellation technology, ensuring you can enjoy your music without distractions. With 30 hours of battery life and comfortable ear cushions, they are perfect for long listening sessions.',
    price: 99.99,
    inventory: 15,
    features: ['Noise Cancellation', 'Wireless', '30h Battery', 'Comfortable Fit'],
  };

  return (
    <div className="container mx-auto px-4 py-8">
      <Link to="/products">
        <Button variant="ghost" className="mb-6">← Back to Products</Button>
      </Link>
      
      <div className="grid grid-cols-1 md:grid-cols-2 gap-8">
        <div>
          <div className="bg-muted rounded-lg aspect-square flex items-center justify-center">
            <p className="text-muted-foreground">Product Image</p>
          </div>
        </div>
        
        <div>
          <div className="flex items-start justify-between mb-4">
            <h1 className="text-3xl font-bold">{product.name}</h1>
            {product.inventory > 0 && product.inventory < 10 && (
              <Badge variant="secondary">Low Stock</Badge>
            )}
            {product.inventory === 0 && (
              <Badge variant="destructive">Out of Stock</Badge>
            )}
          </div>
          
          <p className="text-3xl font-bold mb-4">${product.price}</p>
          
          <p className="text-muted-foreground mb-6">{product.longDescription}</p>
          
          <Card className="mb-6">
            <CardHeader>
              <CardTitle>Features</CardTitle>
            </CardHeader>
            <CardContent>
              <ul className="list-disc list-inside space-y-2">
                {product.features.map((feature, index) => (
                  <li key={index}>{feature}</li>
                ))}
              </ul>
            </CardContent>
          </Card>
          
          <div className="flex gap-4">
            <Button 
              size="lg" 
              className="flex-1"
              disabled={product.inventory === 0}
            >
              Add to Cart
            </Button>
            <Button 
              size="lg" 
              variant="outline"
              disabled={product.inventory === 0}
            >
              Buy Now
            </Button>
          </div>
          
          <p className="text-sm text-muted-foreground mt-4">
            {product.inventory > 0 ? `${product.inventory} items in stock` : 'Out of stock'}
          </p>
        </div>
      </div>
    </div>
  );
}
