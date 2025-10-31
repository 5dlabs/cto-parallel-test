import Link from "next/link"
import { Card, CardContent, CardFooter, CardHeader, CardTitle } from "@/components/ui/card"
import { Button } from "@/components/ui/button"
import { Badge } from "@/components/ui/badge"

// Mock product data - will be replaced with API integration
const products = [
  {
    id: 1,
    name: "Wireless Headphones",
    description: "Premium noise-cancelling wireless headphones with long battery life",
    price: 129.99,
    category: "Electronics",
    inStock: true,
  },
  {
    id: 2,
    name: "Smart Watch",
    description: "Feature-rich smartwatch with health tracking and notifications",
    price: 249.99,
    category: "Electronics",
    inStock: true,
  },
  {
    id: 3,
    name: "Laptop Backpack",
    description: "Durable and stylish backpack perfect for daily commute",
    price: 59.99,
    category: "Accessories",
    inStock: true,
  },
  {
    id: 4,
    name: "USB-C Hub",
    description: "Multi-port USB-C hub with HDMI, USB 3.0, and card reader",
    price: 39.99,
    category: "Electronics",
    inStock: false,
  },
  {
    id: 5,
    name: "Ergonomic Mouse",
    description: "Wireless ergonomic mouse designed for comfort and productivity",
    price: 49.99,
    category: "Accessories",
    inStock: true,
  },
  {
    id: 6,
    name: "Mechanical Keyboard",
    description: "RGB mechanical keyboard with customizable switches",
    price: 149.99,
    category: "Electronics",
    inStock: true,
  },
  {
    id: 7,
    name: "Portable Charger",
    description: "20000mAh portable power bank with fast charging",
    price: 34.99,
    category: "Electronics",
    inStock: true,
  },
  {
    id: 8,
    name: "Phone Stand",
    description: "Adjustable aluminum phone stand for desk and nightstand",
    price: 19.99,
    category: "Accessories",
    inStock: true,
  },
]

export default function ProductsPage() {
  return (
    <div className="container px-4 py-8 md:px-6 md:py-12">
      <div className="mb-8">
        <h1 className="text-3xl font-bold tracking-tight sm:text-4xl">
          All Products
        </h1>
        <p className="mt-2 text-muted-foreground">
          Browse our collection of quality products
        </p>
      </div>

      <div className="grid gap-6 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4">
        {products.map((product) => (
          <Card key={product.id} className="flex flex-col">
            <CardHeader>
              <div className="flex items-start justify-between">
                <Badge variant="secondary">{product.category}</Badge>
                {!product.inStock && (
                  <Badge variant="destructive">Out of Stock</Badge>
                )}
              </div>
              <CardTitle className="line-clamp-2">{product.name}</CardTitle>
            </CardHeader>
            <CardContent className="flex-1">
              <p className="line-clamp-3 text-sm text-muted-foreground">
                {product.description}
              </p>
              <p className="mt-4 text-2xl font-bold">
                ${product.price.toFixed(2)}
              </p>
            </CardContent>
            <CardFooter className="flex flex-col gap-2">
              <Link href={`/products/${product.id}`} className="w-full">
                <Button variant="outline" className="w-full">
                  View Details
                </Button>
              </Link>
              <Button
                className="w-full"
                disabled={!product.inStock}
              >
                {product.inStock ? "Add to Cart" : "Out of Stock"}
              </Button>
            </CardFooter>
          </Card>
        ))}
      </div>
    </div>
  )
}
