import { Card, CardHeader, CardTitle, CardContent } from '@/components/ui/card'
import { Link } from 'react-router-dom'

export default function HomePage() {
  return (
    <div className="space-y-6">
      <section className="text-center space-y-3">
        <h1 className="text-3xl font-bold">Welcome to Cipher Shop</h1>
        <p className="text-muted-foreground max-w-2xl mx-auto">
          Secure, modern e-commerce frontend built with React, Vite, Tailwind CSS, and shadcn/ui.
        </p>
        <Link to="/products" className="inline-block">
          <span className="px-4 py-2 rounded-md bg-primary text-primary-foreground">Browse Products</span>
        </Link>
      </section>

      <section className="grid sm:grid-cols-2 lg:grid-cols-3 gap-4">
        {["Fast","Secure","Accessible"].map((title) => (
          <Card key={title}>
            <CardHeader>
              <CardTitle>{title}</CardTitle>
            </CardHeader>
            <CardContent>
              <p className="text-sm text-muted-foreground">Built with best practices and security in mind.</p>
            </CardContent>
          </Card>
        ))}
      </section>
    </div>
  )
}

