import Link from 'next/link';
import { Button } from '@/components/ui/button';
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card';
import { ShoppingBag, Truck, Shield, Heart } from 'lucide-react';

export default function HomePage() {
  return (
    <div className="flex flex-col">
      {/* Hero Section */}
      <section className="bg-gradient-to-b from-primary/10 to-background py-12 md:py-20">
        <div className="container mx-auto px-4">
          <div className="flex flex-col items-center text-center space-y-6">
            <h1 className="text-4xl font-bold tracking-tighter sm:text-5xl md:text-6xl lg:text-7xl">
              Welcome to E-Shop
            </h1>
            <p className="max-w-[700px] text-lg text-muted-foreground sm:text-xl">
              Discover amazing products at unbeatable prices. Shop with confidence and enjoy fast shipping.
            </p>
            <div className="flex gap-4">
              <Link href="/products">
                <Button size="lg">
                  Shop Now
                </Button>
              </Link>
              <Link href="/products">
                <Button variant="outline" size="lg">
                  Browse Products
                </Button>
              </Link>
            </div>
          </div>
        </div>
      </section>

      {/* Features Section */}
      <section className="py-12 md:py-20">
        <div className="container mx-auto px-4">
          <h2 className="text-3xl font-bold text-center mb-12">Why Shop With Us</h2>
          <div className="grid grid-cols-1 gap-6 sm:grid-cols-2 lg:grid-cols-4">
            <Card>
              <CardHeader>
                <div className="flex justify-center mb-4">
                  <div className="rounded-full bg-primary/10 p-3">
                    <ShoppingBag className="h-6 w-6 text-primary" />
                  </div>
                </div>
                <CardTitle className="text-center">Quality Products</CardTitle>
              </CardHeader>
              <CardContent>
                <CardDescription className="text-center">
                  Carefully curated selection of high-quality items
                </CardDescription>
              </CardContent>
            </Card>

            <Card>
              <CardHeader>
                <div className="flex justify-center mb-4">
                  <div className="rounded-full bg-primary/10 p-3">
                    <Truck className="h-6 w-6 text-primary" />
                  </div>
                </div>
                <CardTitle className="text-center">Fast Shipping</CardTitle>
              </CardHeader>
              <CardContent>
                <CardDescription className="text-center">
                  Quick and reliable delivery to your doorstep
                </CardDescription>
              </CardContent>
            </Card>

            <Card>
              <CardHeader>
                <div className="flex justify-center mb-4">
                  <div className="rounded-full bg-primary/10 p-3">
                    <Shield className="h-6 w-6 text-primary" />
                  </div>
                </div>
                <CardTitle className="text-center">Secure Payment</CardTitle>
              </CardHeader>
              <CardContent>
                <CardDescription className="text-center">
                  Safe and encrypted payment processing
                </CardDescription>
              </CardContent>
            </Card>

            <Card>
              <CardHeader>
                <div className="flex justify-center mb-4">
                  <div className="rounded-full bg-primary/10 p-3">
                    <Heart className="h-6 w-6 text-primary" />
                  </div>
                </div>
                <CardTitle className="text-center">Customer Support</CardTitle>
              </CardHeader>
              <CardContent>
                <CardDescription className="text-center">
                  Dedicated team ready to assist you
                </CardDescription>
              </CardContent>
            </Card>
          </div>
        </div>
      </section>

      {/* CTA Section */}
      <section className="bg-primary/5 py-12 md:py-20">
        <div className="container mx-auto px-4">
          <div className="flex flex-col items-center text-center space-y-6">
            <h2 className="text-3xl font-bold">Ready to Start Shopping?</h2>
            <p className="max-w-[600px] text-muted-foreground">
              Join thousands of satisfied customers and find your perfect products today.
            </p>
            <Link href="/products">
              <Button size="lg">
                View All Products
              </Button>
            </Link>
          </div>
        </div>
      </section>
    </div>
  );
}
