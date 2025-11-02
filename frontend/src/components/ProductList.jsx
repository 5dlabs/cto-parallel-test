import React, { useState, useEffect } from 'react'
import { Link } from 'react-router-dom'
import { Button } from './ui/button'
import { Card, CardContent, CardDescription, CardFooter, CardHeader, CardTitle } from './ui/card'
import { Badge } from './ui/badge'
import { ShoppingCart } from 'lucide-react'

const ProductList = () => {
  const [products, setProducts] = useState([])
  const [loading, setLoading] = useState(true)

  // Mock products for demonstration
  // In a real implementation, this would fetch from the API
  useEffect(() => {
    // Simulate API call
    setTimeout(() => {
      setProducts([
        {
          id: 1,
          name: 'Wireless Headphones',
          description: 'High-quality wireless headphones with noise cancellation',
          price: 99.99,
          inventory_count: 15,
        },
        {
          id: 2,
          name: 'Smart Watch',
          description: 'Feature-rich smartwatch with health tracking',
          price: 199.99,
          inventory_count: 8,
        },
        {
          id: 3,
          name: 'Laptop Stand',
          description: 'Ergonomic laptop stand for better posture',
          price: 49.99,
          inventory_count: 25,
        },
        {
          id: 4,
          name: 'USB-C Hub',
          description: 'Multi-port USB-C hub with HDMI and USB ports',
          price: 39.99,
          inventory_count: 30,
        },
        {
          id: 5,
          name: 'Mechanical Keyboard',
          description: 'RGB mechanical keyboard with Cherry MX switches',
          price: 149.99,
          inventory_count: 12,
        },
        {
          id: 6,
          name: 'Wireless Mouse',
          description: 'Ergonomic wireless mouse with precision tracking',
          price: 29.99,
          inventory_count: 20,
        },
      ])
      setLoading(false)
    }, 500)
  }, [])

  const formatPrice = (price) => {
    return new Intl.NumberFormat('en-US', {
      style: 'currency',
      currency: 'USD',
    }).format(price)
  }

  if (loading) {
    return (
      <div className="container mx-auto px-4 py-8">
        <div className="flex justify-center items-center min-h-[400px]">
          <p className="text-lg text-muted-foreground">Loading products...</p>
        </div>
      </div>
    )
  }

  return (
    <div className="container mx-auto px-4 py-8">
      <div className="mb-8">
        <h1 className="text-4xl font-bold tracking-tight mb-2">Products</h1>
        <p className="text-muted-foreground">Browse our selection of quality products</p>
      </div>

      <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-6">
        {products.map((product) => (
          <Card key={product.id} className="flex flex-col">
            <CardHeader>
              <div className="flex justify-between items-start mb-2">
                <CardTitle className="text-xl">{product.name}</CardTitle>
                {product.inventory_count < 10 && (
                  <Badge variant="destructive">Low Stock</Badge>
                )}
              </div>
              <CardDescription>{product.description}</CardDescription>
            </CardHeader>
            <CardContent className="flex-grow">
              <div className="text-2xl font-bold text-primary">
                {formatPrice(product.price)}
              </div>
              <p className="text-sm text-muted-foreground mt-2">
                {product.inventory_count} in stock
              </p>
            </CardContent>
            <CardFooter className="flex gap-2">
              <Link to={`/products/${product.id}`} className="flex-1">
                <Button variant="outline" className="w-full">
                  View Details
                </Button>
              </Link>
              <Button className="flex-1">
                <ShoppingCart className="h-4 w-4 mr-2" />
                Add to Cart
              </Button>
            </CardFooter>
          </Card>
        ))}
      </div>
    </div>
  )
}

export default ProductList
