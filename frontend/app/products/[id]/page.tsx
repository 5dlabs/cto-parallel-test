'use client';

import { use } from 'react';
import Link from 'next/link';
import { Button } from '@/components/ui/button';
import { Badge } from '@/components/ui/badge';

import { ShoppingCart, ArrowLeft, Star, Truck, Shield, RefreshCcw } from 'lucide-react';

// Mock product data - will be replaced with API calls
const getProduct = (id: string) => {
  const products = [
    {
      id: 1,
      name: 'Premium Wireless Headphones',
      price: 299.99,
      category: 'Electronics',
      description: 'Experience crystal-clear audio with our premium wireless headphones. Featuring active noise cancellation, 30-hour battery life, and premium comfort for all-day wear.',
      features: [
        'Active Noise Cancellation',
        '30-hour battery life',
        'Premium comfort padding',
        'Bluetooth 5.0 connectivity',
        'Foldable design',
      ],
      image: 'https://images.unsplash.com/photo-1505740420928-5e560c06d30e?w=600&h=600&fit=crop',
      inStock: true,
      rating: 4.5,
      reviews: 128,
    },
    {
      id: 2,
      name: 'Smart Watch Series 5',
      price: 399.99,
      category: 'Electronics',
      description: 'Stay connected and track your fitness with the latest smart watch. Features heart rate monitoring, GPS, and water resistance up to 50m.',
      features: [
        'Heart rate monitoring',
        'Built-in GPS',
        'Water resistant (50m)',
        'Sleep tracking',
        '7-day battery life',
      ],
      image: 'https://images.unsplash.com/photo-1523275335684-37898b6baf30?w=600&h=600&fit=crop',
      inStock: true,
      rating: 4.8,
      reviews: 256,
    },
  ];

  return products.find((p) => p.id === parseInt(id)) || products[0];
};

interface ProductDetailPageProps {
  params: Promise<{ id: string }>;
}

export default function ProductDetailPage({ params }: ProductDetailPageProps) {
  const { id } = use(params);
  const product = getProduct(id);

  const handleAddToCart = () => {
    // This will be connected to cart state management
    console.log('Adding product to cart:', product.id);
  };

  return (
    <div className="container mx-auto px-4 py-8">
      <Link href="/products" className="inline-flex items-center text-sm text-muted-foreground hover:text-foreground mb-6">
        <ArrowLeft className="mr-2 h-4 w-4" />
        Back to Products
      </Link>

      <div className="grid gap-8 md:grid-cols-2">
        {/* Product Image */}
        <div className="rounded-lg overflow-hidden bg-muted">
          <img
            src={product.image}
            alt={product.name}
            className="w-full h-auto object-cover"
          />
        </div>

        {/* Product Details */}
        <div className="flex flex-col space-y-6">
          <div>
            <Badge variant="secondary" className="mb-2">
              {product.category}
            </Badge>
            <h1 className="text-3xl font-bold mb-2">{product.name}</h1>
            <div className="flex items-center gap-2 mb-4">
              <div className="flex">
                {[...Array(5)].map((_, i) => (
                  <Star
                    key={i}
                    className={`h-4 w-4 ${
                      i < Math.floor(product.rating)
                        ? 'fill-yellow-400 text-yellow-400'
                        : 'text-gray-300'
                    }`}
                  />
                ))}
              </div>
              <span className="text-sm text-muted-foreground">
                {product.rating} ({product.reviews} reviews)
              </span>
            </div>
          </div>

          <div>
            <p className="text-3xl font-bold">${product.price.toFixed(2)}</p>
          </div>

          <div>
            <h2 className="text-lg font-semibold mb-2">Description</h2>
            <p className="text-muted-foreground">{product.description}</p>
          </div>

          <div>
            <h2 className="text-lg font-semibold mb-2">Key Features</h2>
            <ul className="space-y-2">
              {product.features.map((feature, index) => (
                <li key={index} className="flex items-start">
                  <span className="mr-2 text-primary">•</span>
                  <span className="text-muted-foreground">{feature}</span>
                </li>
              ))}
            </ul>
          </div>

          {product.inStock ? (
            <Button size="lg" onClick={handleAddToCart}>
              <ShoppingCart className="mr-2 h-5 w-5" />
              Add to Cart
            </Button>
          ) : (
            <Button size="lg" variant="secondary" disabled>
              Out of Stock
            </Button>
          )}

          {/* Benefits */}
          <div className="grid grid-cols-1 gap-4 pt-6 border-t">
            <div className="flex items-start gap-3">
              <Truck className="h-5 w-5 text-primary mt-0.5" />
              <div>
                <p className="font-medium">Free Shipping</p>
                <p className="text-sm text-muted-foreground">On orders over $50</p>
              </div>
            </div>
            <div className="flex items-start gap-3">
              <Shield className="h-5 w-5 text-primary mt-0.5" />
              <div>
                <p className="font-medium">2-Year Warranty</p>
                <p className="text-sm text-muted-foreground">Full coverage included</p>
              </div>
            </div>
            <div className="flex items-start gap-3">
              <RefreshCcw className="h-5 w-5 text-primary mt-0.5" />
              <div>
                <p className="font-medium">30-Day Returns</p>
                <p className="text-sm text-muted-foreground">Hassle-free returns</p>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
}
