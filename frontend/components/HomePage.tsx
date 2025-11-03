import Link from "next/link";
import { Button } from "@/components/ui/button";
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "@/components/ui/card";
import { ShoppingBag, Truck, Shield, CreditCard } from "lucide-react";

export default function HomePage() {
  return (
    <div className="flex flex-col">
      {/* Hero Section */}
      <section className="bg-gradient-to-b from-primary/10 to-background py-20 px-4 sm:py-32">
        <div className="container mx-auto text-center">
          <h1 className="text-4xl font-bold tracking-tight sm:text-6xl mb-6">
            Welcome to E-Shop
          </h1>
          <p className="text-xl text-muted-foreground max-w-2xl mx-auto mb-8">
            Discover amazing products at unbeatable prices. Shop with confidence and enjoy fast, secure delivery.
          </p>
          <div className="flex gap-4 justify-center flex-wrap">
            <Link href="/products">
              <Button size="lg" className="text-lg px-8">
                Shop Now
              </Button>
            </Link>
            <Link href="/products">
              <Button size="lg" variant="outline" className="text-lg px-8">
                Browse Products
              </Button>
            </Link>
          </div>
        </div>
      </section>

      {/* Features Section */}
      <section className="py-16 px-4">
        <div className="container mx-auto">
          <h2 className="text-3xl font-bold text-center mb-12">Why Shop With Us</h2>
          <div className="grid grid-cols-1 gap-6 sm:grid-cols-2 lg:grid-cols-4">
            <Card>
              <CardHeader className="text-center">
                <div className="mx-auto mb-4 flex h-12 w-12 items-center justify-center rounded-full bg-primary/10">
                  <ShoppingBag className="h-6 w-6 text-primary" />
                </div>
                <CardTitle>Wide Selection</CardTitle>
              </CardHeader>
              <CardContent>
                <CardDescription className="text-center">
                  Thousands of products across multiple categories to choose from.
                </CardDescription>
              </CardContent>
            </Card>

            <Card>
              <CardHeader className="text-center">
                <div className="mx-auto mb-4 flex h-12 w-12 items-center justify-center rounded-full bg-primary/10">
                  <Truck className="h-6 w-6 text-primary" />
                </div>
                <CardTitle>Fast Delivery</CardTitle>
              </CardHeader>
              <CardContent>
                <CardDescription className="text-center">
                  Quick and reliable shipping to your doorstep in record time.
                </CardDescription>
              </CardContent>
            </Card>

            <Card>
              <CardHeader className="text-center">
                <div className="mx-auto mb-4 flex h-12 w-12 items-center justify-center rounded-full bg-primary/10">
                  <Shield className="h-6 w-6 text-primary" />
                </div>
                <CardTitle>Secure Shopping</CardTitle>
              </CardHeader>
              <CardContent>
                <CardDescription className="text-center">
                  Your data is protected with industry-leading security measures.
                </CardDescription>
              </CardContent>
            </Card>

            <Card>
              <CardHeader className="text-center">
                <div className="mx-auto mb-4 flex h-12 w-12 items-center justify-center rounded-full bg-primary/10">
                  <CreditCard className="h-6 w-6 text-primary" />
                </div>
                <CardTitle>Easy Payment</CardTitle>
              </CardHeader>
              <CardContent>
                <CardDescription className="text-center">
                  Multiple payment options for your convenience and security.
                </CardDescription>
              </CardContent>
            </Card>
          </div>
        </div>
      </section>

      {/* CTA Section */}
      <section className="bg-primary/5 py-16 px-4">
        <div className="container mx-auto text-center">
          <h2 className="text-3xl font-bold mb-4">Ready to Start Shopping?</h2>
          <p className="text-lg text-muted-foreground mb-8 max-w-xl mx-auto">
            Join thousands of satisfied customers and discover the best deals today.
          </p>
          <Link href="/products">
            <Button size="lg" className="text-lg px-8">
              View All Products
            </Button>
          </Link>
        </div>
      </section>
    </div>
  );
}
