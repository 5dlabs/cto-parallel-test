import Link from "next/link"
import { notFound } from "next/navigation"
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card"
import { Button } from "@/components/ui/button"
import { Badge } from "@/components/ui/badge"
import { ArrowLeft, ShoppingCart } from "lucide-react"

// Mock product data - will be replaced with API calls later
const products = [
  {
    id: 1,
    name: "Premium Wireless Headphones",
    price: 299.99,
    description: "High-quality sound with active noise cancellation",
    category: "Electronics",
    inStock: true,
    features: [
      "Active Noise Cancellation",
      "40-hour battery life",
      "Bluetooth 5.0",
      "Premium comfort fit",
      "Built-in microphone",
    ],
  },
  {
    id: 2,
    name: "Smart Fitness Watch",
    price: 199.99,
    description: "Track your fitness goals with style",
    category: "Wearables",
    inStock: true,
    features: [
      "Heart rate monitor",
      "GPS tracking",
      "Water resistant",
      "7-day battery life",
      "Sleep tracking",
    ],
  },
  {
    id: 3,
    name: "Laptop Stand",
    price: 49.99,
    description: "Ergonomic aluminum laptop stand",
    category: "Accessories",
    inStock: true,
    features: [
      "Aluminum construction",
      "Adjustable height",
      "Non-slip pads",
      "Compatible with all laptops",
      "Portable design",
    ],
  },
  {
    id: 4,
    name: "Mechanical Keyboard",
    price: 149.99,
    description: "RGB backlit mechanical gaming keyboard",
    category: "Electronics",
    inStock: false,
    features: [
      "Cherry MX switches",
      "RGB backlighting",
      "Programmable keys",
      "USB-C connection",
      "Durable construction",
    ],
  },
  {
    id: 5,
    name: "Wireless Mouse",
    price: 79.99,
    description: "Precision wireless mouse with ergonomic design",
    category: "Electronics",
    inStock: true,
    features: [
      "Ergonomic design",
      "High precision sensor",
      "Long battery life",
      "Multiple DPI settings",
      "Wireless connectivity",
    ],
  },
  {
    id: 6,
    name: "USB-C Hub",
    price: 59.99,
    description: "7-in-1 USB-C hub with HDMI and ethernet",
    category: "Accessories",
    inStock: true,
    features: [
      "7 ports in one",
      "4K HDMI output",
      "Gigabit ethernet",
      "USB 3.0 ports",
      "Compact design",
    ],
  },
]

interface ProductDetailPageProps {
  params: Promise<{ id: string }>
}

export default async function ProductDetailPage({ params }: ProductDetailPageProps) {
  const { id } = await params
  const product = products.find((p) => p.id === parseInt(id))

  if (!product) {
    notFound()
  }

  return (
    <div className="container mx-auto px-4 py-8">
      <Link href="/products" className="inline-flex items-center text-sm text-muted-foreground hover:text-foreground mb-6">
        <ArrowLeft className="mr-2 h-4 w-4" />
        Back to Products
      </Link>

      <div className="grid grid-cols-1 lg:grid-cols-2 gap-8 mt-6">
        {/* Product Image Placeholder */}
        <div className="aspect-square bg-slate-100 dark:bg-slate-800 rounded-lg flex items-center justify-center">
          <div className="text-center text-muted-foreground">
            <ShoppingCart className="h-24 w-24 mx-auto mb-4 opacity-20" />
            <p>Product Image</p>
          </div>
        </div>

        {/* Product Details */}
        <div className="space-y-6">
          <div>
            <div className="flex items-center gap-2 mb-2">
              <Badge variant="secondary">{product.category}</Badge>
              {!product.inStock && (
                <Badge variant="destructive">Out of Stock</Badge>
              )}
            </div>
            <h1 className="text-3xl md:text-4xl font-bold mb-2">{product.name}</h1>
            <p className="text-lg text-muted-foreground">{product.description}</p>
          </div>

          <div>
            <p className="text-3xl font-bold">${product.price}</p>
          </div>

          <Card>
            <CardHeader>
              <CardTitle>Features</CardTitle>
            </CardHeader>
            <CardContent>
              <ul className="space-y-2">
                {product.features.map((feature, index) => (
                  <li key={index} className="flex items-center">
                    <span className="mr-2">•</span>
                    {feature}
                  </li>
                ))}
              </ul>
            </CardContent>
          </Card>

          <div className="flex flex-col sm:flex-row gap-4">
            <Button className="flex-1" size="lg" disabled={!product.inStock}>
              <ShoppingCart className="mr-2 h-5 w-5" />
              {product.inStock ? "Add to Cart" : "Out of Stock"}
            </Button>
            <Button variant="outline" size="lg" className="flex-1">
              Add to Wishlist
            </Button>
          </div>

          <Card>
            <CardHeader>
              <CardTitle>Product Information</CardTitle>
            </CardHeader>
            <CardContent>
              <dl className="space-y-2">
                <div className="flex justify-between">
                  <dt className="text-muted-foreground">Category:</dt>
                  <dd className="font-medium">{product.category}</dd>
                </div>
                <div className="flex justify-between">
                  <dt className="text-muted-foreground">Availability:</dt>
                  <dd className="font-medium">
                    {product.inStock ? "In Stock" : "Out of Stock"}
                  </dd>
                </div>
                <div className="flex justify-between">
                  <dt className="text-muted-foreground">Shipping:</dt>
                  <dd className="font-medium">Free shipping on orders over $50</dd>
                </div>
              </dl>
            </CardContent>
          </Card>
        </div>
      </div>
    </div>
  )
}
