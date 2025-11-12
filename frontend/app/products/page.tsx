import Link from "next/link";
import { Card, CardContent, CardDescription, CardFooter, CardHeader, CardTitle } from "@/components/ui/card";
import { Button } from "@/components/ui/button";
import { Badge } from "@/components/ui/badge";

// Mock product data - in production this would come from an API
const products = [
  {
    id: 1,
    name: "Wireless Headphones",
    price: 89.99,
    category: "Electronics",
    description: "High-quality wireless headphones with noise cancellation",
    inStock: true,
  },
  {
    id: 2,
    name: "Smart Watch",
    price: 199.99,
    category: "Electronics",
    description: "Feature-rich smartwatch with health tracking",
    inStock: true,
  },
  {
    id: 3,
    name: "Laptop Stand",
    price: 49.99,
    category: "Accessories",
    description: "Ergonomic aluminum laptop stand",
    inStock: true,
  },
  {
    id: 4,
    name: "USB-C Hub",
    price: 39.99,
    category: "Accessories",
    description: "Multi-port USB-C hub with HDMI and SD card reader",
    inStock: false,
  },
  {
    id: 5,
    name: "Mechanical Keyboard",
    price: 129.99,
    category: "Electronics",
    description: "RGB mechanical keyboard with custom switches",
    inStock: true,
  },
  {
    id: 6,
    name: "Webcam HD",
    price: 79.99,
    category: "Electronics",
    description: "1080p HD webcam with built-in microphone",
    inStock: true,
  },
];

export default function ProductList() {
  return (
    <div className="container mx-auto px-4 py-8 sm:px-6 lg:px-8">
      <div className="flex flex-col space-y-6">
        <div>
          <h1 className="text-3xl font-bold tracking-tight">Products</h1>
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
                <div className="flex items-center justify-between">
                  <span className="text-2xl font-bold">
                    ${product.price.toFixed(2)}
                  </span>
                  <Badge variant="outline">{product.category}</Badge>
                </div>
              </CardContent>
              <CardFooter>
                <Link href={`/products/${product.id}`} className="w-full">
                  <Button className="w-full" disabled={!product.inStock}>
                    View Details
                  </Button>
                </Link>
              </CardFooter>
            </Card>
          ))}
        </div>
      </div>
    </div>
  );
}
