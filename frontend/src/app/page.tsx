import Link from "next/link";
import { Button } from "@/components/ui/button";
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "@/components/ui/card";
import { ShoppingBag, Truck, Shield, CreditCard } from "lucide-react";

export default function HomePage() {
  const features = [
    {
      icon: ShoppingBag,
      title: "Wide Selection",
      description: "Thousands of products across multiple categories",
    },
    {
      icon: Truck,
      title: "Fast Delivery",
      description: "Quick and reliable shipping to your doorstep",
    },
    {
      icon: Shield,
      title: "Secure Shopping",
      description: "Your data is protected with enterprise-level security",
    },
    {
      icon: CreditCard,
      title: "Easy Payment",
      description: "Multiple payment options for your convenience",
    },
  ];

  return (
    <div className="flex flex-col">
      {/* Hero Section */}
      <section className="bg-gradient-to-b from-primary/10 to-background py-20 md:py-32">
        <div className="container mx-auto px-4 sm:px-6 lg:px-8">
          <div className="text-center max-w-3xl mx-auto">
            <h1 className="text-4xl md:text-6xl font-bold tracking-tight mb-6">
              Welcome to E-Shop
            </h1>
            <p className="text-lg md:text-xl text-muted-foreground mb-8">
              Discover amazing products at unbeatable prices. Shop now and enjoy
              a seamless shopping experience.
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
          </div>
        </div>
      </section>

      {/* Features Section */}
      <section className="py-16 md:py-24">
        <div className="container mx-auto px-4 sm:px-6 lg:px-8">
          <h2 className="text-3xl font-bold text-center mb-12">
            Why Choose Us
          </h2>
          <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6">
            {features.map((feature, index) => {
              const Icon = feature.icon;
              return (
                <Card key={index} className="text-center">
                  <CardHeader>
                    <div className="mx-auto mb-4 w-12 h-12 rounded-full bg-primary/10 flex items-center justify-center">
                      <Icon className="h-6 w-6 text-primary" />
                    </div>
                    <CardTitle className="text-xl">{feature.title}</CardTitle>
                  </CardHeader>
                  <CardContent>
                    <CardDescription>{feature.description}</CardDescription>
                  </CardContent>
                </Card>
              );
            })}
          </div>
        </div>
      </section>

      {/* CTA Section */}
      <section className="bg-primary text-primary-foreground py-16 md:py-24">
        <div className="container mx-auto px-4 sm:px-6 lg:px-8 text-center">
          <h2 className="text-3xl md:text-4xl font-bold mb-4">
            Ready to Start Shopping?
          </h2>
          <p className="text-lg mb-8 opacity-90">
            Join thousands of satisfied customers today
          </p>
          <Link href="/products">
            <Button size="lg" variant="secondary">
              Explore Products
            </Button>
          </Link>
        </div>
      </section>
    </div>
  );
}
