import Link from "next/link";
import { Button } from "@/components/ui/button";
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "@/components/ui/card";
import { ShoppingBag, TrendingUp, Shield } from "lucide-react";

export default function HomePage() {
  const features = [
    {
      icon: ShoppingBag,
      title: "Wide Selection",
      description: "Browse through thousands of quality products",
    },
    {
      icon: TrendingUp,
      title: "Best Prices",
      description: "Competitive pricing and regular deals",
    },
    {
      icon: Shield,
      title: "Secure Shopping",
      description: "Safe and secure payment processing",
    },
  ];

  return (
    <div className="flex flex-col">
      {/* Hero Section */}
      <section className="bg-gradient-to-b from-primary/10 to-background py-20 md:py-32">
        <div className="container mx-auto px-4 text-center md:px-6">
          <h1 className="text-4xl font-bold tracking-tighter sm:text-5xl md:text-6xl lg:text-7xl">
            Welcome to Our Store
          </h1>
          <p className="mx-auto mt-4 max-w-[700px] text-lg text-muted-foreground md:text-xl">
            Discover amazing products at unbeatable prices. Shop now and enjoy fast shipping.
          </p>
          <div className="mt-8 flex flex-col gap-4 sm:flex-row sm:justify-center">
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
      <section className="py-20 md:py-32">
        <div className="container mx-auto px-4 md:px-6">
          <h2 className="mb-12 text-center text-3xl font-bold md:text-4xl">
            Why Shop With Us
          </h2>
          <div className="grid gap-6 sm:grid-cols-2 lg:grid-cols-3">
            {features.map((feature, index) => {
              const Icon = feature.icon;
              return (
                <Card key={index}>
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
      <section className="bg-primary py-20 text-primary-foreground">
        <div className="container mx-auto px-4 text-center md:px-6">
          <h2 className="text-3xl font-bold md:text-4xl">
            Ready to Start Shopping?
          </h2>
          <p className="mx-auto mt-4 max-w-[600px] text-lg opacity-90">
            Join thousands of satisfied customers and find your perfect products today.
          </p>
          <Link href="/register">
            <Button
              size="lg"
              variant="secondary"
              className="mt-8"
            >
              Create Account
            </Button>
          </Link>
        </div>
      </section>
    </div>
  );
}
