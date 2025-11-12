import Link from "next/link";
import { Button } from "@/components/ui/button";
import { ArrowRight, ShoppingBag, Shield, Truck } from "lucide-react";

export default function Home() {
  return (
    <div className="flex flex-col">
      {/* Hero Section */}
      <section className="w-full py-12 md:py-24 lg:py-32 xl:py-48 bg-gradient-to-b from-slate-50 to-white dark:from-slate-950 dark:to-background">
        <div className="container mx-auto px-4 sm:px-6 lg:px-8">
          <div className="flex flex-col items-center space-y-4 text-center">
            <div className="space-y-2">
              <h1 className="text-3xl font-bold tracking-tighter sm:text-4xl md:text-5xl lg:text-6xl">
                Welcome to E-Shop
              </h1>
              <p className="mx-auto max-w-[700px] text-gray-500 md:text-xl dark:text-gray-400">
                Discover amazing products at unbeatable prices. Shop the latest trends and enjoy fast, secure delivery.
              </p>
            </div>
            <div className="space-x-4">
              <Link href="/products">
                <Button size="lg" className="gap-2">
                  Shop Now
                  <ArrowRight className="h-4 w-4" aria-hidden="true" />
                </Button>
              </Link>
            </div>
          </div>
        </div>
      </section>

      {/* Features Section */}
      <section className="w-full py-12 md:py-24 lg:py-32 bg-white dark:bg-background">
        <div className="container mx-auto px-4 sm:px-6 lg:px-8">
          <div className="grid gap-8 md:grid-cols-3">
            <div className="flex flex-col items-center space-y-2 text-center p-6 rounded-lg border bg-card">
              <Truck className="h-12 w-12 text-primary" aria-hidden="true" />
              <h3 className="text-xl font-bold">Fast Delivery</h3>
              <p className="text-sm text-muted-foreground">
                Get your orders delivered quickly and reliably
              </p>
            </div>
            <div className="flex flex-col items-center space-y-2 text-center p-6 rounded-lg border bg-card">
              <Shield className="h-12 w-12 text-primary" aria-hidden="true" />
              <h3 className="text-xl font-bold">Secure Payment</h3>
              <p className="text-sm text-muted-foreground">
                Shop with confidence using our secure checkout
              </p>
            </div>
            <div className="flex flex-col items-center space-y-2 text-center p-6 rounded-lg border bg-card">
              <ShoppingBag className="h-12 w-12 text-primary" aria-hidden="true" />
              <h3 className="text-xl font-bold">Quality Products</h3>
              <p className="text-sm text-muted-foreground">
                Curated selection of high-quality items
              </p>
            </div>
          </div>
        </div>
      </section>
    </div>
  );
}
