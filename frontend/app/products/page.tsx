'use client';

import Link from 'next/link';
import { useEffect, useState } from 'react';
import { Button } from '@/components/ui/button';
import { Card, CardContent, CardDescription, CardFooter, CardHeader, CardTitle } from '@/components/ui/card';
import { Badge } from '@/components/ui/badge';

const API_BASE_URL = process.env.NEXT_PUBLIC_API_URL || 'http://localhost:8080/api';

interface Product {
  id: number;
  name: string;
  price: number;
  category?: string;
  image?: string;
  description?: string;
  inventory_count: number;
}

async function fetchProducts(): Promise<Product[]> {
  try {
    const response = await fetch(`${API_BASE_URL}/products`);
    if (!response.ok) {
      throw new Error(`API error: ${response.status}`);
    }
    return await response.json();
  } catch (error) {
    console.error('Failed to fetch products:', error);
    // Return empty array on error - no mock fallback
    return [];
  }
}

export default function ProductsPage() {
  const [products, setProducts] = useState<Product[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    fetchProducts()
      .then(data => {
        setProducts(data);
        setLoading(false);
      })
      .catch(err => {
        setError(err.message);
        setLoading(false);
      });
  }, []);

  if (loading) {
    return (
      <div className="container px-4 py-8 md:px-8">
        <div className="flex justify-center items-center min-h-[400px]">
          <p className="text-lg text-muted-foreground">Loading products...</p>
        </div>
      </div>
    );
  }

  if (error) {
    return (
      <div className="container px-4 py-8 md:px-8">
        <div className="flex flex-col items-center justify-center min-h-[400px] text-center">
          <p className="text-xl font-semibold mb-2 text-destructive">Failed to load products</p>
          <p className="text-muted-foreground mb-4">{error}</p>
          <Button onClick={() => window.location.reload()}>Retry</Button>
        </div>
      </div>
    );
  }
  return (
    <div className="container px-4 py-8 md:px-8">
      {/* Page Header */}
      <div className="mb-8 space-y-2">
        <h1 className="text-3xl font-bold tracking-tight md:text-4xl">
          All Products
        </h1>
        <p className="text-muted-foreground">
          Browse our collection of quality products
        </p>
      </div>

      {/* Products Grid */}
      <div className="grid grid-cols-1 gap-6 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4">
        {products.map((product) => {
          const inStock = product.inventory_count > 0;
          return (
            <Card key={product.id} className="flex flex-col overflow-hidden hover:shadow-lg transition-shadow">
              <CardHeader className="p-0">
                <div className="relative aspect-square w-full bg-muted">
                  <div className="absolute inset-0 flex items-center justify-center">
                    <span className="text-sm text-muted-foreground">Product Image</span>
                  </div>
                  {!inStock && (
                    <Badge className="absolute top-2 right-2" variant="destructive">
                      Out of Stock
                    </Badge>
                  )}
                </div>
              </CardHeader>
              <CardContent className="flex-1 p-4 space-y-2">
                <div className="flex items-start justify-between gap-2">
                  <CardTitle className="text-lg line-clamp-2">
                    {product.name}
                  </CardTitle>
                </div>
                {product.category && (
                  <Badge variant="secondary" className="text-xs">
                    {product.category}
                  </Badge>
                )}
                {product.description && (
                  <CardDescription className="line-clamp-2">
                    {product.description}
                  </CardDescription>
                )}
                <div className="pt-2">
                  <p className="text-2xl font-bold">
                    ${product.price.toFixed(2)}
                  </p>
                </div>
              </CardContent>
              <CardFooter className="p-4 pt-0 flex gap-2">
                <Link href={`/products/${product.id}`} className="flex-1">
                  <Button variant="outline" className="w-full">
                    View Details
                  </Button>
                </Link>
                <Button
                  className="flex-1"
                  disabled={!inStock}
                >
                  {inStock ? 'Add to Cart' : 'Unavailable'}
                </Button>
              </CardFooter>
            </Card>
          );
        })}
      </div>

      {/* Empty State (if no products) */}
      {products.length === 0 && (
        <div className="flex flex-col items-center justify-center py-16 text-center">
          <p className="text-xl font-semibold mb-2">No products found</p>
          <p className="text-muted-foreground">Check back later for new items</p>
        </div>
      )}
    </div>
  );
}
