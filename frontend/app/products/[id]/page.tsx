"use client";

import { useParams } from "next/navigation";
import Link from "next/link";
import Image from "next/image";
import { Button } from "@/components/ui/button";
import { Badge } from "@/components/ui/badge";
import { Card, CardContent } from "@/components/ui/card";
import { ArrowLeft, ShoppingCart, Star } from "lucide-react";

// Mock product data - will be replaced with real API data later
const products: Record<string, {
  id: number;
  name: string;
  price: number;
  category: string;
  inStock: boolean;
  image: string;
  description: string;
  rating: number;
  reviews: number;
}> = {
  "1": {
    id: 1,
    name: "Wireless Headphones",
    price: 99.99,
    category: "Electronics",
    inStock: true,
    image: "https://placehold.co/800x600/e2e8f0/475569?text=Headphones",
    description: "Premium wireless headphones with noise cancellation and 30-hour battery life. Experience superior sound quality and comfort.",
    rating: 4.5,
    reviews: 128,
  },
  "2": {
    id: 2,
    name: "Smart Watch",
    price: 249.99,
    category: "Electronics",
    inStock: true,
    image: "https://placehold.co/800x600/e2e8f0/475569?text=Smart+Watch",
    description: "Feature-rich smartwatch with health tracking, GPS, and 5-day battery life. Stay connected and active.",
    rating: 4.8,
    reviews: 256,
  },
  "3": {
    id: 3,
    name: "Laptop Backpack",
    price: 49.99,
    category: "Accessories",
    inStock: true,
    image: "https://placehold.co/800x600/e2e8f0/475569?text=Backpack",
    description: "Durable laptop backpack with multiple compartments and water-resistant material. Perfect for daily commute.",
    rating: 4.3,
    reviews: 89,
  },
  "4": {
    id: 4,
    name: "Portable Charger",
    price: 29.99,
    category: "Electronics",
    inStock: false,
    image: "https://placehold.co/800x600/e2e8f0/475569?text=Charger",
    description: "High-capacity portable charger with fast charging support. Keep your devices powered on the go.",
    rating: 4.6,
    reviews: 432,
  },
  "5": {
    id: 5,
    name: "Bluetooth Speaker",
    price: 79.99,
    category: "Electronics",
    inStock: true,
    image: "https://placehold.co/800x600/e2e8f0/475569?text=Speaker",
    description: "Portable Bluetooth speaker with 360-degree sound and waterproof design. Perfect for outdoor adventures.",
    rating: 4.7,
    reviews: 312,
  },
  "6": {
    id: 6,
    name: "Phone Case",
    price: 19.99,
    category: "Accessories",
    inStock: true,
    image: "https://placehold.co/800x600/e2e8f0/475569?text=Phone+Case",
    description: "Slim protective phone case with military-grade drop protection. Available in multiple colors.",
    rating: 4.4,
    reviews: 567,
  },
};

export default function ProductDetailPage() {
  const params = useParams();
  const productId = params.id as string;
  const product = products[productId];

  if (!product) {
    return (
      <div className="container py-16 text-center">
        <h1 className="mb-4 text-3xl font-bold">Product Not Found</h1>
        <Link href="/products">
          <Button>
            <ArrowLeft className="mr-2 h-4 w-4" />
            Back to Products
          </Button>
        </Link>
      </div>
    );
  }

  return (
    <div className="container py-8">
      <Link href="/products" className="mb-6 inline-flex items-center text-sm text-muted-foreground hover:text-foreground">
        <ArrowLeft className="mr-2 h-4 w-4" />
        Back to Products
      </Link>

      <div className="grid gap-8 md:grid-cols-2">
        {/* Product Image */}
        <div className="relative aspect-[4/3] overflow-hidden rounded-lg bg-muted">
          <Image
            src={product.image}
            alt={product.name}
            width={800}
            height={600}
            className="h-full w-full object-cover"
            unoptimized
          />
          {!product.inStock && (
            <Badge variant="destructive" className="absolute right-4 top-4">
              Out of Stock
            </Badge>
          )}
        </div>

        {/* Product Info */}
        <div className="flex flex-col gap-6">
          <div>
            <Badge variant="secondary" className="mb-2">
              {product.category}
            </Badge>
            <h1 className="mb-2 text-3xl font-bold md:text-4xl">
              {product.name}
            </h1>
            
            {/* Rating */}
            <div className="flex items-center gap-2">
              <div className="flex items-center">
                {Array.from({ length: 5 }).map((_, i) => (
                  <Star
                    key={i}
                    className={`h-5 w-5 ${
                      i < Math.floor(product.rating)
                        ? "fill-yellow-400 text-yellow-400"
                        : "text-gray-300"
                    }`}
                  />
                ))}
              </div>
              <span className="text-sm text-muted-foreground">
                {product.rating} ({product.reviews} reviews)
              </span>
            </div>
          </div>

          <div className="text-4xl font-bold text-primary">
            ${product.price.toFixed(2)}
          </div>

          <p className="text-muted-foreground">{product.description}</p>

          <Card>
            <CardContent className="pt-6">
              <div className="space-y-2">
                <div className="flex justify-between text-sm">
                  <span className="text-muted-foreground">Availability:</span>
                  <span className={product.inStock ? "text-green-600" : "text-red-600"}>
                    {product.inStock ? "In Stock" : "Out of Stock"}
                  </span>
                </div>
                <div className="flex justify-between text-sm">
                  <span className="text-muted-foreground">Category:</span>
                  <span>{product.category}</span>
                </div>
                <div className="flex justify-between text-sm">
                  <span className="text-muted-foreground">Shipping:</span>
                  <span>Free shipping on orders over $50</span>
                </div>
              </div>
            </CardContent>
          </Card>

          <div className="flex gap-4">
            <Button
              size="lg"
              className="flex-1"
              disabled={!product.inStock}
            >
              <ShoppingCart className="mr-2 h-5 w-5" />
              Add to Cart
            </Button>
          </div>
        </div>
      </div>
    </div>
  );
}
