import Link from "next/link"
import { Card, CardContent, CardDescription, CardFooter, CardHeader, CardTitle } from "@/components/ui/card"
import { Button } from "@/components/ui/button"
import { Badge } from "@/components/ui/badge"

// Mock product data - will be replaced with API calls later
const products = [
  {
    id: 1,
    name: "Premium Wireless Headphones",
    price: 299.99,
    description: "High-quality sound with active noise cancellation",
    category: "Electronics",
    inStock: true,
  },
  {
    id: 2,
    name: "Smart Fitness Watch",
    price: 199.99,
    description: "Track your fitness goals with style",
    category: "Wearables",
    inStock: true,
  },
  {
    id: 3,
    name: "Laptop Stand",
    price: 49.99,
    description: "Ergonomic aluminum laptop stand",
    category: "Accessories",
    inStock: true,
  },
  {
    id: 4,
    name: "Mechanical Keyboard",
    price: 149.99,
    description: "RGB backlit mechanical gaming keyboard",
    category: "Electronics",
    inStock: false,
  },
  {
    id: 5,
    name: "Wireless Mouse",
    price: 79.99,
    description: "Precision wireless mouse with ergonomic design",
    category: "Electronics",
    inStock: true,
  },
  {
    id: 6,
    name: "USB-C Hub",
    price: 59.99,
    description: "7-in-1 USB-C hub with HDMI and ethernet",
    category: "Accessories",
    inStock: true,
  },
]

export default function ProductListPage() {
  return (
    <div className="container mx-auto px-4 py-8">
      <div className="mb-8">
        <h1 className="text-3xl md:text-4xl font-bold mb-2">All Products</h1>
        <p className="text-muted-foreground">
          Discover our full range of products
        </p>
      </div>

      <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-6">
        {products.map((product) => (
          <Card key={product.id} className="flex flex-col">
            <CardHeader>
              <div className="flex items-start justify-between">
                <Badge variant="secondary">{product.category}</Badge>
                {!product.inStock && (
                  <Badge variant="destructive">Out of Stock</Badge>
                )}
              </div>
              <CardTitle className="mt-4">{product.name}</CardTitle>
              <CardDescription>{product.description}</CardDescription>
            </CardHeader>
            <CardContent className="flex-1">
              <p className="text-2xl font-bold">${product.price}</p>
            </CardContent>
            <CardFooter className="flex flex-col gap-2">
              <Link href={`/products/${product.id}`} className="w-full">
                <Button variant="outline" className="w-full">
                  View Details
                </Button>
              </Link>
              <Button className="w-full" disabled={!product.inStock}>
                {product.inStock ? "Add to Cart" : "Out of Stock"}
              </Button>
            </CardFooter>
          </Card>
        ))}
      </div>
    </div>
  )
}
