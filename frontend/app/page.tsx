import Link from "next/link"
import { Button } from "@/components/ui/button"
import { Card, CardDescription, CardHeader, CardTitle } from "@/components/ui/card"
import { ShoppingBag, TrendingUp, Shield } from "lucide-react"

export default function HomePage() {
  return (
    <div className="flex flex-col">
      {/* Hero Section */}
      <section className="bg-gradient-to-b from-slate-50 to-white dark:from-slate-950 dark:to-slate-900 py-12 md:py-20 lg:py-28">
        <div className="container mx-auto px-4">
          <div className="flex flex-col items-center text-center space-y-6">
            <h1 className="text-4xl md:text-5xl lg:text-6xl font-bold tracking-tight">
              Welcome to ShopHub
            </h1>
            <p className="text-lg md:text-xl text-muted-foreground max-w-2xl">
              Discover amazing products at unbeatable prices. Shop from thousands of items
              across all categories.
            </p>
            <div className="flex flex-col sm:flex-row gap-4 mt-8">
              <Link href="/products">
                <Button size="lg" className="w-full sm:w-auto">
                  Browse Products
                </Button>
              </Link>
              <Link href="/register">
                <Button size="lg" variant="outline" className="w-full sm:w-auto">
                  Sign Up Now
                </Button>
              </Link>
            </div>
          </div>
        </div>
      </section>

      {/* Features Section */}
      <section className="py-12 md:py-20">
        <div className="container mx-auto px-4">
          <h2 className="text-3xl font-bold text-center mb-12">Why Choose ShopHub?</h2>
          <div className="grid grid-cols-1 md:grid-cols-3 gap-6">
            <Card>
              <CardHeader>
                <div className="flex justify-center mb-4">
                  <ShoppingBag className="h-12 w-12 text-primary" />
                </div>
                <CardTitle className="text-center">Wide Selection</CardTitle>
                <CardDescription className="text-center">
                  Browse thousands of products from top brands
                </CardDescription>
              </CardHeader>
            </Card>

            <Card>
              <CardHeader>
                <div className="flex justify-center mb-4">
                  <TrendingUp className="h-12 w-12 text-primary" />
                </div>
                <CardTitle className="text-center">Best Prices</CardTitle>
                <CardDescription className="text-center">
                  Get the best deals and exclusive discounts
                </CardDescription>
              </CardHeader>
            </Card>

            <Card>
              <CardHeader>
                <div className="flex justify-center mb-4">
                  <Shield className="h-12 w-12 text-primary" />
                </div>
                <CardTitle className="text-center">Secure Shopping</CardTitle>
                <CardDescription className="text-center">
                  Safe and secure payment methods
                </CardDescription>
              </CardHeader>
            </Card>
          </div>
        </div>
      </section>

      {/* CTA Section */}
      <section className="bg-slate-50 dark:bg-slate-900 py-12 md:py-20">
        <div className="container mx-auto px-4">
          <div className="flex flex-col items-center text-center space-y-6">
            <h2 className="text-3xl font-bold">Ready to Start Shopping?</h2>
            <p className="text-lg text-muted-foreground max-w-2xl">
              Join thousands of satisfied customers and find your perfect products today.
            </p>
            <Link href="/products">
              <Button size="lg">
                Explore Products
              </Button>
            </Link>
          </div>
        </div>
      </section>
    </div>
  )
}
