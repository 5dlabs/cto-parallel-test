import Link from "next/link";
import { Button } from "@/components/ui/button";
import { Card, CardContent } from "@/components/ui/card";
import { Badge } from "@/components/ui/badge";
import { Header } from "@/components/Header";
import { Footer } from "@/components/Footer";
import { ArrowLeft, Star, Truck, Shield } from "lucide-react";

// Sample product data - In production, this would come from an API
const productsData: Record<string, any> = {
  "1": {
    id: 1,
    name: "Premium Wireless Headphones",
    description: "Experience superior sound quality with our Premium Wireless Headphones featuring active noise cancellation, 30-hour battery life, and premium comfort padding.",
    fullDescription: "These headphones deliver exceptional audio quality with deep bass, clear mids, and crisp highs. The active noise cancellation technology adapts to your environment, blocking out unwanted noise while preserving sound quality. With up to 30 hours of battery life, you can enjoy your music all day long. The memory foam ear cushions and adjustable headband ensure maximum comfort even during extended listening sessions.",
    price: 299.99,
    category: "Electronics",
    inStock: true,
    rating: 4.8,
    reviews: 342,
    features: [
      "Active Noise Cancellation",
      "30-hour battery life",
      "Premium comfort padding",
      "Bluetooth 5.0",
      "Quick charge support",
      "Foldable design"
    ],
    image: "https://images.unsplash.com/photo-1505740420928-5e560c06d30e?w=800&h=800&fit=crop",
  },
  "2": {
    id: 2,
    name: "Smart Watch Pro",
    description: "Track your fitness and stay connected with our advanced Smart Watch Pro",
    fullDescription: "Stay connected and monitor your health with this feature-packed smartwatch. Track your steps, heart rate, sleep patterns, and more. Receive notifications, answer calls, and control your music right from your wrist. With GPS tracking and water resistance, it's perfect for any activity.",
    price: 399.99,
    category: "Electronics",
    inStock: true,
    rating: 4.6,
    reviews: 256,
    features: [
      "Heart rate monitoring",
      "GPS tracking",
      "Water resistant (50m)",
      "5-day battery life",
      "Sleep tracking",
      "Notification support"
    ],
    image: "https://images.unsplash.com/photo-1523275335684-37898b6baf30?w=800&h=800&fit=crop",
  },
};

interface ProductDetailPageProps {
  params: Promise<{ id: string }>;
}

export default async function ProductDetailPage({ params }: ProductDetailPageProps) {
  const { id } = await params;
  const product = productsData[id] || productsData["1"];

  return (
    <>
      <Header />
      <main className="flex-1">
        <div className="container mx-auto px-4 py-8">
          {/* Breadcrumb */}
          <div className="mb-6">
            <Link href="/products">
              <Button variant="ghost" size="sm">
                <ArrowLeft className="mr-2 h-4 w-4" />
                Back to Products
              </Button>
            </Link>
          </div>

          {/* Product Detail */}
          <div className="grid gap-8 md:grid-cols-2">
            {/* Product Image */}
            <div className="aspect-square relative bg-slate-100 rounded-lg overflow-hidden">
              <img
                src={product.image}
                alt={product.name}
                className="object-cover w-full h-full"
              />
              <div className="absolute top-4 right-4">
                <Badge variant={product.inStock ? "default" : "secondary"}>
                  {product.inStock ? "In Stock" : "Out of Stock"}
                </Badge>
              </div>
            </div>

            {/* Product Info */}
            <div className="flex flex-col">
              <div className="mb-4">
                <h1 className="text-4xl font-bold tracking-tight mb-2">
                  {product.name}
                </h1>
                <div className="flex items-center gap-2 mb-2">
                  <div className="flex items-center">
                    {[...Array(5)].map((_, i) => (
                      <Star
                        key={i}
                        className={`h-5 w-5 ${
                          i < Math.floor(product.rating)
                            ? "fill-yellow-400 text-yellow-400"
                            : "text-gray-300"
                        }`}
                      />
                    ))}
                  </div>
                  <span className="text-sm text-muted-foreground">
                    {product.rating} ({product.reviews} reviews)
                  </span>
                </div>
                <p className="text-sm text-muted-foreground">{product.category}</p>
              </div>

              <p className="text-3xl font-bold mb-4">${product.price.toFixed(2)}</p>

              <p className="text-muted-foreground mb-6">{product.description}</p>

              <div className="space-y-4 mb-6">
                <Button size="lg" className="w-full" disabled={!product.inStock}>
                  Add to Cart
                </Button>
                <Button size="lg" variant="outline" className="w-full">
                  Add to Wishlist
                </Button>
              </div>

              <Card className="mb-6">
                <CardContent className="pt-6">
                  <h3 className="font-semibold mb-4">Key Features</h3>
                  <ul className="space-y-2">
                    {product.features.map((feature: string, index: number) => (
                      <li key={index} className="flex items-start">
                        <span className="text-primary mr-2">•</span>
                        {feature}
                      </li>
                    ))}
                  </ul>
                </CardContent>
              </Card>

              <div className="grid gap-4 sm:grid-cols-2">
                <Card>
                  <CardContent className="pt-6 flex items-start gap-3">
                    <Truck className="h-5 w-5 text-primary mt-0.5" />
                    <div>
                      <p className="font-semibold text-sm">Free Shipping</p>
                      <p className="text-xs text-muted-foreground">On all orders</p>
                    </div>
                  </CardContent>
                </Card>
                <Card>
                  <CardContent className="pt-6 flex items-start gap-3">
                    <Shield className="h-5 w-5 text-primary mt-0.5" />
                    <div>
                      <p className="font-semibold text-sm">2 Year Warranty</p>
                      <p className="text-xs text-muted-foreground">Manufacturer warranty</p>
                    </div>
                  </CardContent>
                </Card>
              </div>
            </div>
          </div>

          {/* Product Description */}
          <div className="mt-12">
            <h2 className="text-2xl font-bold mb-4">Product Description</h2>
            <p className="text-muted-foreground leading-relaxed">
              {product.fullDescription}
            </p>
          </div>
        </div>
      </main>
      <Footer />
    </>
  );
}
