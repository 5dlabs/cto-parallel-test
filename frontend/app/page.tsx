import Link from "next/link";
import { Button } from "@/components/ui/button";
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "@/components/ui/card";
import { ShoppingBag, Truck, Shield, CreditCard } from "lucide-react";

export default function Home() {
  const features = [
    {
      icon: ShoppingBag,
      title: "Wide Selection",
      description: "Browse thousands of products across multiple categories",
    },
    {
      icon: Truck,
      title: "Fast Delivery",
      description: "Get your orders delivered quickly to your doorstep",
    },
    {
      icon: Shield,
      title: "Secure Shopping",
      description: "Shop with confidence with our secure payment system",
    },
    {
      icon: CreditCard,
      title: "Easy Payment",
      description: "Multiple payment options for your convenience",
    },
  ];

  return (
    <div className="container mx-auto px-4 py-12">
      {/* Hero Section */}
      <section className="text-center py-16 md:py-24">
        <h1 className="text-4xl md:text-6xl font-bold tracking-tight mb-6">
          Welcome to E-Shop
        </h1>
        <p className="text-xl md:text-2xl text-muted-foreground mb-8 max-w-2xl mx-auto">
          Discover amazing products at great prices. Shop the latest trends and get fast delivery.
        </p>
        <div className="flex flex-col sm:flex-row gap-4 justify-center">
          <Link href="/products">
            <Button size="lg" className="w-full sm:w-auto">
              Browse Products
            </Button>
          </Link>
          <Link href="/register">
            <Button size="lg" variant="outline" className="w-full sm:w-auto">
              Sign Up
            </Button>
          </Link>
        </div>
      </section>

      {/* Features Section */}
      <section className="py-12">
        <h2 className="text-3xl font-bold text-center mb-12">
          Why Shop With Us?
        </h2>
        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6">
          {features.map((feature) => {
            const Icon = feature.icon;
            return (
              <Card key={feature.title} className="text-center">
                <CardHeader>
                  <div className="mx-auto mb-4 w-12 h-12 rounded-full bg-primary/10 flex items-center justify-center">
                    <Icon className="h-6 w-6 text-primary" />
                  </div>
                  <CardTitle>{feature.title}</CardTitle>
                </CardHeader>
                <CardContent>
                  <CardDescription>{feature.description}</CardDescription>
                </CardContent>
              </Card>
            );
          })}
        </div>
      </section>

      {/* CTA Section */}
      <section className="py-16 text-center">
        <Card className="bg-primary/5">
          <CardHeader>
            <CardTitle className="text-3xl">Ready to Start Shopping?</CardTitle>
            <CardDescription className="text-lg">
              Join thousands of satisfied customers today
            </CardDescription>
          </CardHeader>
          <CardContent>
            <Link href="/products">
              <Button size="lg">Explore Products</Button>
            </Link>
          </CardContent>
        </Card>
      </section>
    </div>
  );
}
