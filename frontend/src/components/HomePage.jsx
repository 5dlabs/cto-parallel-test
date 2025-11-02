import React from 'react'
import { Link } from 'react-router-dom'
import { Button } from './ui/button'
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from './ui/card'
import { ShoppingBag, Truck, Shield, CreditCard } from 'lucide-react'

function HomePage() {
  return (
    <div className="container mx-auto px-4 py-8">
      {/* Hero Section */}
      <section className="text-center py-12 md:py-20">
        <h1 className="text-4xl md:text-6xl font-bold mb-6">
          Welcome to ShopHub
        </h1>
        <p className="text-xl md:text-2xl text-muted-foreground mb-8 max-w-2xl mx-auto">
          Discover amazing products at unbeatable prices. Quality you can trust, delivered to your door.
        </p>
        <div className="flex flex-col sm:flex-row gap-4 justify-center">
          <Link to="/products">
            <Button size="lg" className="w-full sm:w-auto">
              Shop Now
            </Button>
          </Link>
          <Link to="/register">
            <Button size="lg" variant="outline" className="w-full sm:w-auto">
              Create Account
            </Button>
          </Link>
        </div>
      </section>

      {/* Features Section */}
      <section className="py-12">
        <h2 className="text-3xl font-bold text-center mb-12">Why Choose ShopHub?</h2>
        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6">
          <Card>
            <CardHeader className="text-center">
              <ShoppingBag className="h-12 w-12 mx-auto mb-4 text-primary" />
              <CardTitle>Wide Selection</CardTitle>
            </CardHeader>
            <CardContent>
              <CardDescription className="text-center">
                Browse thousands of products across multiple categories
              </CardDescription>
            </CardContent>
          </Card>

          <Card>
            <CardHeader className="text-center">
              <Truck className="h-12 w-12 mx-auto mb-4 text-primary" />
              <CardTitle>Fast Shipping</CardTitle>
            </CardHeader>
            <CardContent>
              <CardDescription className="text-center">
                Free delivery on orders over $50. Express shipping available
              </CardDescription>
            </CardContent>
          </Card>

          <Card>
            <CardHeader className="text-center">
              <Shield className="h-12 w-12 mx-auto mb-4 text-primary" />
              <CardTitle>Secure Shopping</CardTitle>
            </CardHeader>
            <CardContent>
              <CardDescription className="text-center">
                Your data is protected with industry-leading security
              </CardDescription>
            </CardContent>
          </Card>

          <Card>
            <CardHeader className="text-center">
              <CreditCard className="h-12 w-12 mx-auto mb-4 text-primary" />
              <CardTitle>Easy Returns</CardTitle>
            </CardHeader>
            <CardContent>
              <CardDescription className="text-center">
                30-day money-back guarantee on all purchases
              </CardDescription>
            </CardContent>
          </Card>
        </div>
      </section>

      {/* CTA Section */}
      <section className="py-12 text-center">
        <Card className="bg-primary text-primary-foreground">
          <CardHeader>
            <CardTitle className="text-3xl">Ready to Start Shopping?</CardTitle>
            <CardDescription className="text-primary-foreground/80 text-lg">
              Join thousands of happy customers and find your perfect products today
            </CardDescription>
          </CardHeader>
          <CardContent>
            <Link to="/products">
              <Button size="lg" variant="secondary">
                Browse Products
              </Button>
            </Link>
          </CardContent>
        </Card>
      </section>
    </div>
  )
}

export default HomePage
