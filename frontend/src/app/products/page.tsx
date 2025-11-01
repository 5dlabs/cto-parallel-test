"use client";

import Link from "next/link";
import { Card, CardContent, CardDescription, CardFooter, CardHeader, CardTitle } from "@/components/ui/card";
import { Button } from "@/components/ui/button";
import { Badge } from "@/components/ui/badge";

// Mock product data - in a real app, this would come from an API
const products = [
  {
    id: 1,
    name: "Wireless Headphones",
    description: "Premium noise-cancelling wireless headphones with 30-hour battery life",
    price: 199.99,
    category: "Electronics",
    inStock: true,
  },
  {
    id: 2,
    name: "Smart Watch",
    description: "Feature-packed smartwatch with health tracking and notifications",
    price: 299.99,
    category: "Electronics",
    inStock: true,
  },
  {
    id: 3,
    name: "Running Shoes",
    description: "Comfortable and durable running shoes for all terrains",
    price: 89.99,
    category: "Sports",
    inStock: true,
  },
  {
    id: 4,
    name: "Laptop Backpack",
    description: "Spacious backpack with padded laptop compartment and USB charging port",
    price: 49.99,
    category: "Accessories",
    inStock: true,
  },
  {
    id: 5,
    name: "Stainless Steel Water Bottle",
    description: "Insulated water bottle keeps drinks cold for 24 hours",
    price: 24.99,
    category: "Accessories",
    inStock: false,
  },
  {
    id: 6,
    name: "Yoga Mat",
    description: "Non-slip yoga mat with carrying strap, perfect for all exercises",
    price: 34.99,
    category: "Sports",
    inStock: true,
  },
  {
    id: 7,
    name: "Portable Charger",
    description: "20000mAh power bank with fast charging and dual USB ports",
    price: 39.99,
    category: "Electronics",
    inStock: true,
  },
  {
    id: 8,
    name: "Coffee Maker",
    description: "Programmable coffee maker with thermal carafe and auto-brew feature",
    price: 79.99,
    category: "Home",
    inStock: true,
  },
];

export default function ProductsPage() {
  return (
    <div className="container mx-auto px-4 py-8">
      <div className="mb-8">
        <h1 className="text-3xl md:text-4xl font-bold mb-2">Products</h1>
        <p className="text-zinc-600">Browse our collection of quality products</p>
      </div>

      <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-6">
        {products.map((product) => (
          <Card key={product.id} className="flex flex-col">
            <CardHeader>
              <div className="flex justify-between items-start mb-2">
                <Badge variant="secondary">{product.category}</Badge>
                {!product.inStock && (
                  <Badge variant="destructive">Out of Stock</Badge>
                )}
              </div>
              <CardTitle className="text-xl">{product.name}</CardTitle>
              <CardDescription className="line-clamp-2">
                {product.description}
              </CardDescription>
            </CardHeader>
            <CardContent className="flex-1">
              <p className="text-2xl font-bold text-primary">
                ${product.price.toFixed(2)}
              </p>
            </CardContent>
            <CardFooter className="flex gap-2">
              <Link href={`/products/${product.id}`} className="flex-1">
                <Button variant="outline" className="w-full">
                  View Details
                </Button>
              </Link>
              <Button 
                disabled={!product.inStock}
                className="flex-1"
              >
                Add to Cart
              </Button>
            </CardFooter>
          </Card>
        ))}
      </div>
    </div>
  );
}
