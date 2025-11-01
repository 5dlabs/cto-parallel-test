import Link from "next/link";
import { Button } from "@/components/ui/button";
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "@/components/ui/card";
import { ShoppingBag, Truck, Shield, CreditCard } from "lucide-react";

export default function Home() {
  const features = [
    {
      icon: ShoppingBag,
      title: "Wide Selection",
      description: "Browse thousands of quality products across multiple categories",
    },
    {
      icon: Truck,
      title: "Fast Delivery",
      description: "Free shipping on orders over $50 with tracking included",
    },
    {
      icon: Shield,
      title: "Secure Shopping",
      description: "Your transactions are protected with industry-leading security",
    },
    {
      icon: CreditCard,
      title: "Easy Payments",
      description: "Multiple payment options for your convenience",
    },
  ];

  return (
    <div className="flex flex-col">
      {/* Hero Section */}
      <section className="bg-gradient-to-b from-zinc-50 to-white py-16 md:py-24">
        <div className="container mx-auto px-4">
          <div className="max-w-3xl mx-auto text-center">
            <h1 className="text-4xl md:text-5xl lg:text-6xl font-bold tracking-tight mb-6">
              Welcome to E-Commerce
            </h1>
            <p className="text-lg md:text-xl text-zinc-600 mb-8">
              Discover amazing products at unbeatable prices. Shop with confidence and enjoy fast, free delivery.
            </p>
            <div className="flex flex-col sm:flex-row gap-4 justify-center">
              <Link href="/products">
                <Button size="lg" className="w-full sm:w-auto">
                  Shop Now
                </Button>
              </Link>
              <Link href="/register">
                <Button size="lg" variant="outline" className="w-full sm:w-auto">
                  Create Account
                </Button>
              </Link>
            </div>
          </div>
        </div>
      </section>

      {/* Features Section */}
      <section className="py-16 md:py-24">
        <div className="container mx-auto px-4">
          <h2 className="text-3xl md:text-4xl font-bold text-center mb-12">
            Why Shop With Us
          </h2>
          <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-6">
            {features.map((feature, index) => (
              <Card key={index} className="text-center">
                <CardHeader>
                  <div className="mx-auto mb-4 w-12 h-12 rounded-full bg-primary/10 flex items-center justify-center">
                    <feature.icon className="h-6 w-6 text-primary" />
                  </div>
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
      <section className="bg-zinc-900 text-white py-16 md:py-24">
        <div className="container mx-auto px-4 text-center">
          <h2 className="text-3xl md:text-4xl font-bold mb-4">
            Ready to Start Shopping?
          </h2>
          <p className="text-lg text-zinc-300 mb-8 max-w-2xl mx-auto">
            Join thousands of satisfied customers and discover your next favorite product today.
          </p>
          <Link href="/products">
            <Button size="lg" variant="secondary">
              Browse Products
            </Button>
          </Link>
        </div>
      </section>
    </div>
  );
}
