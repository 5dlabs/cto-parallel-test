"use client";

import Link from "next/link";
import { Card, CardContent, CardFooter, CardHeader, CardTitle } from "@/components/ui/card";
import { Button } from "@/components/ui/button";
import { Badge } from "@/components/ui/badge";
import { ShoppingCart } from "lucide-react";

// Mock product data - will be replaced with API calls
const products = [
  {
    id: 1,
    name: "Wireless Headphones",
    price: 79.99,
    category: "Electronics",
    image: "https://via.placeholder.com/300x200/4F46E5/ffffff?text=Headphones",
    inStock: true,
  },
  {
    id: 2,
    name: "Smart Watch",
    price: 199.99,
    category: "Electronics",
    image: "https://via.placeholder.com/300x200/7C3AED/ffffff?text=Smart+Watch",
    inStock: true,
  },
  {
    id: 3,
    name: "Laptop Backpack",
    price: 49.99,
    category: "Accessories",
    image: "https://via.placeholder.com/300x200/EC4899/ffffff?text=Backpack",
    inStock: true,
  },
  {
    id: 4,
    name: "USB-C Cable",
    price: 14.99,
    category: "Accessories",
    image: "https://via.placeholder.com/300x200/10B981/ffffff?text=USB-C+Cable",
    inStock: true,
  },
  {
    id: 5,
    name: "Mechanical Keyboard",
    price: 129.99,
    category: "Electronics",
    image: "https://via.placeholder.com/300x200/F59E0B/ffffff?text=Keyboard",
    inStock: false,
  },
  {
    id: 6,
    name: "Wireless Mouse",
    price: 39.99,
    category: "Electronics",
    image: "https://via.placeholder.com/300x200/EF4444/ffffff?text=Mouse",
    inStock: true,
  },
  {
    id: 7,
    name: "Phone Case",
    price: 24.99,
    category: "Accessories",
    image: "https://via.placeholder.com/300x200/3B82F6/ffffff?text=Phone+Case",
    inStock: true,
  },
  {
    id: 8,
    name: "Portable Charger",
    price: 34.99,
    category: "Accessories",
    image: "https://via.placeholder.com/300x200/8B5CF6/ffffff?text=Charger",
    inStock: true,
  },
];

export default function ProductsPage() {
  const handleAddToCart = (productId: number) => {
    // This will be replaced with actual cart logic
    console.log(`Adding product ${productId} to cart`);
  };

  return (
    <div className="container mx-auto px-4 sm:px-6 lg:px-8 py-8">
      {/* Page Header */}
      <div className="mb-8">
        <h1 className="text-3xl md:text-4xl font-bold mb-2">Our Products</h1>
        <p className="text-muted-foreground">
          Browse our wide selection of quality products
        </p>
      </div>

      {/* Product Grid */}
      <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-6">
        {products.map((product) => (
          <Card key={product.id} className="flex flex-col">
            <CardHeader className="p-0">
              <Link href={`/products/${product.id}`}>
                <div className="relative overflow-hidden rounded-t-lg aspect-video bg-muted">
                  <img
                    src={product.image}
                    alt={product.name}
                    className="object-cover w-full h-full hover:scale-105 transition-transform duration-300"
                  />
                </div>
              </Link>
            </CardHeader>
            <CardContent className="flex-1 p-4">
              <div className="mb-2">
                <Badge variant="secondary" className="text-xs">
                  {product.category}
                </Badge>
              </div>
              <Link href={`/products/${product.id}`}>
                <CardTitle className="text-lg hover:text-primary transition-colors">
                  {product.name}
                </CardTitle>
              </Link>
              <div className="mt-2">
                <span className="text-2xl font-bold">
                  ${product.price.toFixed(2)}
                </span>
              </div>
              {!product.inStock && (
                <Badge variant="destructive" className="mt-2">
                  Out of Stock
                </Badge>
              )}
            </CardContent>
            <CardFooter className="p-4 pt-0">
              <Button
                className="w-full"
                onClick={() => handleAddToCart(product.id)}
                disabled={!product.inStock}
              >
                <ShoppingCart className="mr-2 h-4 w-4" />
                {product.inStock ? "Add to Cart" : "Out of Stock"}
              </Button>
            </CardFooter>
          </Card>
        ))}
      </div>
    </div>
  );
}
