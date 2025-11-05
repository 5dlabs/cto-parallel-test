"use client";

import Link from "next/link";
import { Card, CardContent, CardDescription, CardFooter, CardHeader, CardTitle } from "@/components/ui/card";
import { Button } from "@/components/ui/button";
import { Badge } from "@/components/ui/badge";

interface Product {
  id: number;
  name: string;
  description: string;
  price: number;
  category: string;
  inStock: boolean;
}

export default function ProductListPage() {
  // In a real app, this would come from an API
  const products: Product[] = [
    {
      id: 1,
      name: "Wireless Headphones",
      description: "Premium noise-cancelling wireless headphones",
      price: 299.99,
      category: "Electronics",
      inStock: true,
    },
    {
      id: 2,
      name: "Smart Watch",
      description: "Fitness tracker with heart rate monitor",
      price: 199.99,
      category: "Electronics",
      inStock: true,
    },
    {
      id: 3,
      name: "Laptop Backpack",
      description: "Durable backpack with padded laptop compartment",
      price: 79.99,
      category: "Accessories",
      inStock: true,
    },
    {
      id: 4,
      name: "Mechanical Keyboard",
      description: "RGB backlit mechanical gaming keyboard",
      price: 149.99,
      category: "Electronics",
      inStock: false,
    },
    {
      id: 5,
      name: "USB-C Hub",
      description: "7-in-1 USB-C hub with HDMI and USB 3.0",
      price: 49.99,
      category: "Accessories",
      inStock: true,
    },
    {
      id: 6,
      name: "Wireless Mouse",
      description: "Ergonomic wireless mouse with precision tracking",
      price: 39.99,
      category: "Electronics",
      inStock: true,
    },
  ];

  return (
    <div className="container mx-auto px-4 py-8 md:px-6">
      <div className="mb-8">
        <h1 className="text-3xl font-bold md:text-4xl">Products</h1>
        <p className="mt-2 text-muted-foreground">
          Browse our collection of quality products
        </p>
      </div>

      <div className="grid gap-6 sm:grid-cols-2 lg:grid-cols-3">
        {products.map((product) => (
          <Card key={product.id} className="flex flex-col">
            <CardHeader>
              <div className="flex items-start justify-between">
                <CardTitle className="line-clamp-1">{product.name}</CardTitle>
                {!product.inStock && (
                  <Badge variant="secondary">Out of Stock</Badge>
                )}
              </div>
              <CardDescription className="line-clamp-2">
                {product.description}
              </CardDescription>
            </CardHeader>
            <CardContent className="flex-1">
              <div className="flex items-baseline gap-2">
                <span className="text-2xl font-bold">
                  ${product.price.toFixed(2)}
                </span>
                <Badge variant="outline">{product.category}</Badge>
              </div>
            </CardContent>
            <CardFooter>
              <Link href={`/products/${product.id}`} className="w-full">
                <Button className="w-full" variant={product.inStock ? "default" : "secondary"}>
                  {product.inStock ? "View Details" : "View Product"}
                </Button>
              </Link>
            </CardFooter>
          </Card>
        ))}
      </div>
    </div>
  );
}
