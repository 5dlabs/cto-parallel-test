'use client';

import Link from 'next/link';
import { Card, CardContent, CardDescription, CardFooter, CardHeader, CardTitle } from '@/components/ui/card';
import { Button } from '@/components/ui/button';
import { Badge } from '@/components/ui/badge';

// Mock product data - in a real app, this would come from an API
const products = [
  {
    id: 1,
    name: 'Wireless Headphones',
    description: 'Premium noise-cancelling headphones with 30-hour battery life',
    price: 299.99,
    category: 'Electronics',
    inStock: true,
  },
  {
    id: 2,
    name: 'Smart Watch',
    description: 'Fitness tracker with heart rate monitor and GPS',
    price: 249.99,
    category: 'Electronics',
    inStock: true,
  },
  {
    id: 3,
    name: 'Laptop Backpack',
    description: 'Water-resistant backpack with multiple compartments',
    price: 79.99,
    category: 'Accessories',
    inStock: true,
  },
  {
    id: 4,
    name: 'Coffee Maker',
    description: 'Programmable coffee maker with thermal carafe',
    price: 89.99,
    category: 'Home & Kitchen',
    inStock: false,
  },
  {
    id: 5,
    name: 'Yoga Mat',
    description: 'Extra thick exercise mat with carrying strap',
    price: 34.99,
    category: 'Sports',
    inStock: true,
  },
  {
    id: 6,
    name: 'Desk Lamp',
    description: 'LED desk lamp with adjustable brightness and color temperature',
    price: 45.99,
    category: 'Home & Office',
    inStock: true,
  },
  {
    id: 7,
    name: 'Water Bottle',
    description: 'Insulated stainless steel water bottle, keeps drinks cold for 24 hours',
    price: 24.99,
    category: 'Sports',
    inStock: true,
  },
  {
    id: 8,
    name: 'Bluetooth Speaker',
    description: 'Portable waterproof speaker with 360-degree sound',
    price: 129.99,
    category: 'Electronics',
    inStock: true,
  },
];

export default function ProductListPage() {
  return (
    <div className="container mx-auto px-4 py-8 md:py-12">
      <div className="mb-8">
        <h1 className="mb-2 text-3xl font-bold md:text-4xl">All Products</h1>
        <p className="text-muted-foreground">
          Browse our collection of quality products
        </p>
      </div>

      <div className="grid gap-6 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4">
        {products.map((product) => (
          <Card key={product.id} className="flex flex-col">
            <CardHeader>
              <div className="mb-2 flex items-center justify-between">
                <Badge variant="secondary">{product.category}</Badge>
                {!product.inStock && (
                  <Badge variant="destructive">Out of Stock</Badge>
                )}
              </div>
              <CardTitle className="line-clamp-1">{product.name}</CardTitle>
              <CardDescription className="line-clamp-2">
                {product.description}
              </CardDescription>
            </CardHeader>
            <CardContent className="flex-1">
              <p className="text-2xl font-bold">${product.price.toFixed(2)}</p>
            </CardContent>
            <CardFooter className="flex gap-2">
              <Link href={`/products/${product.id}`} className="flex-1">
                <Button variant="outline" className="w-full">
                  View Details
                </Button>
              </Link>
              <Button
                className="flex-1"
                disabled={!product.inStock}
                aria-label={`Add ${product.name} to cart`}
              >
                Add to Cart
              </Button>
            </CardFooter>
          </Card>
        ))}
      </div>

      {products.length === 0 && (
        <div className="py-12 text-center">
          <p className="text-muted-foreground">No products found.</p>
        </div>
      )}
    </div>
  );
}
