import { Link } from "react-router-dom"
import { Button } from "@/components/ui/button"
import { Card, CardHeader, CardTitle, CardDescription, CardContent } from "@/components/ui/card"

export default function HomePage() {
  return (
    <div className="container py-8">
      <div className="mb-10 text-center">
        <h1 className="text-4xl font-bold tracking-tight">Welcome to ShopSmart</h1>
        <p className="mt-2 text-muted-foreground">Your one‑stop shop for great products.</p>
        <Button asChild className="mt-6">
          <Link to="/products">Browse Products</Link>
        </Button>
      </div>
      <div className="grid gap-6 md:grid-cols-3">
        {["Fast Delivery", "Secure Checkout", "Top Quality"].map((title) => (
          <Card key={title}>
            <CardHeader>
              <CardTitle>{title}</CardTitle>
              <CardDescription>We focus on what matters most.</CardDescription>
            </CardHeader>
            <CardContent>
              Shop with confidence. We value privacy and security.
            </CardContent>
          </Card>
        ))}
      </div>
    </div>
  )
}

