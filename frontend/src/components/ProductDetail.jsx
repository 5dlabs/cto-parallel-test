import React, { useState, useEffect } from 'react';
import { useParams, Link } from 'react-router-dom';
import { Card, CardHeader, CardTitle, CardContent } from './ui/card';
import { Button } from './ui/button';
import { Badge } from './ui/badge';
import { ArrowLeft, ShoppingCart } from 'lucide-react';

function ProductDetail() {
  const { id } = useParams();
  const [product, setProduct] = useState(null);
  const [loading, setLoading] = useState(true);
  const [quantity, setQuantity] = useState(1);

  useEffect(() => {
    // In a real app, this would fetch from the API
    const mockProducts = {
      '1': { id: 1, name: 'Laptop Pro', description: 'High-performance laptop for professionals', price: 1299.99, inventory_count: 15, details: 'Intel Core i7, 16GB RAM, 512GB SSD, 14" Display' },
      '2': { id: 2, name: 'Wireless Mouse', description: 'Ergonomic wireless mouse with precision tracking', price: 29.99, inventory_count: 50, details: '2400 DPI, Bluetooth 5.0, Rechargeable battery' },
      '3': { id: 3, name: 'Mechanical Keyboard', description: 'RGB mechanical keyboard with tactile switches', price: 89.99, inventory_count: 30, details: 'Cherry MX switches, Aluminum frame, Customizable RGB' },
      '4': { id: 4, name: 'USB-C Hub', description: '7-in-1 USB-C hub with HDMI and card reader', price: 49.99, inventory_count: 25, details: '4K HDMI, 3x USB 3.0, SD/microSD card slots' },
      '5': { id: 5, name: 'Webcam HD', description: '1080p HD webcam with built-in microphone', price: 79.99, inventory_count: 20, details: '1080p 30fps, Auto-focus, Noise-canceling mic' },
      '6': { id: 6, name: 'Monitor 27"', description: '4K UHD monitor with IPS panel', price: 399.99, inventory_count: 10, details: '3840x2160, IPS panel, 99% sRGB, HDR10' },
    };

    setTimeout(() => {
      setProduct(mockProducts[id] || null);
      setLoading(false);
    }, 300);
  }, [id]);

  const handleQuantityChange = (delta) => {
    const newQuantity = quantity + delta;
    if (newQuantity >= 1 && newQuantity <= (product?.inventory_count || 1)) {
      setQuantity(newQuantity);
    }
  };

  if (loading) {
    return (
      <div className="text-center py-12">
        <p className="text-muted-foreground">Loading product...</p>
      </div>
    );
  }

  if (!product) {
    return (
      <div className="text-center py-12 space-y-4">
        <h2 className="text-2xl font-bold">Product Not Found</h2>
        <p className="text-muted-foreground">The product you're looking for doesn't exist.</p>
        <Link to="/products">
          <Button>
            <ArrowLeft className="mr-2 h-4 w-4" />
            Back to Products
          </Button>
        </Link>
      </div>
    );
  }

  return (
    <div className="space-y-6">
      <Link to="/products">
        <Button variant="ghost">
          <ArrowLeft className="mr-2 h-4 w-4" />
          Back to Products
        </Button>
      </Link>

      <div className="grid md:grid-cols-2 gap-8">
        <Card className="h-fit">
          <CardContent className="p-0">
            <div className="aspect-square bg-muted flex items-center justify-center rounded-t-lg">
              <p className="text-muted-foreground">Product Image</p>
            </div>
          </CardContent>
        </Card>

        <div className="space-y-6">
          <div>
            <div className="flex items-start justify-between mb-2">
              <h1 className="text-3xl font-bold">{product.name}</h1>
              {product.inventory_count < 15 && (
                <Badge variant="secondary">Low Stock</Badge>
              )}
            </div>
            <p className="text-muted-foreground">{product.description}</p>
          </div>

          <div className="space-y-2">
            <p className="text-3xl font-bold text-primary">${product.price.toFixed(2)}</p>
            <p className="text-sm text-muted-foreground">
              {product.inventory_count} available
            </p>
          </div>

          <Card>
            <CardHeader>
              <CardTitle className="text-lg">Product Details</CardTitle>
            </CardHeader>
            <CardContent>
              <p className="text-sm">{product.details}</p>
            </CardContent>
          </Card>

          <div className="space-y-4">
            <div className="flex items-center gap-4">
              <span className="text-sm font-medium">Quantity:</span>
              <div className="flex items-center gap-2">
                <Button
                  variant="outline"
                  size="icon"
                  onClick={() => handleQuantityChange(-1)}
                  disabled={quantity <= 1}
                >
                  -
                </Button>
                <span className="w-12 text-center font-medium">{quantity}</span>
                <Button
                  variant="outline"
                  size="icon"
                  onClick={() => handleQuantityChange(1)}
                  disabled={quantity >= product.inventory_count}
                >
                  +
                </Button>
              </div>
            </div>

            <Button size="lg" className="w-full">
              <ShoppingCart className="mr-2 h-5 w-5" />
              Add to Cart
            </Button>
          </div>
        </div>
      </div>
    </div>
  );
}

export default ProductDetail;
