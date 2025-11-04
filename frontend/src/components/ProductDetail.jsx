import React from 'react'
import { useParams, useNavigate } from 'react-router-dom'
import { Button } from './ui/button'
import { Badge } from './ui/badge'
import { Card, CardContent } from './ui/card'
import { ShoppingCart, ArrowLeft, Star } from 'lucide-react'

function ProductDetail() {
  const { id } = useParams()
  const navigate = useNavigate()
  const [quantity, setQuantity] = React.useState(1)

  // Mock product data - in real app, this would come from API based on id
  const product = {
    id: parseInt(id),
    name: 'Wireless Headphones',
    description: 'High-quality wireless headphones with active noise cancellation technology. Features 30-hour battery life, premium audio drivers, and comfortable over-ear design.',
    price: 99.99,
    inventory: 15,
    image: 'https://via.placeholder.com/600x400?text=Product+Image',
    rating: 4.5,
    reviews: 127,
    features: [
      'Active Noise Cancellation',
      '30-hour battery life',
      'Premium audio quality',
      'Comfortable over-ear design',
      'Bluetooth 5.0 connectivity',
      'Built-in microphone'
    ]
  }

  const handleAddToCart = () => {
    console.log(`Adding ${quantity} of product ${id} to cart`)
    // In real app, this would call API endpoint
  }

  const handleQuantityChange = (delta) => {
    const newQuantity = quantity + delta
    if (newQuantity >= 1 && newQuantity <= product.inventory) {
      setQuantity(newQuantity)
    }
  }

  return (
    <div className="space-y-6">
      <Button variant="ghost" onClick={() => navigate(-1)}>
        <ArrowLeft className="w-4 h-4 mr-2" />
        Back to Products
      </Button>

      <div className="grid md:grid-cols-2 gap-8">
        {/* Product Image */}
        <Card>
          <CardContent className="p-6">
            <div className="aspect-square bg-muted rounded-lg overflow-hidden">
              <img
                src={product.image}
                alt={product.name}
                className="w-full h-full object-cover"
              />
            </div>
          </CardContent>
        </Card>

        {/* Product Details */}
        <div className="space-y-6">
          <div>
            <h1 className="text-3xl font-bold mb-2">{product.name}</h1>
            <div className="flex items-center gap-4 mb-4">
              <div className="flex items-center gap-1">
                <Star className="w-5 h-5 fill-primary text-primary" />
                <span className="font-medium">{product.rating}</span>
                <span className="text-muted-foreground">({product.reviews} reviews)</span>
              </div>
              <Badge variant={product.inventory > 10 ? "secondary" : "destructive"}>
                {product.inventory} in stock
              </Badge>
            </div>
            <p className="text-muted-foreground">{product.description}</p>
          </div>

          <div className="border-t pt-6">
            <h3 className="font-semibold mb-3">Key Features:</h3>
            <ul className="space-y-2">
              {product.features.map((feature, index) => (
                <li key={index} className="flex items-start gap-2">
                  <span className="text-primary mt-1">•</span>
                  <span>{feature}</span>
                </li>
              ))}
            </ul>
          </div>

          <Card>
            <CardContent className="p-6 space-y-4">
              <div className="flex items-baseline gap-2">
                <span className="text-3xl font-bold">${product.price.toFixed(2)}</span>
                <span className="text-muted-foreground">per unit</span>
              </div>

              <div className="flex items-center gap-4">
                <span className="font-medium">Quantity:</span>
                <div className="flex items-center gap-2">
                  <Button
                    variant="outline"
                    size="icon"
                    onClick={() => handleQuantityChange(-1)}
                    disabled={quantity <= 1}
                  >
                    -
                  </Button>
                  <span className="w-12 text-center font-medium">{quantity}</span>
                  <Button
                    variant="outline"
                    size="icon"
                    onClick={() => handleQuantityChange(1)}
                    disabled={quantity >= product.inventory}
                  >
                    +
                  </Button>
                </div>
              </div>

              <div className="flex items-baseline gap-2 pt-2 border-t">
                <span className="text-lg font-medium">Total:</span>
                <span className="text-2xl font-bold">${(product.price * quantity).toFixed(2)}</span>
              </div>

              <Button
                className="w-full"
                size="lg"
                onClick={handleAddToCart}
                disabled={product.inventory === 0}
              >
                <ShoppingCart className="w-5 h-5 mr-2" />
                Add to Cart
              </Button>
            </CardContent>
          </Card>
        </div>
      </div>
    </div>
  )
}

export default ProductDetail
