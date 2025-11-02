import { useParams, Link } from 'react-router-dom'
import { Button } from '@/components/ui/button'
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card'
import { Badge } from '@/components/ui/badge'

function ProductDetail() {
  const { id } = useParams()

  // Mock data - in a real app, this would come from API
  const product = {
    id,
    name: 'Premium Headphones',
    price: 299.99,
    description: 'High-quality wireless headphones with noise cancellation',
    longDescription: 'Experience premium audio quality with our state-of-the-art wireless headphones. Featuring advanced noise cancellation technology, 40-hour battery life, and premium comfort padding for all-day wear.',
    inStock: true,
    features: [
      'Active Noise Cancellation',
      '40-hour Battery Life',
      'Bluetooth 5.0',
      'Premium Comfort Padding',
      'Foldable Design',
    ],
  }

  return (
    <div className="space-y-6">
      <div>
        <Link to="/products">
          <Button variant="ghost" className="mb-4">
            ← Back to Products
          </Button>
        </Link>
      </div>

      <div className="grid md:grid-cols-2 gap-8">
        <Card>
          <CardContent className="p-6">
            <div className="aspect-square bg-muted rounded-lg flex items-center justify-center">
              <p className="text-muted-foreground">Product Image</p>
            </div>
          </CardContent>
        </Card>

        <div className="space-y-6">
          <div>
            <h1 className="text-3xl font-bold tracking-tight">{product.name}</h1>
            <p className="text-2xl font-bold mt-2">${product.price}</p>
            {product.inStock ? (
              <Badge className="mt-2" variant="secondary">In Stock</Badge>
            ) : (
              <Badge className="mt-2" variant="destructive">Out of Stock</Badge>
            )}
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
                    <span className="mr-2">✓</span>
                    <span>{feature}</span>
                  </li>
                ))}
              </ul>
            </CardContent>
          </Card>

          <div className="flex gap-4">
            <Button className="flex-1" disabled={!product.inStock}>
              Add to Cart
            </Button>
            <Button variant="outline" className="flex-1">
              Add to Wishlist
            </Button>
          </div>
        </div>
      </div>
    </div>
  )
}

export default ProductDetail
