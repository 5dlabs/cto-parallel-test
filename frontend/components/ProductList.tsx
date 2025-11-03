"use client";

import Link from "next/link";
import { Button } from "@/components/ui/button";
import {
  Card,
  CardContent,
  CardDescription,
  CardFooter,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import { Badge } from "@/components/ui/badge";

// Mock product data - in real app, this would come from API
const products = [
  {
    id: 1,
    name: "Premium Wireless Headphones",
    price: 299.99,
    description: "High-quality wireless headphones with noise cancellation",
    category: "Electronics",
    stock: 15,
  },
  {
    id: 2,
    name: "Smart Watch Pro",
    price: 399.99,
    description: "Advanced fitness tracking and smart notifications",
    category: "Electronics",
    stock: 8,
  },
  {
    id: 3,
    name: "Ergonomic Office Chair",
    price: 549.99,
    description: "Comfortable chair with lumbar support",
    category: "Furniture",
    stock: 20,
  },
  {
    id: 4,
    name: "4K Ultra HD Monitor",
    price: 699.99,
    description: "27-inch display with stunning color accuracy",
    category: "Electronics",
    stock: 12,
  },
  {
    id: 5,
    name: "Mechanical Keyboard",
    price: 149.99,
    description: "RGB backlit gaming keyboard",
    category: "Electronics",
    stock: 25,
  },
  {
    id: 6,
    name: "Wireless Mouse",
    price: 79.99,
    description: "Precision wireless mouse with ergonomic design",
    category: "Electronics",
    stock: 30,
  },
];

export default function ProductList() {
  return (
    <div className="container mx-auto px-4 py-8">
      <div className="mb-8">
        <h1 className="text-4xl font-bold tracking-tight mb-2">Products</h1>
        <p className="text-muted-foreground">
          Browse our collection of premium products
        </p>
      </div>

      <div className="grid grid-cols-1 gap-6 sm:grid-cols-2 lg:grid-cols-3">
        {products.map((product) => (
          <Card key={product.id} className="flex flex-col">
            <CardHeader>
              <div className="flex items-start justify-between mb-2">
                <Badge variant="secondary">{product.category}</Badge>
                {product.stock < 10 && (
                  <Badge variant="destructive">Low Stock</Badge>
                )}
              </div>
              <CardTitle className="text-xl">{product.name}</CardTitle>
              <CardDescription className="line-clamp-2">
                {product.description}
              </CardDescription>
            </CardHeader>
            <CardContent className="flex-1">
              <div className="text-3xl font-bold text-primary">
                ${product.price.toFixed(2)}
              </div>
              <p className="text-sm text-muted-foreground mt-2">
                {product.stock} in stock
              </p>
            </CardContent>
            <CardFooter className="flex gap-2">
              <Link href={`/products/${product.id}`} className="flex-1">
                <Button variant="outline" className="w-full">
                  View Details
                </Button>
              </Link>
              <Button className="flex-1">Add to Cart</Button>
            </CardFooter>
          </Card>
        ))}
      </div>
    </div>
  );
}
