'use client';

import { useParams, useRouter } from 'next/navigation';
import { Button } from '@/components/ui/button';
import { Badge } from '@/components/ui/badge';
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card';
import { ArrowLeft, ShoppingCart, Star } from 'lucide-react';

// Mock product data - in a real app, this would come from an API
const getProduct = (id: string) => {
  const products = [
    {
      id: 1,
      name: 'Wireless Headphones',
      description: 'Premium noise-cancelling headphones with 30-hour battery life',
      fullDescription: 'Experience superior sound quality with our premium wireless headphones. Featuring advanced noise-cancelling technology, these headphones deliver an immersive audio experience. With up to 30 hours of battery life, comfortable ear cushions, and a foldable design, they are perfect for travel and daily use.',
      price: 299.99,
      category: 'Electronics',
      inStock: true,
      rating: 4.5,
      reviews: 128,
      features: [
        'Active Noise Cancellation',
        '30-hour battery life',
        'Bluetooth 5.0',
        'Comfortable over-ear design',
        'Built-in microphone',
      ],
    },
    {
      id: 2,
      name: 'Smart Watch',
      description: 'Fitness tracker with heart rate monitor and GPS',
      fullDescription: 'Stay connected and track your fitness goals with our advanced smart watch. Monitor your heart rate, track your workouts, and navigate with built-in GPS. Water-resistant design perfect for swimming and all-day wear.',
      price: 249.99,
      category: 'Electronics',
      inStock: true,
      rating: 4.3,
      reviews: 89,
      features: [
        'Heart rate monitoring',
        'Built-in GPS',
        'Water-resistant up to 50m',
        '7-day battery life',
        'Sleep tracking',
      ],
    },
    {
      id: 3,
      name: 'Laptop Backpack',
      description: 'Water-resistant backpack with multiple compartments',
      fullDescription: 'Protect your laptop and essentials with our durable, water-resistant backpack. Features multiple compartments for organization, padded laptop sleeve, and comfortable shoulder straps for all-day comfort.',
      price: 79.99,
      category: 'Accessories',
      inStock: true,
      rating: 4.7,
      reviews: 203,
      features: [
        'Fits laptops up to 17 inches',
        'Water-resistant material',
        'USB charging port',
        'Multiple compartments',
        'Padded back panel',
      ],
    },
    {
      id: 4,
      name: 'Coffee Maker',
      description: 'Programmable coffee maker with thermal carafe',
      fullDescription: 'Brew the perfect cup of coffee every morning with our programmable coffee maker. Features a thermal carafe that keeps coffee hot for hours, auto-brew function, and a permanent filter.',
      price: 89.99,
      category: 'Home & Kitchen',
      inStock: false,
      rating: 4.4,
      reviews: 156,
      features: [
        'Programmable brew time',
        'Thermal carafe',
        '12-cup capacity',
        'Auto shut-off',
        'Permanent filter included',
      ],
    },
    {
      id: 5,
      name: 'Yoga Mat',
      description: 'Extra thick exercise mat with carrying strap',
      fullDescription: 'Enhance your yoga practice with our extra thick, non-slip yoga mat. Perfect for all types of exercises, this mat provides excellent cushioning and support. Includes a convenient carrying strap.',
      price: 34.99,
      category: 'Sports',
      inStock: true,
      rating: 4.6,
      reviews: 312,
      features: [
        'Extra thick (6mm)',
        'Non-slip surface',
        'Eco-friendly material',
        'Carrying strap included',
        'Easy to clean',
      ],
    },
    {
      id: 6,
      name: 'Desk Lamp',
      description: 'LED desk lamp with adjustable brightness and color temperature',
      fullDescription: 'Illuminate your workspace with our versatile LED desk lamp. Features adjustable brightness levels and color temperature to reduce eye strain. Touch controls and flexible arm for easy positioning.',
      price: 45.99,
      category: 'Home & Office',
      inStock: true,
      rating: 4.5,
      reviews: 92,
      features: [
        'Adjustable brightness',
        'Multiple color temperatures',
        'Touch controls',
        'Flexible arm',
        'Energy-efficient LED',
      ],
    },
    {
      id: 7,
      name: 'Water Bottle',
      description: 'Insulated stainless steel water bottle, keeps drinks cold for 24 hours',
      fullDescription: 'Stay hydrated with our premium insulated water bottle. Double-wall vacuum insulation keeps drinks cold for 24 hours or hot for 12 hours. BPA-free, leak-proof design perfect for any adventure.',
      price: 24.99,
      category: 'Sports',
      inStock: true,
      rating: 4.8,
      reviews: 421,
      features: [
        'Double-wall insulation',
        'Keeps cold for 24 hours',
        'BPA-free',
        'Leak-proof lid',
        '32 oz capacity',
      ],
    },
    {
      id: 8,
      name: 'Bluetooth Speaker',
      description: 'Portable waterproof speaker with 360-degree sound',
      fullDescription: 'Take your music anywhere with our powerful Bluetooth speaker. 360-degree sound, waterproof design, and up to 12 hours of playtime. Perfect for outdoor adventures and gatherings.',
      price: 129.99,
      category: 'Electronics',
      inStock: true,
      rating: 4.6,
      reviews: 187,
      features: [
        '360-degree sound',
        'Waterproof (IPX7)',
        '12-hour battery',
        'Bluetooth 5.0',
        'Built-in microphone',
      ],
    },
  ];

  return products.find(p => p.id === parseInt(id));
};

export default function ProductDetailPage() {
  const params = useParams();
  const router = useRouter();
  const productId = params.id as string;
  const product = getProduct(productId);

  if (!product) {
    return (
      <div className="container mx-auto px-4 py-12 text-center">
        <h1 className="mb-4 text-2xl font-bold">Product Not Found</h1>
        <p className="mb-6 text-muted-foreground">
          The product you&apos;re looking for doesn&apos;t exist.
        </p>
        <Button onClick={() => router.push('/products')}>
          Back to Products
        </Button>
      </div>
    );
  }

  return (
    <div className="container mx-auto px-4 py-8 md:py-12">
      <Button
        variant="ghost"
        onClick={() => router.back()}
        className="mb-6"
        aria-label="Go back"
      >
        <ArrowLeft className="mr-2 h-4 w-4" />
        Back
      </Button>

      <div className="grid gap-8 md:grid-cols-2">
        {/* Product Image Placeholder */}
        <div className="aspect-square w-full overflow-hidden rounded-lg bg-muted">
          <div className="flex h-full items-center justify-center text-muted-foreground">
            <div className="text-center">
              <ShoppingCart className="mx-auto mb-2 h-16 w-16" />
              <p>Product Image</p>
            </div>
          </div>
        </div>

        {/* Product Details */}
        <div className="flex flex-col">
          <div className="mb-4">
            <Badge variant="secondary" className="mb-2">
              {product.category}
            </Badge>
            <h1 className="mb-2 text-3xl font-bold md:text-4xl">
              {product.name}
            </h1>
            <div className="mb-2 flex items-center gap-2">
              <div className="flex items-center">
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
            <p className="text-muted-foreground">{product.description}</p>
          </div>

          <div className="mb-6">
            <p className="mb-1 text-3xl font-bold">
              ${product.price.toFixed(2)}
            </p>
            {product.inStock ? (
              <Badge variant="secondary">In Stock</Badge>
            ) : (
              <Badge variant="destructive">Out of Stock</Badge>
            )}
          </div>

          <div className="mb-6 flex gap-4">
            <Button
              size="lg"
              className="flex-1"
              disabled={!product.inStock}
              aria-label="Add to cart"
            >
              <ShoppingCart className="mr-2 h-5 w-5" />
              Add to Cart
            </Button>
          </div>

          <Card>
            <CardHeader>
              <CardTitle>Product Details</CardTitle>
              <CardDescription>{product.fullDescription}</CardDescription>
            </CardHeader>
            <CardContent>
              <h3 className="mb-2 font-semibold">Key Features:</h3>
              <ul className="space-y-2">
                {product.features.map((feature, index) => (
                  <li key={index} className="flex items-start">
                    <span className="mr-2 text-primary">•</span>
                    <span className="text-sm">{feature}</span>
                  </li>
                ))}
              </ul>
            </CardContent>
          </Card>
        </div>
      </div>
    </div>
  );
}
