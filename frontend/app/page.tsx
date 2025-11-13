import Link from "next/link";
import { Button } from "@/components/ui/button";
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "@/components/ui/card";
import { ShoppingBag, Truck, Shield, Clock } from "lucide-react";

export default function HomePage() {
  const features = [
    {
      icon: ShoppingBag,
      title: "Wide Selection",
      description: "Browse thousands of products from top brands",
    },
    {
      icon: Truck,
      title: "Fast Shipping",
      description: "Free delivery on orders over $50",
    },
    {
      icon: Shield,
      title: "Secure Payment",
      description: "Your payment information is safe with us",
    },
    {
      icon: Clock,
      title: "24/7 Support",
      description: "We're here to help whenever you need us",
    },
  ];

  return (
    <div className="flex flex-col">
      {/* Hero Section */}
      <section className="bg-gradient-to-b from-primary/10 to-background py-20 md:py-32">
        <div className="container flex flex-col items-center text-center">
          <h1 className="mb-6 text-4xl font-bold tracking-tight md:text-6xl">
            Welcome to E-Shop
          </h1>
          <p className="mb-8 max-w-2xl text-lg text-muted-foreground md:text-xl">
            Discover amazing products at unbeatable prices. Shop the latest trends
            and enjoy fast, secure delivery.
          </p>
          <div className="flex flex-col gap-4 sm:flex-row">
            <Link href="/products">
              <Button size="lg" className="w-full sm:w-auto">
                Shop Now
              </Button>
            </Link>
            <Link href="/products">
              <Button size="lg" variant="outline" className="w-full sm:w-auto">
                Browse Products
              </Button>
            </Link>
          </div>
        </div>
      </section>

      {/* Features Section */}
      <section className="py-16 md:py-24">
        <div className="container">
          <h2 className="mb-12 text-center text-3xl font-bold md:text-4xl">
            Why Shop With Us
          </h2>
          <div className="grid gap-6 sm:grid-cols-2 lg:grid-cols-4">
            {features.map((feature) => {
              const Icon = feature.icon;
              return (
                <Card key={feature.title} className="border-2">
                  <CardHeader>
                    <div className="mb-4 flex h-12 w-12 items-center justify-center rounded-lg bg-primary/10">
                      <Icon className="h-6 w-6 text-primary" />
                    </div>
                    <CardTitle>{feature.title}</CardTitle>
                    <CardDescription>{feature.description}</CardDescription>
                  </CardHeader>
                </Card>
              );
            })}
          </div>
        </div>
      </section>

      {/* CTA Section */}
      <section className="bg-primary py-16 text-primary-foreground md:py-24">
        <div className="container text-center">
          <h2 className="mb-4 text-3xl font-bold md:text-4xl">
            Ready to Start Shopping?
          </h2>
          <p className="mb-8 text-lg opacity-90">
            Join thousands of satisfied customers today
          </p>
          <Link href="/register">
            <Button size="lg" variant="secondary">
              Create Account
            </Button>
          </Link>
        </div>
      </section>
    </div>
  );
}
