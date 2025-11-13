import Link from "next/link";
import { Card, CardContent, CardDescription, CardFooter, CardHeader, CardTitle } from "@/components/ui/card";
import { Button } from "@/components/ui/button";
import { Badge } from "@/components/ui/badge";

// Sample product data - in a real app, this would come from an API
const products = [
  {
    id: 1,
    name: "Premium Wireless Headphones",
    description: "High-quality sound with active noise cancellation",
    price: 299.99,
    category: "Electronics",
    inStock: true,
  },
  {
    id: 2,
    name: "Smart Watch Pro",
    description: "Track your fitness and stay connected",
    price: 399.99,
    category: "Electronics",
    inStock: true,
  },
  {
    id: 3,
    name: "Laptop Backpack",
    description: "Durable and spacious laptop bag",
    price: 79.99,
    category: "Accessories",
    inStock: true,
  },
  {
    id: 4,
    name: "Mechanical Keyboard",
    description: "RGB backlit gaming keyboard",
    price: 149.99,
    category: "Electronics",
    inStock: false,
  },
  {
    id: 5,
    name: "Wireless Mouse",
    description: "Ergonomic design for all-day comfort",
    price: 49.99,
    category: "Electronics",
    inStock: true,
  },
  {
    id: 6,
    name: "USB-C Hub",
    description: "7-in-1 connectivity solution",
    price: 59.99,
    category: "Accessories",
    inStock: true,
  },
  {
    id: 7,
    name: "Portable SSD",
    description: "1TB high-speed external storage",
    price: 179.99,
    category: "Storage",
    inStock: true,
  },
  {
    id: 8,
    name: "Webcam HD Pro",
    description: "1080p video calls with auto-focus",
    price: 89.99,
    category: "Electronics",
    inStock: true,
  },
];

export default function ProductsPage() {
  return (
    <div className="container mx-auto px-4 py-12">
      <div className="mb-8">
        <h1 className="text-4xl font-bold mb-4">Our Products</h1>
        <p className="text-lg text-muted-foreground">
          Browse our collection of high-quality products
        </p>
      </div>

      <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-6">
        {products.map((product) => (
          <Card key={product.id} className="flex flex-col">
            <CardHeader>
              <div className="flex items-start justify-between mb-2">
                <Badge variant="secondary">{product.category}</Badge>
                {!product.inStock && (
                  <Badge variant="destructive">Out of Stock</Badge>
                )}
              </div>
              <CardTitle className="line-clamp-2">{product.name}</CardTitle>
              <CardDescription className="line-clamp-2">
                {product.description}
              </CardDescription>
            </CardHeader>
            <CardContent className="flex-1">
              <p className="text-2xl font-bold">${product.price.toFixed(2)}</p>
            </CardContent>
            <CardFooter className="flex gap-2">
              <Link href={`/products/${product.id}`} className="flex-1">
                <Button variant="outline" className="w-full">
                  View Details
                </Button>
              </Link>
              <Button
                className="flex-1"
                disabled={!product.inStock}
                aria-label={`Add ${product.name} to cart`}
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
