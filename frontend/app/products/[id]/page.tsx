"use client";

import { useParams, useRouter } from "next/navigation";
import { Button } from "@/components/ui/button";
import { Badge } from "@/components/ui/badge";
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "@/components/ui/card";
import { ArrowLeft, ShoppingCart } from "lucide-react";

// Mock product data - in production this would come from an API
const products = [
  {
    id: 1,
    name: "Wireless Headphones",
    price: 89.99,
    category: "Electronics",
    description: "High-quality wireless headphones with noise cancellation",
    inStock: true,
    details: "Premium wireless headphones featuring active noise cancellation, 30-hour battery life, and comfortable over-ear design. Perfect for music lovers and professionals.",
    specs: ["Bluetooth 5.0", "30h Battery", "Active Noise Cancellation", "Foldable Design"],
  },
  {
    id: 2,
    name: "Smart Watch",
    price: 199.99,
    category: "Electronics",
    description: "Feature-rich smartwatch with health tracking",
    inStock: true,
    details: "Advanced smartwatch with comprehensive health and fitness tracking. Monitor your heart rate, sleep, and daily activities with style.",
    specs: ["Heart Rate Monitor", "GPS Tracking", "Water Resistant", "7-Day Battery"],
  },
  {
    id: 3,
    name: "Laptop Stand",
    price: 49.99,
    category: "Accessories",
    description: "Ergonomic aluminum laptop stand",
    inStock: true,
    details: "Premium aluminum laptop stand designed to improve posture and increase productivity. Compatible with all laptop sizes.",
    specs: ["Aluminum Build", "Adjustable Height", "Cable Management", "Non-Slip Base"],
  },
  {
    id: 4,
    name: "USB-C Hub",
    price: 39.99,
    category: "Accessories",
    description: "Multi-port USB-C hub with HDMI and SD card reader",
    inStock: false,
    details: "Versatile USB-C hub expanding your device connectivity. Features multiple ports for all your peripherals.",
    specs: ["HDMI 4K Output", "SD/microSD Reader", "3x USB 3.0 Ports", "100W Power Delivery"],
  },
  {
    id: 5,
    name: "Mechanical Keyboard",
    price: 129.99,
    category: "Electronics",
    description: "RGB mechanical keyboard with custom switches",
    inStock: true,
    details: "Professional mechanical keyboard with customizable RGB lighting and premium switches for the ultimate typing experience.",
    specs: ["Mechanical Switches", "RGB Backlighting", "USB-C Connection", "N-Key Rollover"],
  },
  {
    id: 6,
    name: "Webcam HD",
    price: 79.99,
    category: "Electronics",
    description: "1080p HD webcam with built-in microphone",
    inStock: true,
    details: "Crystal-clear 1080p webcam perfect for video calls and streaming. Features auto-focus and built-in noise-cancelling microphone.",
    specs: ["1080p Resolution", "Auto Focus", "Built-in Mic", "Wide Angle Lens"],
  },
];

export default function ProductDetail() {
  const params = useParams();
  const router = useRouter();
  const productId = parseInt(params.id as string);
  const product = products.find(p => p.id === productId);

  if (!product) {
    return (
      <div className="container mx-auto px-4 py-8 sm:px-6 lg:px-8">
        <div className="flex flex-col items-center justify-center space-y-4 py-16">
          <h1 className="text-2xl font-bold">Product Not Found</h1>
          <p className="text-muted-foreground">The product you&apos;re looking for doesn&apos;t exist.</p>
          <Button onClick={() => router.push("/products")}>
            <ArrowLeft className="mr-2 h-4 w-4" aria-hidden="true" />
            Back to Products
          </Button>
        </div>
      </div>
    );
  }

  return (
    <div className="container mx-auto px-4 py-8 sm:px-6 lg:px-8">
      <Button
        variant="ghost"
        onClick={() => router.back()}
        className="mb-6"
        aria-label="Go back"
      >
        <ArrowLeft className="mr-2 h-4 w-4" aria-hidden="true" />
        Back
      </Button>

      <div className="grid gap-8 md:grid-cols-2">
        {/* Product Image Placeholder */}
        <div className="aspect-square rounded-lg border bg-muted flex items-center justify-center">
          <span className="text-4xl text-muted-foreground">{product.name[0]}</span>
        </div>

        {/* Product Details */}
        <div className="flex flex-col space-y-4">
          <div>
            <div className="flex items-center gap-2 mb-2">
              <Badge variant="outline">{product.category}</Badge>
              {!product.inStock && (
                <Badge variant="secondary">Out of Stock</Badge>
              )}
            </div>
            <h1 className="text-3xl font-bold tracking-tight">{product.name}</h1>
            <p className="mt-2 text-xl font-semibold">${product.price.toFixed(2)}</p>
          </div>

          <p className="text-muted-foreground">{product.description}</p>

          <Card>
            <CardHeader>
              <CardTitle className="text-lg">Product Details</CardTitle>
              <CardDescription>{product.details}</CardDescription>
            </CardHeader>
            <CardContent>
              <h4 className="font-semibold mb-2">Specifications:</h4>
              <ul className="list-disc list-inside space-y-1 text-sm text-muted-foreground">
                {product.specs.map((spec, index) => (
                  <li key={index}>{spec}</li>
                ))}
              </ul>
            </CardContent>
          </Card>

          <div className="flex gap-4 pt-4">
            <Button
              size="lg"
              className="flex-1 gap-2"
              disabled={!product.inStock}
              onClick={() => {
                // In production, this would add to cart
                alert("Added to cart!");
              }}
            >
              <ShoppingCart className="h-4 w-4" aria-hidden="true" />
              Add to Cart
            </Button>
          </div>
        </div>
      </div>
    </div>
  );
}
