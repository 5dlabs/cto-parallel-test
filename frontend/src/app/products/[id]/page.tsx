import Link from "next/link"
import { Button } from "@/components/ui/button"
import { Badge } from "@/components/ui/badge"
import { Card, CardContent } from "@/components/ui/card"
import { ArrowLeft, ShoppingCart, Star } from "lucide-react"

// Mock product data - will be replaced with API integration
interface Product {
  id: number
  name: string
  description: string
  fullDescription: string
  price: number
  category: string
  inStock: boolean
  rating: number
  reviews: number
  features: string[]
}

const getProduct = (id: string): Product => {
  const products: Record<string, Product> = {
    "1": {
      id: 1,
      name: "Wireless Headphones",
      description: "Premium noise-cancelling wireless headphones with long battery life",
      fullDescription: "Experience crystal-clear audio with our premium wireless headphones. Featuring active noise cancellation, 30-hour battery life, and comfortable over-ear design. Perfect for music lovers, travelers, and professionals who demand the best audio quality.",
      price: 129.99,
      category: "Electronics",
      inStock: true,
      rating: 4.5,
      reviews: 234,
      features: [
        "Active Noise Cancellation",
        "30-hour battery life",
        "Bluetooth 5.0",
        "Comfortable over-ear design",
        "Built-in microphone",
      ],
    },
  }
  
  return products[id] || products["1"]
}

export default async function ProductDetailPage({
  params,
}: {
  params: Promise<{ id: string }>
}) {
  const { id } = await params
  const product = getProduct(id)

  return (
    <div className="container px-4 py-8 md:px-6 md:py-12">
      {/* Back Button */}
      <Link href="/products" className="mb-6 inline-flex items-center text-sm text-muted-foreground hover:text-foreground">
        <ArrowLeft className="mr-2 h-4 w-4" />
        Back to Products
      </Link>

      <div className="grid gap-8 lg:grid-cols-2">
        {/* Product Image Placeholder */}
        <Card>
          <CardContent className="flex aspect-square items-center justify-center bg-muted p-6">
            <div className="text-center">
              <ShoppingCart className="mx-auto h-24 w-24 text-muted-foreground" />
              <p className="mt-4 text-sm text-muted-foreground">Product Image</p>
            </div>
          </CardContent>
        </Card>

        {/* Product Details */}
        <div className="flex flex-col space-y-6">
          <div>
            <Badge variant="secondary" className="mb-3">
              {product.category}
            </Badge>
            <h1 className="text-3xl font-bold tracking-tight sm:text-4xl">
              {product.name}
            </h1>
            <div className="mt-3 flex items-center space-x-2">
              <div className="flex items-center">
                {Array.from({ length: 5 }).map((_, i) => (
                  <Star
                    key={i}
                    className={`h-5 w-5 ${
                      i < Math.floor(product.rating)
                        ? "fill-primary text-primary"
                        : "text-muted-foreground"
                    }`}
                  />
                ))}
              </div>
              <span className="text-sm text-muted-foreground">
                {product.rating} ({product.reviews} reviews)
              </span>
            </div>
          </div>

          <p className="text-lg text-muted-foreground">
            {product.fullDescription}
          </p>

          {product.features && (
            <div>
              <h3 className="mb-3 text-lg font-semibold">Key Features</h3>
              <ul className="space-y-2">
                {product.features.map((feature: string, index: number) => (
                  <li key={index} className="flex items-start">
                    <span className="mr-2 mt-1 h-1.5 w-1.5 flex-shrink-0 rounded-full bg-primary" />
                    <span className="text-sm text-muted-foreground">{feature}</span>
                  </li>
                ))}
              </ul>
            </div>
          )}

          <div className="space-y-4">
            <div className="flex items-baseline space-x-2">
              <span className="text-4xl font-bold">${product.price.toFixed(2)}</span>
              {product.inStock ? (
                <Badge variant="secondary">In Stock</Badge>
              ) : (
                <Badge variant="destructive">Out of Stock</Badge>
              )}
            </div>

            <div className="flex flex-col gap-3 sm:flex-row">
              <Button
                size="lg"
                className="flex-1"
                disabled={!product.inStock}
              >
                <ShoppingCart className="mr-2 h-5 w-5" />
                Add to Cart
              </Button>
              <Button size="lg" variant="outline">
                Buy Now
              </Button>
            </div>
          </div>

          <div className="rounded-lg border bg-muted/50 p-4">
            <h3 className="mb-2 font-semibold">Shipping Information</h3>
            <p className="text-sm text-muted-foreground">
              Free shipping on orders over $50. Expected delivery in 3-5 business days.
            </p>
          </div>
        </div>
      </div>
    </div>
  )
}
