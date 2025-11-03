import Link from "next/link";
import { Button } from "@/components/ui/button";
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "@/components/ui/card";
import { Header } from "@/components/Header";
import { Footer } from "@/components/Footer";
import { ShoppingBag, Truck, Shield, CreditCard } from "lucide-react";

export default function HomePage() {
  return (
    <>
      <Header />
      <main className="flex-1">
        {/* Hero Section */}
        <section className="w-full py-12 md:py-24 lg:py-32 xl:py-48 bg-gradient-to-b from-slate-50 to-white">
          <div className="container mx-auto px-4 md:px-6">
            <div className="flex flex-col items-center space-y-4 text-center">
              <div className="space-y-2">
                <h1 className="text-3xl font-bold tracking-tighter sm:text-4xl md:text-5xl lg:text-6xl/none">
                  Welcome to E-Shop
                </h1>
                <p className="mx-auto max-w-[700px] text-gray-500 md:text-xl">
                  Discover amazing products at unbeatable prices. Shop the latest trends and enjoy fast, free shipping on all orders.
                </p>
              </div>
              <div className="space-x-4">
                <Link href="/products">
                  <Button size="lg" className="h-11">
                    Shop Now
                  </Button>
                </Link>
                <Button variant="outline" size="lg" className="h-11">
                  Learn More
                </Button>
              </div>
            </div>
          </div>
        </section>

        {/* Features Section */}
        <section className="w-full py-12 md:py-24 lg:py-32 bg-white">
          <div className="container mx-auto px-4 md:px-6">
            <h2 className="text-3xl font-bold tracking-tighter text-center mb-12">
              Why Shop With Us
            </h2>
            <div className="grid gap-6 sm:grid-cols-2 lg:grid-cols-4">
              <Card>
                <CardHeader>
                  <ShoppingBag className="h-10 w-10 mb-2 text-primary" />
                  <CardTitle>Quality Products</CardTitle>
                  <CardDescription>
                    Carefully curated selection of premium items
                  </CardDescription>
                </CardHeader>
              </Card>
              <Card>
                <CardHeader>
                  <Truck className="h-10 w-10 mb-2 text-primary" />
                  <CardTitle>Fast Shipping</CardTitle>
                  <CardDescription>
                    Free delivery on all orders, no minimum
                  </CardDescription>
                </CardHeader>
              </Card>
              <Card>
                <CardHeader>
                  <Shield className="h-10 w-10 mb-2 text-primary" />
                  <CardTitle>Secure Shopping</CardTitle>
                  <CardDescription>
                    Your data is protected with industry-leading security
                  </CardDescription>
                </CardHeader>
              </Card>
              <Card>
                <CardHeader>
                  <CreditCard className="h-10 w-10 mb-2 text-primary" />
                  <CardTitle>Easy Returns</CardTitle>
                  <CardDescription>
                    30-day money-back guarantee on all purchases
                  </CardDescription>
                </CardHeader>
              </Card>
            </div>
          </div>
        </section>

        {/* CTA Section */}
        <section className="w-full py-12 md:py-24 lg:py-32 bg-slate-900 text-white">
          <div className="container mx-auto px-4 md:px-6">
            <div className="flex flex-col items-center space-y-4 text-center">
              <div className="space-y-2">
                <h2 className="text-3xl font-bold tracking-tighter sm:text-4xl md:text-5xl">
                  Ready to Start Shopping?
                </h2>
                <p className="mx-auto max-w-[600px] text-gray-300 md:text-xl">
                  Join thousands of satisfied customers and experience the best online shopping today.
                </p>
              </div>
              <Link href="/products">
                <Button size="lg" variant="secondary" className="h-11">
                  Browse Products
                </Button>
              </Link>
            </div>
          </div>
        </section>
      </main>
      <Footer />
    </>
  );
}
