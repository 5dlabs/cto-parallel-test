import { Link } from 'react-router-dom'
import { Button } from '@/components/ui/button'
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card'

function HomePage() {
  return (
    <div className="space-y-12">
      {/* Hero Section */}
      <section className="text-center space-y-6 py-12">
        <h1 className="text-4xl md:text-6xl font-bold tracking-tight">
          Welcome to Our Store
        </h1>
        <p className="text-xl text-muted-foreground max-w-2xl mx-auto">
          Discover amazing products at great prices. Shop the latest trends and exclusive deals.
        </p>
        <div className="flex gap-4 justify-center">
          <Link to="/products">
            <Button size="lg">
              Shop Now
            </Button>
          </Link>
          <Link to="/register">
            <Button size="lg" variant="outline">
              Sign Up
            </Button>
          </Link>
        </div>
      </section>

      {/* Features Section */}
      <section className="grid md:grid-cols-3 gap-6">
        <Card>
          <CardHeader>
            <CardTitle>Free Shipping</CardTitle>
            <CardDescription>
              On orders over $50
            </CardDescription>
          </CardHeader>
          <CardContent>
            <p className="text-sm text-muted-foreground">
              Enjoy free shipping on all orders over $50. No code needed.
            </p>
          </CardContent>
        </Card>

        <Card>
          <CardHeader>
            <CardTitle>Easy Returns</CardTitle>
            <CardDescription>
              30-day return policy
            </CardDescription>
          </CardHeader>
          <CardContent>
            <p className="text-sm text-muted-foreground">
              Not satisfied? Return your purchase within 30 days for a full refund.
            </p>
          </CardContent>
        </Card>

        <Card>
          <CardHeader>
            <CardTitle>Secure Payment</CardTitle>
            <CardDescription>
              100% secure transactions
            </CardDescription>
          </CardHeader>
          <CardContent>
            <p className="text-sm text-muted-foreground">
              Your payment information is always secure with our encrypted checkout.
            </p>
          </CardContent>
        </Card>
      </section>
    </div>
  )
}

export default HomePage
