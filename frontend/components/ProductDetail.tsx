"use client";

import { useState } from "react";
import { useRouter } from "next/navigation";
import { Button } from "@/components/ui/button";
import { Badge } from "@/components/ui/badge";
import {
  Card,
  CardContent,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import { ArrowLeft, ShoppingCart, Package, Truck, Shield } from "lucide-react";

// Mock product data - in real app, this would come from API
const products = [
  {
    id: 1,
    name: "Premium Wireless Headphones",
    price: 299.99,
    description: "High-quality wireless headphones with noise cancellation and superior sound quality. Perfect for music lovers and professionals.",
    category: "Electronics",
    stock: 15,
    features: [
      "Active Noise Cancellation",
      "40-hour battery life",
      "Bluetooth 5.0 connectivity",
      "Premium memory foam cushions",
    ],
  },
  {
    id: 2,
    name: "Smart Watch Pro",
    price: 399.99,
    description: "Advanced fitness tracking and smart notifications with health monitoring features.",
    category: "Electronics",
    stock: 8,
    features: [
      "Heart rate monitoring",
      "Sleep tracking",
      "GPS navigation",
      "Water resistant",
    ],
  },
  {
    id: 3,
    name: "Ergonomic Office Chair",
    price: 549.99,
    description: "Comfortable office chair with lumbar support and adjustable features.",
    category: "Furniture",
    stock: 20,
    features: [
      "Adjustable lumbar support",
      "Height adjustable",
      "Breathable mesh back",
      "360-degree swivel",
    ],
  },
  {
    id: 4,
    name: "4K Ultra HD Monitor",
    price: 699.99,
    description: "27-inch display with stunning color accuracy for professionals and gamers.",
    category: "Electronics",
    stock: 12,
    features: [
      "3840 x 2160 resolution",
      "HDR10 support",
      "144Hz refresh rate",
      "USB-C connectivity",
    ],
  },
  {
    id: 5,
    name: "Mechanical Keyboard",
    price: 149.99,
    description: "RGB backlit gaming keyboard with mechanical switches.",
    category: "Electronics",
    stock: 25,
    features: [
      "Mechanical switches",
      "RGB backlighting",
      "Programmable keys",
      "Aluminum frame",
    ],
  },
  {
    id: 6,
    name: "Wireless Mouse",
    price: 79.99,
    description: "Precision wireless mouse with ergonomic design.",
    category: "Electronics",
    stock: 30,
    features: [
      "Wireless connectivity",
      "Ergonomic design",
      "Adjustable DPI",
      "Long battery life",
    ],
  },
];

interface ProductDetailProps {
  id: string;
}

export default function ProductDetail({ id }: ProductDetailProps) {
  const router = useRouter();
  const [quantity, setQuantity] = useState(1);
  
  const product = products.find((p) => p.id === parseInt(id));

  if (!product) {
    return (
      <div className="container mx-auto px-4 py-16 text-center">
        <h1 className="text-3xl font-bold mb-4">Product Not Found</h1>
        <p className="text-muted-foreground mb-8">
          The product you&apos;re looking for doesn&apos;t exist.
        </p>
        <Button onClick={() => router.push("/products")}>
          Back to Products
        </Button>
      </div>
    );
  }

  const handleAddToCart = () => {
    // In a real app, this would add to cart context/state
    alert(`Added ${quantity} ${product.name} to cart`);
  };

  return (
    <div className="container mx-auto px-4 py-8">
      <Button
        variant="ghost"
        className="mb-6"
        onClick={() => router.push("/products")}
      >
        <ArrowLeft className="mr-2 h-4 w-4" />
        Back to Products
      </Button>

      <div className="grid gap-8 lg:grid-cols-2">
        {/* Product Image Placeholder */}
        <div className="aspect-square bg-muted rounded-lg flex items-center justify-center">
          <Package className="h-32 w-32 text-muted-foreground" />
        </div>

        {/* Product Details */}
        <div className="flex flex-col">
          <div className="mb-4">
            <Badge variant="secondary" className="mb-2">
              {product.category}
            </Badge>
            <h1 className="text-4xl font-bold tracking-tight mb-2">
              {product.name}
            </h1>
            <p className="text-3xl font-bold text-primary mb-4">
              ${product.price.toFixed(2)}
            </p>
            <p className="text-muted-foreground mb-4">{product.description}</p>
            <div className="flex items-center gap-2 mb-6">
              <Badge variant={product.stock > 10 ? "default" : "destructive"}>
                {product.stock > 10 ? "In Stock" : "Low Stock"}
              </Badge>
              <span className="text-sm text-muted-foreground">
                {product.stock} available
              </span>
            </div>
          </div>

          {/* Features */}
          <Card className="mb-6">
            <CardHeader>
              <CardTitle>Key Features</CardTitle>
            </CardHeader>
            <CardContent>
              <ul className="space-y-2">
                {product.features.map((feature, index) => (
                  <li key={index} className="flex items-start">
                    <span className="mr-2">•</span>
                    <span>{feature}</span>
                  </li>
                ))}
              </ul>
            </CardContent>
          </Card>

          {/* Add to Cart */}
          <div className="space-y-4">
            <div className="flex items-end gap-4">
              <div className="flex-1">
                <Label htmlFor="quantity">Quantity</Label>
                <Input
                  id="quantity"
                  type="number"
                  min="1"
                  max={product.stock}
                  value={quantity}
                  onChange={(e) => setQuantity(Math.max(1, parseInt(e.target.value) || 1))}
                  className="w-24"
                />
              </div>
              <Button size="lg" className="flex-1" onClick={handleAddToCart}>
                <ShoppingCart className="mr-2 h-5 w-5" />
                Add to Cart
              </Button>
            </div>
          </div>

          {/* Benefits */}
          <div className="grid grid-cols-3 gap-4 mt-8">
            <div className="text-center">
              <div className="mx-auto mb-2 flex h-10 w-10 items-center justify-center rounded-full bg-primary/10">
                <Truck className="h-5 w-5 text-primary" />
              </div>
              <p className="text-xs text-muted-foreground">Fast Delivery</p>
            </div>
            <div className="text-center">
              <div className="mx-auto mb-2 flex h-10 w-10 items-center justify-center rounded-full bg-primary/10">
                <Shield className="h-5 w-5 text-primary" />
              </div>
              <p className="text-xs text-muted-foreground">Secure Payment</p>
            </div>
            <div className="text-center">
              <div className="mx-auto mb-2 flex h-10 w-10 items-center justify-center rounded-full bg-primary/10">
                <Package className="h-5 w-5 text-primary" />
              </div>
              <p className="text-xs text-muted-foreground">Easy Returns</p>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
}
