import React from 'react'
import { Link } from 'react-router-dom'
import { Button } from './ui/button'
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from './ui/card'
import { ShoppingBag, Package, TruckIcon } from 'lucide-react'

const HomePage = () => {
  return (
    <div className="container mx-auto px-4 py-8">
      {/* Hero Section */}
      <section className="text-center py-12 md:py-20">
        <h1 className="text-4xl md:text-6xl font-bold tracking-tight mb-6">
          Welcome to E-Shop
        </h1>
        <p className="text-xl text-muted-foreground mb-8 max-w-2xl mx-auto">
          Discover amazing products at unbeatable prices. Your one-stop shop for everything you need.
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
        <h2 className="text-3xl font-bold text-center mb-8">Why Choose Us</h2>
        <div className="grid grid-cols-1 md:grid-cols-3 gap-6">
          <Card>
            <CardHeader>
              <div className="flex justify-center mb-4">
                <ShoppingBag className="h-12 w-12 text-primary" />
              </div>
              <CardTitle className="text-center">Wide Selection</CardTitle>
            </CardHeader>
            <CardContent>
              <CardDescription className="text-center">
                Browse through thousands of products across multiple categories to find exactly what you need.
              </CardDescription>
            </CardContent>
          </Card>

          <Card>
            <CardHeader>
              <div className="flex justify-center mb-4">
                <Package className="h-12 w-12 text-primary" />
              </div>
              <CardTitle className="text-center">Quality Products</CardTitle>
            </CardHeader>
            <CardContent>
              <CardDescription className="text-center">
                All our products are carefully selected and tested to ensure the highest quality standards.
              </CardDescription>
            </CardContent>
          </Card>

          <Card>
            <CardHeader>
              <div className="flex justify-center mb-4">
                <TruckIcon className="h-12 w-12 text-primary" />
              </div>
              <CardTitle className="text-center">Fast Delivery</CardTitle>
            </CardHeader>
            <CardContent>
              <CardDescription className="text-center">
                Get your orders delivered quickly and safely right to your doorstep with our express shipping.
              </CardDescription>
            </CardContent>
          </Card>
        </div>
      </section>

      {/* CTA Section */}
      <section className="py-12 text-center">
        <Card className="max-w-2xl mx-auto">
          <CardHeader>
            <CardTitle className="text-2xl">Ready to Start Shopping?</CardTitle>
            <CardDescription>
              Create an account today and get access to exclusive deals and offers.
            </CardDescription>
          </CardHeader>
          <CardContent>
            <Link to="/products">
              <Button size="lg">Browse Products</Button>
            </Link>
          </CardContent>
        </Card>
      </section>
    </div>
  )
}

export default HomePage
