import React, { useState, useEffect } from 'react'
import { useParams, Link } from 'react-router-dom'
import { Button } from './ui/button'
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from './ui/card'
import { Badge } from './ui/badge'
import { ShoppingCart, ArrowLeft } from 'lucide-react'

const ProductDetail = () => {
  const { id } = useParams()
  const [product, setProduct] = useState(null)
  const [loading, setLoading] = useState(true)
  const [quantity, setQuantity] = useState(1)

  // Mock product data
  // In a real implementation, this would fetch from the API
  useEffect(() => {
    setTimeout(() => {
      const mockProducts = {
        1: {
          id: 1,
          name: 'Wireless Headphones',
          description: 'High-quality wireless headphones with noise cancellation',
          price: 99.99,
          inventory_count: 15,
          longDescription: 'Experience superior sound quality with our premium wireless headphones. Featuring active noise cancellation, comfortable over-ear design, and up to 30 hours of battery life. Perfect for music lovers, travelers, and professionals.',
          features: [
            'Active Noise Cancellation',
            '30 Hours Battery Life',
            'Bluetooth 5.0',
            'Comfortable Over-Ear Design',
            'Built-in Microphone',
          ],
        },
        2: {
          id: 2,
          name: 'Smart Watch',
          description: 'Feature-rich smartwatch with health tracking',
          price: 199.99,
          inventory_count: 8,
          longDescription: 'Stay connected and healthy with our advanced smartwatch. Track your fitness goals, monitor your heart rate, and receive notifications right on your wrist.',
          features: [
            'Heart Rate Monitor',
            'Sleep Tracking',
            'GPS Navigation',
            'Water Resistant',
            '7 Day Battery Life',
          ],
        },
        3: {
          id: 3,
          name: 'Laptop Stand',
          description: 'Ergonomic laptop stand for better posture',
          price: 49.99,
          inventory_count: 25,
          longDescription: 'Improve your workspace ergonomics with this adjustable laptop stand. Designed to reduce neck and back strain while working.',
          features: [
            'Adjustable Height',
            'Aluminum Construction',
            'Cable Management',
            'Non-Slip Base',
            'Supports up to 15" Laptops',
          ],
        },
      }
      setProduct(mockProducts[id] || null)
      setLoading(false)
    }, 500)
  }, [id])

  const formatPrice = (price) => {
    return new Intl.NumberFormat('en-US', {
      style: 'currency',
      currency: 'USD',
    }).format(price)
  }

  const handleQuantityChange = (delta) => {
    const newQuantity = quantity + delta
    if (newQuantity >= 1 && newQuantity <= (product?.inventory_count || 1)) {
      setQuantity(newQuantity)
    }
  }

  if (loading) {
    return (
      <div className="container mx-auto px-4 py-8">
        <div className="flex justify-center items-center min-h-[400px]">
          <p className="text-lg text-muted-foreground">Loading product...</p>
        </div>
      </div>
    )
  }

  if (!product) {
    return (
      <div className="container mx-auto px-4 py-8">
        <Card>
          <CardContent className="py-8">
            <p className="text-center text-muted-foreground">Product not found</p>
            <div className="flex justify-center mt-4">
              <Link to="/products">
                <Button>
                  <ArrowLeft className="h-4 w-4 mr-2" />
                  Back to Products
                </Button>
              </Link>
            </div>
          </CardContent>
        </Card>
      </div>
    )
  }

  return (
    <div className="container mx-auto px-4 py-8">
      <Link to="/products" className="inline-flex items-center text-sm text-muted-foreground hover:text-primary mb-6">
        <ArrowLeft className="h-4 w-4 mr-2" />
        Back to Products
      </Link>

      <div className="grid grid-cols-1 lg:grid-cols-2 gap-8">
        {/* Product Image Placeholder */}
        <div className="aspect-square bg-muted rounded-lg flex items-center justify-center">
          <p className="text-muted-foreground">Product Image</p>
        </div>

        {/* Product Details */}
        <div className="space-y-6">
          <div>
            <div className="flex items-start justify-between mb-2">
              <h1 className="text-3xl font-bold tracking-tight">{product.name}</h1>
              {product.inventory_count < 10 && (
                <Badge variant="destructive">Low Stock</Badge>
              )}
            </div>
            <p className="text-muted-foreground">{product.description}</p>
          </div>

          <div className="text-4xl font-bold text-primary">
            {formatPrice(product.price)}
          </div>

          <Card>
            <CardHeader>
              <CardTitle>Description</CardTitle>
            </CardHeader>
            <CardContent>
              <p className="text-muted-foreground">{product.longDescription}</p>
            </CardContent>
          </Card>

          <Card>
            <CardHeader>
              <CardTitle>Features</CardTitle>
            </CardHeader>
            <CardContent>
              <ul className="space-y-2">
                {product.features.map((feature, index) => (
                  <li key={index} className="flex items-center">
                    <span className="h-1.5 w-1.5 rounded-full bg-primary mr-3" />
                    {feature}
                  </li>
                ))}
              </ul>
            </CardContent>
          </Card>

          <div className="flex items-center gap-4">
            <div className="flex items-center border rounded-md">
              <Button
                variant="ghost"
                size="sm"
                onClick={() => handleQuantityChange(-1)}
                disabled={quantity <= 1}
              >
                -
              </Button>
              <span className="px-4 py-2 min-w-[3rem] text-center">{quantity}</span>
              <Button
                variant="ghost"
                size="sm"
                onClick={() => handleQuantityChange(1)}
                disabled={quantity >= product.inventory_count}
              >
                +
              </Button>
            </div>
            <Button size="lg" className="flex-1">
              <ShoppingCart className="h-4 w-4 mr-2" />
              Add to Cart
            </Button>
          </div>

          <p className="text-sm text-muted-foreground">
            {product.inventory_count} items in stock
          </p>
        </div>
      </div>
    </div>
  )
}

export default ProductDetail
