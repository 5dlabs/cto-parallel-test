import Link from 'next/link';
import { Button } from '@/components/ui/button';
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card';
import { ShoppingBag, TrendingUp, Shield, Truck } from 'lucide-react';

export default function Home() {
  const features = [
    {
      icon: ShoppingBag,
      title: 'Wide Selection',
      description: 'Browse thousands of quality products across multiple categories',
    },
    {
      icon: TrendingUp,
      title: 'Best Prices',
      description: 'Competitive pricing with frequent deals and discounts',
    },
    {
      icon: Shield,
      title: 'Secure Shopping',
      description: 'Safe and secure payment processing for peace of mind',
    },
    {
      icon: Truck,
      title: 'Fast Delivery',
      description: 'Quick and reliable shipping to your doorstep',
    },
  ];

  return (
    <div className="flex flex-col">
      {/* Hero Section */}
      <section className="bg-gradient-to-b from-primary/10 to-background">
        <div className="container px-4 py-16 md:py-24 md:px-8">
          <div className="flex flex-col items-center text-center space-y-8">
            <h1 className="text-4xl font-bold tracking-tight sm:text-5xl md:text-6xl lg:text-7xl">
              Welcome to <span className="text-primary">ShopHub</span>
            </h1>
            <p className="max-w-2xl text-lg text-muted-foreground sm:text-xl">
              Discover amazing products at unbeatable prices. Your one-stop destination
              for quality shopping.
            </p>
            <div className="flex flex-col sm:flex-row gap-4">
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
        <div className="container px-4 md:px-8">
          <div className="text-center space-y-4 mb-12">
            <h2 className="text-3xl font-bold tracking-tight sm:text-4xl">
              Why Shop With Us
            </h2>
            <p className="text-muted-foreground max-w-2xl mx-auto">
              We provide the best shopping experience with quality products and excellent service
            </p>
          </div>

          <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6">
            {features.map((feature, index) => {
              const Icon = feature.icon;
              return (
                <Card key={index} className="border-2 hover:border-primary transition-colors">
                  <CardHeader>
                    <div className="w-12 h-12 rounded-full bg-primary/10 flex items-center justify-center mb-4">
                      <Icon className="h-6 w-6 text-primary" />
                    </div>
                    <CardTitle className="text-xl">{feature.title}</CardTitle>
                  </CardHeader>
                  <CardContent>
                    <CardDescription className="text-base">
                      {feature.description}
                    </CardDescription>
                  </CardContent>
                </Card>
              );
            })}
          </div>
        </div>
      </section>

      {/* CTA Section */}
      <section className="bg-primary/5 py-16 md:py-24">
        <div className="container px-4 md:px-8">
          <div className="flex flex-col items-center text-center space-y-8">
            <h2 className="text-3xl font-bold tracking-tight sm:text-4xl max-w-2xl">
              Ready to Start Shopping?
            </h2>
            <p className="text-lg text-muted-foreground max-w-xl">
              Join thousands of satisfied customers and discover your next favorite product today.
            </p>
            <Link href="/products">
              <Button size="lg">
                Browse Products
              </Button>
            </Link>
          </div>
        </div>
      </section>
    </div>
  );
}
