import Link from "next/link"
import { Button } from "@/components/ui/button"
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "@/components/ui/card"
import { ShoppingBag, Truck, Shield, CreditCard } from "lucide-react"

export default function HomePage() {
  const features = [
    {
      icon: ShoppingBag,
      title: "Wide Selection",
      description: "Browse thousands of quality products across multiple categories",
    },
    {
      icon: Truck,
      title: "Fast Delivery",
      description: "Get your orders delivered quickly and reliably to your doorstep",
    },
    {
      icon: Shield,
      title: "Secure Shopping",
      description: "Shop with confidence with our secure payment and data protection",
    },
    {
      icon: CreditCard,
      title: "Easy Payments",
      description: "Multiple payment options for your convenience and flexibility",
    },
  ]

  return (
    <div className="flex flex-col">
      {/* Hero Section */}
      <section className="w-full bg-gradient-to-b from-primary/10 to-background py-12 md:py-24 lg:py-32">
        <div className="container px-4 md:px-6">
          <div className="flex flex-col items-center space-y-4 text-center">
            <div className="space-y-2">
              <h1 className="text-3xl font-bold tracking-tighter sm:text-4xl md:text-5xl lg:text-6xl">
                Welcome to E-Commerce
              </h1>
              <p className="mx-auto max-w-[700px] text-muted-foreground md:text-xl">
                Discover amazing products at unbeatable prices. Shop with confidence and enjoy fast, secure delivery.
              </p>
            </div>
            <div className="flex flex-col gap-2 min-[400px]:flex-row">
              <Link href="/products">
                <Button size="lg" className="w-full min-[400px]:w-auto">
                  Shop Now
                </Button>
              </Link>
              <Link href="/register">
                <Button size="lg" variant="outline" className="w-full min-[400px]:w-auto">
                  Sign Up
                </Button>
              </Link>
            </div>
          </div>
        </div>
      </section>

      {/* Features Section */}
      <section className="w-full py-12 md:py-24 lg:py-32">
        <div className="container px-4 md:px-6">
          <div className="mb-12 text-center">
            <h2 className="text-3xl font-bold tracking-tighter sm:text-4xl md:text-5xl">
              Why Shop With Us?
            </h2>
            <p className="mt-4 text-muted-foreground md:text-xl">
              Experience the best online shopping with our premium features
            </p>
          </div>
          <div className="grid gap-6 sm:grid-cols-2 lg:grid-cols-4">
            {features.map((feature) => (
              <Card key={feature.title} className="border-2">
                <CardHeader>
                  <feature.icon className="mb-2 h-10 w-10 text-primary" />
                  <CardTitle className="text-xl">{feature.title}</CardTitle>
                </CardHeader>
                <CardContent>
                  <CardDescription className="text-base">
                    {feature.description}
                  </CardDescription>
                </CardContent>
              </Card>
            ))}
          </div>
        </div>
      </section>

      {/* CTA Section */}
      <section className="w-full border-t bg-muted/50 py-12 md:py-24 lg:py-32">
        <div className="container px-4 md:px-6">
          <div className="flex flex-col items-center space-y-4 text-center">
            <div className="space-y-2">
              <h2 className="text-3xl font-bold tracking-tighter sm:text-4xl md:text-5xl">
                Ready to Start Shopping?
              </h2>
              <p className="mx-auto max-w-[600px] text-muted-foreground md:text-xl">
                Join thousands of satisfied customers and discover your next favorite product today.
              </p>
            </div>
            <Link href="/products">
              <Button size="lg">Browse Products</Button>
            </Link>
          </div>
        </div>
      </section>
    </div>
  )
}
