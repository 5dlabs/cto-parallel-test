'use client';

import { useParams, useRouter } from 'next/navigation';
import { Button } from '@/components/ui/button';
import { Badge } from '@/components/ui/badge';
import { Card, CardContent } from '@/components/ui/card';
import { Separator } from '@/components/ui/separator';
import { ArrowLeft, ShoppingCart, Heart, Share2 } from 'lucide-react';

// Mock product data - in production this would come from an API
const mockProducts = [
  {
    id: 1,
    name: 'Wireless Headphones',
    price: 79.99,
    category: 'Electronics',
    description: 'High-quality wireless headphones with advanced noise cancellation technology. Perfect for music lovers and professionals who need focus in noisy environments.',
    features: [
      'Active Noise Cancellation',
      '30-hour battery life',
      'Bluetooth 5.0',
      'Comfortable over-ear design',
      'Built-in microphone for calls',
    ],
    inStock: true,
    rating: 4.5,
    reviews: 234,
  },
  {
    id: 2,
    name: 'Smart Watch',
    price: 199.99,
    category: 'Electronics',
    description: 'Feature-rich smartwatch with comprehensive fitness tracking, heart rate monitoring, and smartphone notifications.',
    features: [
      'Heart rate monitoring',
      'GPS tracking',
      'Water resistant',
      'Sleep tracking',
      '7-day battery life',
    ],
    inStock: true,
    rating: 4.7,
    reviews: 567,
  },
  // Add more products as needed
];

export default function ProductDetailPage() {
  const params = useParams();
  const router = useRouter();
  const productId = parseInt(params.id as string);
  
  // Find product by ID
  const product = mockProducts.find(p => p.id === productId) || mockProducts[0];

  const handleAddToCart = () => {
    // In production, this would add to cart state/context
    alert(`Added ${product.name} to cart!`);
  };

  return (
    <div className="container px-4 py-8 md:px-8">
      {/* Back Button */}
      <Button
        variant="ghost"
        className="mb-6"
        onClick={() => router.back()}
      >
        <ArrowLeft className="mr-2 h-4 w-4" />
        Back to Products
      </Button>

      {/* Product Content */}
      <div className="grid grid-cols-1 gap-8 lg:grid-cols-2">
        {/* Product Image */}
        <div className="space-y-4">
          <Card className="overflow-hidden">
            <div className="relative aspect-square w-full bg-muted">
              <div className="absolute inset-0 flex items-center justify-center">
                <span className="text-lg text-muted-foreground">Product Image</span>
              </div>
            </div>
          </Card>
          
          {/* Thumbnail Gallery (placeholder) */}
          <div className="grid grid-cols-4 gap-4">
            {[1, 2, 3, 4].map((i) => (
              <Card key={i} className="overflow-hidden cursor-pointer hover:ring-2 ring-primary transition-all">
                <div className="aspect-square bg-muted flex items-center justify-center">
                  <span className="text-xs text-muted-foreground">{i}</span>
                </div>
              </Card>
            ))}
          </div>
        </div>

        {/* Product Info */}
        <div className="space-y-6">
          {/* Title and Category */}
          <div className="space-y-2">
            <Badge variant="secondary">{product.category}</Badge>
            <h1 className="text-3xl font-bold tracking-tight md:text-4xl">
              {product.name}
            </h1>
            <div className="flex items-center gap-4">
              <div className="flex items-center">
                <span className="text-yellow-500">★</span>
                <span className="ml-1 text-sm font-medium">{product.rating}</span>
                <span className="ml-1 text-sm text-muted-foreground">
                  ({product.reviews} reviews)
                </span>
              </div>
              {product.inStock ? (
                <Badge variant="default" className="bg-green-500">In Stock</Badge>
              ) : (
                <Badge variant="destructive">Out of Stock</Badge>
              )}
            </div>
          </div>

          <Separator />

          {/* Price */}
          <div>
            <p className="text-4xl font-bold">${product.price.toFixed(2)}</p>
          </div>

          {/* Description */}
          <div className="space-y-2">
            <h2 className="text-lg font-semibold">Description</h2>
            <p className="text-muted-foreground leading-relaxed">
              {product.description}
            </p>
          </div>

          {/* Features */}
          <div className="space-y-2">
            <h2 className="text-lg font-semibold">Key Features</h2>
            <ul className="space-y-2">
              {product.features.map((feature, index) => (
                <li key={index} className="flex items-start">
                  <span className="mr-2 text-primary">✓</span>
                  <span className="text-muted-foreground">{feature}</span>
                </li>
              ))}
            </ul>
          </div>

          <Separator />

          {/* Actions */}
          <div className="space-y-4">
            <div className="flex gap-2">
              <Button
                className="flex-1"
                size="lg"
                onClick={handleAddToCart}
                disabled={!product.inStock}
              >
                <ShoppingCart className="mr-2 h-5 w-5" />
                {product.inStock ? 'Add to Cart' : 'Out of Stock'}
              </Button>
              <Button variant="outline" size="lg">
                <Heart className="h-5 w-5" />
              </Button>
              <Button variant="outline" size="lg">
                <Share2 className="h-5 w-5" />
              </Button>
            </div>
            
            <Card className="bg-primary/5">
              <CardContent className="p-4">
                <div className="grid grid-cols-3 gap-4 text-center text-sm">
                  <div>
                    <p className="font-semibold">Free Shipping</p>
                    <p className="text-xs text-muted-foreground">Orders over $50</p>
                  </div>
                  <div>
                    <p className="font-semibold">30-Day Returns</p>
                    <p className="text-xs text-muted-foreground">Easy returns</p>
                  </div>
                  <div>
                    <p className="font-semibold">Warranty</p>
                    <p className="text-xs text-muted-foreground">1-year coverage</p>
                  </div>
                </div>
              </CardContent>
            </Card>
          </div>
        </div>
      </div>
    </div>
  );
}
