'use client';

import Link from 'next/link';
import { Button } from '@/components/ui/button';
import { Card, CardContent, CardFooter, CardHeader, CardTitle } from '@/components/ui/card';
import { Badge } from '@/components/ui/badge';
import { ShoppingCart } from 'lucide-react';

// Mock product data - will be replaced with API calls
const products = [
  {
    id: 1,
    name: 'Premium Wireless Headphones',
    price: 299.99,
    category: 'Electronics',
    image: 'https://images.unsplash.com/photo-1505740420928-5e560c06d30e?w=400&h=300&fit=crop',
    inStock: true,
  },
  {
    id: 2,
    name: 'Smart Watch Series 5',
    price: 399.99,
    category: 'Electronics',
    image: 'https://images.unsplash.com/photo-1523275335684-37898b6baf30?w=400&h=300&fit=crop',
    inStock: true,
  },
  {
    id: 3,
    name: 'Leather Messenger Bag',
    price: 149.99,
    category: 'Accessories',
    image: 'https://images.unsplash.com/photo-1553062407-98eeb64c6a62?w=400&h=300&fit=crop',
    inStock: true,
  },
  {
    id: 4,
    name: 'Minimalist Desk Lamp',
    price: 79.99,
    category: 'Home',
    image: 'https://images.unsplash.com/photo-1507473885765-e6ed057f782c?w=400&h=300&fit=crop',
    inStock: false,
  },
  {
    id: 5,
    name: 'Running Shoes Pro',
    price: 129.99,
    category: 'Sports',
    image: 'https://images.unsplash.com/photo-1542291026-7eec264c27ff?w=400&h=300&fit=crop',
    inStock: true,
  },
  {
    id: 6,
    name: 'Stainless Steel Water Bottle',
    price: 34.99,
    category: 'Sports',
    image: 'https://images.unsplash.com/photo-1602143407151-7111542de6e8?w=400&h=300&fit=crop',
    inStock: true,
  },
  {
    id: 7,
    name: 'Wireless Keyboard',
    price: 89.99,
    category: 'Electronics',
    image: 'https://images.unsplash.com/photo-1587829741301-dc798b83add3?w=400&h=300&fit=crop',
    inStock: true,
  },
  {
    id: 8,
    name: 'Cotton T-Shirt Pack',
    price: 49.99,
    category: 'Clothing',
    image: 'https://images.unsplash.com/photo-1521572163474-6864f9cf17ab?w=400&h=300&fit=crop',
    inStock: true,
  },
];

export default function ProductListPage() {
  const handleAddToCart = (productId: number) => {
    // This will be connected to cart state management
    console.log('Adding product to cart:', productId);
  };

  return (
    <div className="container mx-auto px-4 py-8">
      <div className="mb-8">
        <h1 className="text-4xl font-bold mb-2">Our Products</h1>
        <p className="text-muted-foreground">
          Browse our collection of quality products
        </p>
      </div>

      <div className="grid grid-cols-1 gap-6 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4">
        {products.map((product) => (
          <Card key={product.id} className="flex flex-col overflow-hidden">
            <Link href={`/products/${product.id}`}>
              <div className="aspect-[4/3] overflow-hidden">
                <img
                  src={product.image}
                  alt={product.name}
                  className="h-full w-full object-cover transition-transform hover:scale-105"
                />
              </div>
            </Link>
            <CardHeader>
              <div className="flex items-start justify-between">
                <CardTitle className="line-clamp-2 text-lg">
                  <Link href={`/products/${product.id}`} className="hover:text-primary">
                    {product.name}
                  </Link>
                </CardTitle>
              </div>
              <Badge variant="secondary" className="w-fit">
                {product.category}
              </Badge>
            </CardHeader>
            <CardContent className="flex-1">
              <p className="text-2xl font-bold">${product.price.toFixed(2)}</p>
            </CardContent>
            <CardFooter>
              {product.inStock ? (
                <Button
                  className="w-full"
                  onClick={() => handleAddToCart(product.id)}
                >
                  <ShoppingCart className="mr-2 h-4 w-4" />
                  Add to Cart
                </Button>
              ) : (
                <Button className="w-full" variant="secondary" disabled>
                  Out of Stock
                </Button>
              )}
            </CardFooter>
          </Card>
        ))}
      </div>
    </div>
  );
}
