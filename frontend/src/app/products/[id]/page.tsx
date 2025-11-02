"use client";

import { use } from "react";
import Link from "next/link";
import { Button } from "@/components/ui/button";
import { Badge } from "@/components/ui/badge";
import { Card, CardContent } from "@/components/ui/card";
import { ShoppingCart, ArrowLeft, Package, Truck, Shield } from "lucide-react";

// Mock product data - will be replaced with API calls
const getProduct = (id: string) => {
  const products = [
    {
      id: 1,
      name: "Wireless Headphones",
      price: 79.99,
      category: "Electronics",
      image: "https://via.placeholder.com/600x400/4F46E5/ffffff?text=Headphones",
      inStock: true,
      description:
        "Premium wireless headphones with active noise cancellation, 30-hour battery life, and superior sound quality. Perfect for music lovers and professionals.",
      features: [
        "Active Noise Cancellation",
        "30-hour battery life",
        "Bluetooth 5.0",
        "Comfortable over-ear design",
        "Built-in microphone",
      ],
    },
    {
      id: 2,
      name: "Smart Watch",
      price: 199.99,
      category: "Electronics",
      image: "https://via.placeholder.com/600x400/7C3AED/ffffff?text=Smart+Watch",
      inStock: true,
      description:
        "Advanced fitness and health tracking smartwatch with heart rate monitoring, GPS, and smartphone notifications. Stay connected and healthy.",
      features: [
        "Heart rate monitoring",
        "GPS tracking",
        "Water resistant",
        "7-day battery life",
        "Smartphone notifications",
      ],
    },
    {
      id: 3,
      name: "Laptop Backpack",
      price: 49.99,
      category: "Accessories",
      image: "https://via.placeholder.com/600x400/EC4899/ffffff?text=Backpack",
      inStock: true,
      description:
        "Durable and spacious laptop backpack with multiple compartments, USB charging port, and water-resistant material. Perfect for work and travel.",
      features: [
        "Fits 15.6 inch laptops",
        "USB charging port",
        "Water-resistant",
        "Multiple compartments",
        "Comfortable padded straps",
      ],
    },
  ];

  return products.find((p) => p.id === parseInt(id)) || products[0];
};

export default function ProductDetailPage({
  params,
}: {
  params: Promise<{ id: string }>;
}) {
  const { id } = use(params);
  const product = getProduct(id);

  const handleAddToCart = () => {
    console.log(`Adding product ${product.id} to cart`);
  };

  return (
    <div className="container mx-auto px-4 sm:px-6 lg:px-8 py-8">
      {/* Back Button */}
      <Link href="/products">
        <Button variant="ghost" className="mb-6">
          <ArrowLeft className="mr-2 h-4 w-4" />
          Back to Products
        </Button>
      </Link>

      <div className="grid grid-cols-1 lg:grid-cols-2 gap-8">
        {/* Product Image */}
        <div className="relative overflow-hidden rounded-lg bg-muted aspect-[4/3]">
          <img
            src={product.image}
            alt={product.name}
            className="object-cover w-full h-full"
          />
        </div>

        {/* Product Details */}
        <div className="flex flex-col">
          <div className="mb-4">
            <Badge variant="secondary">{product.category}</Badge>
          </div>
          <h1 className="text-3xl md:text-4xl font-bold mb-4">
            {product.name}
          </h1>
          <div className="mb-6">
            <span className="text-4xl font-bold">${product.price.toFixed(2)}</span>
          </div>
          <p className="text-muted-foreground mb-6">{product.description}</p>

          {/* Features */}
          <div className="mb-6">
            <h3 className="font-semibold mb-3">Key Features:</h3>
            <ul className="space-y-2">
              {product.features.map((feature, index) => (
                <li key={index} className="flex items-start">
                  <span className="mr-2 text-primary">•</span>
                  <span className="text-sm">{feature}</span>
                </li>
              ))}
            </ul>
          </div>

          {/* Stock Status */}
          <div className="mb-6">
            {product.inStock ? (
              <Badge variant="default" className="bg-green-600">
                In Stock
              </Badge>
            ) : (
              <Badge variant="destructive">Out of Stock</Badge>
            )}
          </div>

          {/* Add to Cart Button */}
          <Button
            size="lg"
            className="w-full mb-6"
            onClick={handleAddToCart}
            disabled={!product.inStock}
          >
            <ShoppingCart className="mr-2 h-5 w-5" />
            {product.inStock ? "Add to Cart" : "Out of Stock"}
          </Button>

          {/* Additional Info */}
          <div className="grid grid-cols-1 sm:grid-cols-3 gap-4 mt-4">
            <Card>
              <CardContent className="flex flex-col items-center p-4">
                <Package className="h-8 w-8 mb-2 text-primary" />
                <span className="text-xs text-center text-muted-foreground">
                  Free Shipping
                </span>
              </CardContent>
            </Card>
            <Card>
              <CardContent className="flex flex-col items-center p-4">
                <Truck className="h-8 w-8 mb-2 text-primary" />
                <span className="text-xs text-center text-muted-foreground">
                  2-3 Day Delivery
                </span>
              </CardContent>
            </Card>
            <Card>
              <CardContent className="flex flex-col items-center p-4">
                <Shield className="h-8 w-8 mb-2 text-primary" />
                <span className="text-xs text-center text-muted-foreground">
                  1 Year Warranty
                </span>
              </CardContent>
            </Card>
          </div>
        </div>
      </div>
    </div>
  );
}
