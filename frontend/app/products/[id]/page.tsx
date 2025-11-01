'use client';

import { useParams, useRouter } from 'next/navigation';
import { useEffect, useState } from 'react';
import { Button } from '@/components/ui/button';
import { Badge } from '@/components/ui/badge';
import { Card, CardContent } from '@/components/ui/card';
import { Separator } from '@/components/ui/separator';
import { ArrowLeft, ShoppingCart, Heart, Share2 } from 'lucide-react';

const API_BASE_URL = process.env.NEXT_PUBLIC_API_URL || 'http://localhost:8080/api';

interface Product {
  id: number;
  name: string;
  price: number;
  category?: string;
  description?: string;
  inventory_count: number;
}

async function fetchProduct(id: number): Promise<Product | null> {
  try {
    const response = await fetch(`${API_BASE_URL}/products/${id}`);
    if (!response.ok) {
      throw new Error(`API error: ${response.status}`);
    }
    return await response.json();
  } catch (error) {
    console.error('Failed to fetch product:', error);
    return null;
  }
}

async function addToCart(productId: number, quantity: number): Promise<boolean> {
  try {
    const token = typeof window !== 'undefined' ? localStorage.getItem('token') : null;
    const response = await fetch(`${API_BASE_URL}/cart/add`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
        ...(token && { 'Authorization': `Bearer ${token}` }),
      },
      body: JSON.stringify({ product_id: productId, quantity }),
    });
    return response.ok;
  } catch (error) {
    console.error('Failed to add to cart:', error);
    return false;
  }
}

export default function ProductDetailPage() {
  const params = useParams();
  const router = useRouter();
  const productId = parseInt(params.id as string);

  const [product, setProduct] = useState<Product | null>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    fetchProduct(productId)
      .then(data => {
        setProduct(data);
        setLoading(false);
      })
      .catch(err => {
        setError(err.message);
        setLoading(false);
      });
  }, [productId]);

  const handleAddToCart = async () => {
    if (!product) return;

    const success = await addToCart(product.id, 1);
    if (success) {
      alert(`Added ${product.name} to cart!`);
    } else {
      alert('Failed to add to cart. Please login first.');
    }
  };

  if (loading) {
    return (
      <div className="container px-4 py-8 md:px-8">
        <div className="flex justify-center items-center min-h-[400px]">
          <p className="text-lg text-muted-foreground">Loading product...</p>
        </div>
      </div>
    );
  }

  if (error || !product) {
    return (
      <div className="container px-4 py-8 md:px-8">
        <div className="flex flex-col items-center justify-center min-h-[400px] text-center">
          <p className="text-xl font-semibold mb-2 text-destructive">Product not found</p>
          <p className="text-muted-foreground mb-4">{error || 'This product does not exist'}</p>
          <Button onClick={() => router.push('/products')}>Back to Products</Button>
        </div>
      </div>
    );
  }

  const inStock = product.inventory_count > 0;

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
            {product.category && <Badge variant="secondary">{product.category}</Badge>}
            <h1 className="text-3xl font-bold tracking-tight md:text-4xl">
              {product.name}
            </h1>
            <div className="flex items-center gap-4">
              {inStock ? (
                <Badge variant="default" className="bg-green-500">In Stock ({product.inventory_count} available)</Badge>
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
          {product.description && (
            <div className="space-y-2">
              <h2 className="text-lg font-semibold">Description</h2>
              <p className="text-muted-foreground leading-relaxed">
                {product.description}
              </p>
            </div>
          )}

          <Separator />

          {/* Actions */}
          <div className="space-y-4">
            <div className="flex gap-2">
              <Button
                className="flex-1"
                size="lg"
                onClick={handleAddToCart}
                disabled={!inStock}
              >
                <ShoppingCart className="mr-2 h-5 w-5" />
                {inStock ? 'Add to Cart' : 'Out of Stock'}
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
