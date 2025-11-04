import React from 'react'
import { Link } from 'react-router-dom'
import { Button } from './ui/button'
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from './ui/card'
import { ShoppingBag, Truck, Shield, CreditCard } from 'lucide-react'

function HomePage() {
  return (
    <div className="space-y-12">
      {/* Hero Section */}
      <section className="text-center space-y-6 py-12">
        <h1 className="text-4xl md:text-6xl font-bold tracking-tight">
          Welcome to E-Commerce Shop
        </h1>
        <p className="text-xl text-muted-foreground max-w-2xl mx-auto">
          Discover amazing products at great prices. Shop with confidence and enjoy fast, reliable delivery.
        </p>
        <div className="flex gap-4 justify-center">
          <Link to="/products">
            <Button size="lg" className="text-lg px-8">
              Shop Now
            </Button>
          </Link>
          <Link to="/register">
            <Button size="lg" variant="outline" className="text-lg px-8">
              Sign Up
            </Button>
          </Link>
        </div>
      </section>

      {/* Features Section */}
      <section className="grid md:grid-cols-2 lg:grid-cols-4 gap-6">
        <Card>
          <CardHeader className="text-center">
            <ShoppingBag className="w-12 h-12 mx-auto mb-2 text-primary" />
            <CardTitle>Wide Selection</CardTitle>
          </CardHeader>
          <CardContent>
            <CardDescription className="text-center">
              Browse thousands of quality products across multiple categories
            </CardDescription>
          </CardContent>
        </Card>

        <Card>
          <CardHeader className="text-center">
            <Truck className="w-12 h-12 mx-auto mb-2 text-primary" />
            <CardTitle>Fast Delivery</CardTitle>
          </CardHeader>
          <CardContent>
            <CardDescription className="text-center">
              Get your orders delivered quickly with our reliable shipping partners
            </CardDescription>
          </CardContent>
        </Card>

        <Card>
          <CardHeader className="text-center">
            <Shield className="w-12 h-12 mx-auto mb-2 text-primary" />
            <CardTitle>Secure Shopping</CardTitle>
          </CardHeader>
          <CardContent>
            <CardDescription className="text-center">
              Shop with confidence knowing your data is protected
            </CardDescription>
          </CardContent>
        </Card>

        <Card>
          <CardHeader className="text-center">
            <CreditCard className="w-12 h-12 mx-auto mb-2 text-primary" />
            <CardTitle>Easy Payment</CardTitle>
          </CardHeader>
          <CardContent>
            <CardDescription className="text-center">
              Multiple payment options for your convenience
            </CardDescription>
          </CardContent>
        </Card>
      </section>

      {/* Call to Action */}
      <section className="bg-muted rounded-lg p-12 text-center space-y-4">
        <h2 className="text-3xl font-bold">Ready to Start Shopping?</h2>
        <p className="text-muted-foreground text-lg max-w-xl mx-auto">
          Join thousands of satisfied customers and discover your next favorite product today.
        </p>
        <Link to="/products">
          <Button size="lg" className="mt-4">
            Browse Products
          </Button>
        </Link>
      </section>
    </div>
  )
}

export default HomePage
