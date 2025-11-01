"use client";

import { useParams, useRouter } from "next/navigation";
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "@/components/ui/card";
import { Button } from "@/components/ui/button";
import { Badge } from "@/components/ui/badge";
import { Separator } from "@/components/ui/separator";
import { ArrowLeft, ShoppingCart } from "lucide-react";

// Mock product data - in a real app, this would come from an API
const productsData = [
  {
    id: 1,
    name: "Wireless Headphones",
    description: "Premium noise-cancelling wireless headphones with 30-hour battery life",
    price: 199.99,
    category: "Electronics",
    inStock: true,
    features: [
      "Active noise cancellation",
      "30-hour battery life",
      "Bluetooth 5.0 connectivity",
      "Comfortable over-ear design",
      "Built-in microphone for calls"
    ],
  },
  {
    id: 2,
    name: "Smart Watch",
    description: "Feature-packed smartwatch with health tracking and notifications",
    price: 299.99,
    category: "Electronics",
    inStock: true,
    features: [
      "Heart rate monitoring",
      "GPS tracking",
      "Water resistant",
      "7-day battery life",
      "Smartphone notifications"
    ],
  },
  {
    id: 3,
    name: "Running Shoes",
    description: "Comfortable and durable running shoes for all terrains",
    price: 89.99,
    category: "Sports",
    inStock: true,
    features: [
      "Breathable mesh upper",
      "Cushioned midsole",
      "Durable rubber outsole",
      "Lightweight design",
      "Available in multiple colors"
    ],
  },
  {
    id: 4,
    name: "Laptop Backpack",
    description: "Spacious backpack with padded laptop compartment and USB charging port",
    price: 49.99,
    category: "Accessories",
    inStock: true,
    features: [
      "Fits laptops up to 15.6 inches",
      "USB charging port",
      "Water-resistant material",
      "Multiple compartments",
      "Padded shoulder straps"
    ],
  },
  {
    id: 5,
    name: "Stainless Steel Water Bottle",
    description: "Insulated water bottle keeps drinks cold for 24 hours",
    price: 24.99,
    category: "Accessories",
    inStock: false,
    features: [
      "Double-wall insulation",
      "Keeps cold for 24 hours",
      "Leak-proof design",
      "BPA-free",
      "Wide mouth for easy filling"
    ],
  },
  {
    id: 6,
    name: "Yoga Mat",
    description: "Non-slip yoga mat with carrying strap, perfect for all exercises",
    price: 34.99,
    category: "Sports",
    inStock: true,
    features: [
      "Non-slip surface",
      "6mm thickness for comfort",
      "Lightweight and portable",
      "Easy to clean",
      "Includes carrying strap"
    ],
  },
  {
    id: 7,
    name: "Portable Charger",
    description: "20000mAh power bank with fast charging and dual USB ports",
    price: 39.99,
    category: "Electronics",
    inStock: true,
    features: [
      "20000mAh capacity",
      "Fast charging support",
      "Dual USB ports",
      "LED power indicator",
      "Compact and portable"
    ],
  },
  {
    id: 8,
    name: "Coffee Maker",
    description: "Programmable coffee maker with thermal carafe and auto-brew feature",
    price: 79.99,
    category: "Home",
    inStock: true,
    features: [
      "Programmable timer",
      "Thermal carafe keeps coffee hot",
      "Auto-brew feature",
      "12-cup capacity",
      "Easy to clean"
    ],
  },
];

export default function ProductDetailPage() {
  const params = useParams();
  const router = useRouter();
  const productId = parseInt(params.id as string);
  
  const product = productsData.find((p) => p.id === productId);

  if (!product) {
    return (
      <div className="container mx-auto px-4 py-8">
        <Card>
          <CardHeader>
            <CardTitle>Product Not Found</CardTitle>
            <CardDescription>The product you&apos;re looking for doesn&apos;t exist.</CardDescription>
          </CardHeader>
          <CardContent>
            <Button onClick={() => router.push("/products")} className="gap-2">
              <ArrowLeft className="h-4 w-4" />
              Back to Products
            </Button>
          </CardContent>
        </Card>
      </div>
    );
  }

  return (
    <div className="container mx-auto px-4 py-8">
      <Button 
        variant="ghost" 
        onClick={() => router.back()} 
        className="mb-6 gap-2"
      >
        <ArrowLeft className="h-4 w-4" />
        Back
      </Button>

      <div className="grid grid-cols-1 lg:grid-cols-2 gap-8">
        {/* Product Image Placeholder */}
        <div className="aspect-square bg-zinc-100 rounded-lg flex items-center justify-center">
          <p className="text-zinc-400 text-lg">Product Image</p>
        </div>

        {/* Product Details */}
        <div className="flex flex-col gap-6">
          <div>
            <div className="flex items-start justify-between mb-2">
              <Badge variant="secondary">{product.category}</Badge>
              {!product.inStock && (
                <Badge variant="destructive">Out of Stock</Badge>
              )}
            </div>
            <h1 className="text-3xl md:text-4xl font-bold mb-4">{product.name}</h1>
            <p className="text-lg text-zinc-600">{product.description}</p>
          </div>

          <Separator />

          <div>
            <p className="text-3xl font-bold text-primary">
              ${product.price.toFixed(2)}
            </p>
          </div>

          <div>
            <h2 className="text-xl font-semibold mb-3">Features</h2>
            <ul className="space-y-2">
              {product.features.map((feature, index) => (
                <li key={index} className="flex items-start gap-2">
                  <span className="text-primary mt-1">•</span>
                  <span>{feature}</span>
                </li>
              ))}
            </ul>
          </div>

          <Separator />

          <div className="flex flex-col sm:flex-row gap-4">
            <Button 
              size="lg" 
              className="flex-1 gap-2"
              disabled={!product.inStock}
            >
              <ShoppingCart className="h-5 w-5" />
              Add to Cart
            </Button>
            <Button 
              size="lg" 
              variant="outline" 
              className="flex-1"
              disabled={!product.inStock}
            >
              Buy Now
            </Button>
          </div>

          {!product.inStock && (
            <p className="text-sm text-zinc-600 text-center">
              This product is currently out of stock. Check back later!
            </p>
          )}
        </div>
      </div>
    </div>
  );
}
