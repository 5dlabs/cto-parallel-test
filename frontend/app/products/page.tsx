import Link from 'next/link';
import { Button } from '@/components/ui/button';
import { Card, CardContent, CardDescription, CardFooter, CardHeader, CardTitle } from '@/components/ui/card';
import { Badge } from '@/components/ui/badge';

// Mock product data - in production this would come from an API
const products = [
  {
    id: 1,
    name: 'Wireless Headphones',
    price: 79.99,
    category: 'Electronics',
    image: '/placeholder-product.jpg',
    description: 'High-quality wireless headphones with noise cancellation',
    inStock: true,
  },
  {
    id: 2,
    name: 'Smart Watch',
    price: 199.99,
    category: 'Electronics',
    image: '/placeholder-product.jpg',
    description: 'Feature-rich smartwatch with fitness tracking',
    inStock: true,
  },
  {
    id: 3,
    name: 'Laptop Backpack',
    price: 49.99,
    category: 'Accessories',
    image: '/placeholder-product.jpg',
    description: 'Durable backpack with padded laptop compartment',
    inStock: true,
  },
  {
    id: 4,
    name: 'Mechanical Keyboard',
    price: 129.99,
    category: 'Electronics',
    image: '/placeholder-product.jpg',
    description: 'Premium mechanical keyboard with RGB lighting',
    inStock: false,
  },
  {
    id: 5,
    name: 'Portable Charger',
    price: 34.99,
    category: 'Electronics',
    image: '/placeholder-product.jpg',
    description: '20000mAh power bank with fast charging',
    inStock: true,
  },
  {
    id: 6,
    name: 'USB-C Hub',
    price: 59.99,
    category: 'Accessories',
    image: '/placeholder-product.jpg',
    description: 'Multi-port USB-C hub with HDMI and Ethernet',
    inStock: true,
  },
  {
    id: 7,
    name: 'Wireless Mouse',
    price: 39.99,
    category: 'Electronics',
    image: '/placeholder-product.jpg',
    description: 'Ergonomic wireless mouse with precision tracking',
    inStock: true,
  },
  {
    id: 8,
    name: 'Phone Stand',
    price: 24.99,
    category: 'Accessories',
    image: '/placeholder-product.jpg',
    description: 'Adjustable phone stand for desk or nightstand',
    inStock: true,
  },
];

export default function ProductsPage() {
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
        {products.map((product) => (
          <Card key={product.id} className="flex flex-col overflow-hidden hover:shadow-lg transition-shadow">
            <CardHeader className="p-0">
              <div className="relative aspect-square w-full bg-muted">
                <div className="absolute inset-0 flex items-center justify-center">
                  <span className="text-sm text-muted-foreground">Product Image</span>
                </div>
                {!product.inStock && (
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
              <Badge variant="secondary" className="text-xs">
                {product.category}
              </Badge>
              <CardDescription className="line-clamp-2">
                {product.description}
              </CardDescription>
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
                disabled={!product.inStock}
              >
                {product.inStock ? 'Add to Cart' : 'Unavailable'}
              </Button>
            </CardFooter>
          </Card>
        ))}
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
