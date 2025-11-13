import Link from "next/link";
import Image from "next/image";
import { Card, CardContent, CardFooter, CardHeader, CardTitle } from "@/components/ui/card";
import { Button } from "@/components/ui/button";
import { Badge } from "@/components/ui/badge";

// Mock product data - will be replaced with real API data later
const products = [
  {
    id: 1,
    name: "Wireless Headphones",
    price: 99.99,
    category: "Electronics",
    inStock: true,
    image: "https://placehold.co/400x300/e2e8f0/475569?text=Headphones",
  },
  {
    id: 2,
    name: "Smart Watch",
    price: 249.99,
    category: "Electronics",
    inStock: true,
    image: "https://placehold.co/400x300/e2e8f0/475569?text=Smart+Watch",
  },
  {
    id: 3,
    name: "Laptop Backpack",
    price: 49.99,
    category: "Accessories",
    inStock: true,
    image: "https://placehold.co/400x300/e2e8f0/475569?text=Backpack",
  },
  {
    id: 4,
    name: "Portable Charger",
    price: 29.99,
    category: "Electronics",
    inStock: false,
    image: "https://placehold.co/400x300/e2e8f0/475569?text=Charger",
  },
  {
    id: 5,
    name: "Bluetooth Speaker",
    price: 79.99,
    category: "Electronics",
    inStock: true,
    image: "https://placehold.co/400x300/e2e8f0/475569?text=Speaker",
  },
  {
    id: 6,
    name: "Phone Case",
    price: 19.99,
    category: "Accessories",
    inStock: true,
    image: "https://placehold.co/400x300/e2e8f0/475569?text=Phone+Case",
  },
];

export default function ProductsPage() {
  return (
    <div className="container py-8">
      <div className="mb-8">
        <h1 className="mb-2 text-3xl font-bold md:text-4xl">Products</h1>
        <p className="text-muted-foreground">
          Browse our collection of quality products
        </p>
      </div>

      <div className="grid gap-6 sm:grid-cols-2 lg:grid-cols-3">
        {products.map((product) => (
          <Card key={product.id} className="flex flex-col overflow-hidden">
            <CardHeader className="p-0">
              <div className="relative aspect-[4/3] w-full overflow-hidden bg-muted">
                <Image
                  src={product.image}
                  alt={product.name}
                  width={400}
                  height={300}
                  className="h-full w-full object-cover transition-transform hover:scale-105"
                  unoptimized
                />
                {!product.inStock && (
                  <Badge
                    variant="destructive"
                    className="absolute right-2 top-2"
                  >
                    Out of Stock
                  </Badge>
                )}
              </div>
            </CardHeader>
            <CardContent className="flex-1 p-4">
              <Badge variant="secondary" className="mb-2">
                {product.category}
              </Badge>
              <CardTitle className="mb-2 text-xl">{product.name}</CardTitle>
              <p className="text-2xl font-bold text-primary">
                ${product.price.toFixed(2)}
              </p>
            </CardContent>
            <CardFooter className="p-4 pt-0">
              <Link href={`/products/${product.id}`} className="w-full">
                <Button className="w-full" disabled={!product.inStock}>
                  {product.inStock ? "View Details" : "Out of Stock"}
                </Button>
              </Link>
            </CardFooter>
          </Card>
        ))}
      </div>
    </div>
  );
}
