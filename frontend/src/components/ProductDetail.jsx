import React from 'react';
import { useParams, Link } from 'react-router-dom';
import { Card, CardContent, CardDescription, CardFooter, CardHeader, CardTitle } from './ui/card';
import { Button } from './ui/button';
import { Badge } from './ui/badge';
import { ArrowLeft, ShoppingCart } from 'lucide-react';

const ProductDetail = () => {
  const { id } = useParams();
  
  // Mock product data - will be replaced with API calls later
  const products = {
    1: { id: 1, name: 'Premium Headphones', description: 'High-quality wireless headphones with noise cancellation', price: 199.99, inventory_count: 15, features: ['Wireless', 'Noise Cancellation', '30-hour battery', 'Premium sound quality'] },
    2: { id: 2, name: 'Smart Watch', description: 'Feature-rich smartwatch with fitness tracking', price: 299.99, inventory_count: 8, features: ['Heart rate monitor', 'GPS tracking', 'Water resistant', 'Smart notifications'] },
    3: { id: 3, name: 'Laptop Stand', description: 'Ergonomic aluminum stand for laptops', price: 49.99, inventory_count: 25, features: ['Aluminum construction', 'Adjustable height', 'Cooling design', 'Cable management'] },
    4: { id: 4, name: 'Wireless Mouse', description: 'Precision wireless mouse with ergonomic design', price: 39.99, inventory_count: 30, features: ['Wireless connectivity', 'Ergonomic design', 'Precision sensor', 'Long battery life'] },
    5: { id: 5, name: 'USB-C Hub', description: 'Multi-port USB-C adapter', price: 59.99, inventory_count: 12, features: ['Multiple ports', 'Fast data transfer', 'Compact design', 'Plug and play'] },
    6: { id: 6, name: 'Mechanical Keyboard', description: 'RGB backlit mechanical keyboard', price: 149.99, inventory_count: 0, features: ['Mechanical switches', 'RGB backlighting', 'Programmable keys', 'Durable construction'] },
  };

  const product = products[id];

  if (!product) {
    return (
      <div className="container mx-auto px-4 py-8">
        <Card>
          <CardHeader>
            <CardTitle>Product Not Found</CardTitle>
            <CardDescription>The product you're looking for doesn't exist.</CardDescription>
          </CardHeader>
          <CardFooter>
            <Link to="/products">
              <Button>
                <ArrowLeft className="mr-2 h-4 w-4" />
                Back to Products
              </Button>
            </Link>
          </CardFooter>
        </Card>
      </div>
    );
  }

  return (
    <div className="container mx-auto px-4 py-8">
      <Link to="/products" className="inline-flex items-center text-primary hover:underline mb-6">
        <ArrowLeft className="mr-2 h-4 w-4" />
        Back to Products
      </Link>

      <div className="grid grid-cols-1 lg:grid-cols-2 gap-8">
        <div className="bg-muted rounded-lg aspect-square flex items-center justify-center">
          <p className="text-muted-foreground text-lg">Product Image</p>
        </div>

        <div>
          <div className="flex items-start justify-between mb-4">
            <h1 className="text-4xl font-bold">{product.name}</h1>
            {product.inventory_count === 0 && (
              <Badge variant="destructive">Out of Stock</Badge>
            )}
            {product.inventory_count > 0 && product.inventory_count < 10 && (
              <Badge variant="secondary">Low Stock</Badge>
            )}
          </div>

          <p className="text-5xl font-bold text-primary mb-4">${product.price}</p>
          
          <p className="text-muted-foreground mb-6">{product.description}</p>

          <div className="mb-6">
            <h3 className="font-semibold mb-2">Features:</h3>
            <ul className="list-disc list-inside space-y-1">
              {product.features.map((feature, index) => (
                <li key={index} className="text-muted-foreground">{feature}</li>
              ))}
            </ul>
          </div>

          <div className="mb-6">
            <p className="text-sm text-muted-foreground">
              {product.inventory_count > 0 
                ? `${product.inventory_count} items in stock` 
                : 'Currently out of stock'}
            </p>
          </div>

          <div className="flex gap-4">
            <Button 
              size="lg" 
              className="flex-1"
              disabled={product.inventory_count === 0}
            >
              <ShoppingCart className="mr-2 h-5 w-5" />
              Add to Cart
            </Button>
          </div>
        </div>
      </div>
    </div>
  );
};

export default ProductDetail;
