import React from 'react'
import { useParams, Link } from 'react-router-dom'
import { Card, CardContent, CardDescription, CardFooter, CardHeader, CardTitle } from './ui/card'
import { Button } from './ui/button'
import { Badge } from './ui/badge'
import { ShoppingCart, ArrowLeft } from 'lucide-react'

// Mock product data - will be replaced with API calls
const mockProducts = [
  {
    id: 1,
    name: 'Wireless Headphones',
    description: 'Premium noise-cancelling wireless headphones with superior sound quality and comfort. Features include 30-hour battery life, quick charging, and multi-device connectivity.',
    price: 199.99,
    inventory_count: 25,
    category: 'Electronics',
    features: ['Noise Cancellation', '30-hour Battery', 'Bluetooth 5.0', 'Quick Charge']
  },
  {
    id: 2,
    name: 'Smart Watch',
    description: 'Fitness tracking smart watch with heart rate monitor, GPS, and water resistance. Track your workouts, monitor your health, and stay connected.',
    price: 299.99,
    inventory_count: 15,
    category: 'Electronics',
    features: ['Heart Rate Monitor', 'GPS Tracking', 'Water Resistant', 'Sleep Tracking']
  },
  {
    id: 3,
    name: 'Laptop Backpack',
    description: 'Durable laptop backpack with USB charging port and multiple compartments. Perfect for work, travel, or school.',
    price: 49.99,
    inventory_count: 50,
    category: 'Accessories',
    features: ['USB Charging Port', 'Water Resistant', 'Laptop Compartment', 'Anti-theft Design']
  },
  {
    id: 4,
    name: 'Bluetooth Speaker',
    description: 'Portable waterproof Bluetooth speaker with 360-degree sound. Perfect for outdoor adventures and parties.',
    price: 79.99,
    inventory_count: 30,
    category: 'Electronics',
    features: ['Waterproof', '360° Sound', '12-hour Battery', 'Wireless Pairing']
  },
  {
    id: 5,
    name: 'Yoga Mat',
    description: 'Non-slip eco-friendly yoga mat made from sustainable materials. Provides excellent cushioning and support.',
    price: 34.99,
    inventory_count: 40,
    category: 'Sports',
    features: ['Non-slip Surface', 'Eco-friendly', 'Extra Thick', 'Carrying Strap']
  },
  {
    id: 6,
    name: 'Coffee Maker',
    description: 'Programmable drip coffee maker with thermal carafe. Brew perfect coffee every morning with customizable settings.',
    price: 89.99,
    inventory_count: 20,
    category: 'Home',
    features: ['Programmable', 'Thermal Carafe', 'Auto Shut-off', 'Brew Strength Control']
  }
]

function ProductDetail() {
  const { id } = useParams()
  const [quantity, setQuantity] = React.useState(1)
  const product = mockProducts.find(p => p.id === parseInt(id))

  const handleAddToCart = () => {
    // This will be connected to cart state management later
    console.log('Add to cart:', product.id, 'quantity:', quantity)
    alert(`Added ${quantity} ${product.name}(s) to cart!`)
  }

  const incrementQuantity = () => {
    if (quantity < product.inventory_count) {
      setQuantity(quantity + 1)
    }
  }

  const decrementQuantity = () => {
    if (quantity > 1) {
      setQuantity(quantity - 1)
    }
  }

  if (!product) {
    return (
      <div className="container mx-auto px-4 py-8">
        <Card>
          <CardHeader>
            <CardTitle>Product Not Found</CardTitle>
            <CardDescription>
              The product you're looking for doesn't exist.
            </CardDescription>
          </CardHeader>
          <CardFooter>
            <Link to="/products">
              <Button>
                <ArrowLeft className="h-4 w-4 mr-2" />
                Back to Products
              </Button>
            </Link>
          </CardFooter>
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
        <Card className="overflow-hidden">
          <div className="aspect-square bg-muted flex items-center justify-center">
            <span className="text-4xl text-muted-foreground">📦</span>
          </div>
        </Card>

        {/* Product Information */}
        <div className="space-y-6">
          <div>
            <div className="flex items-start justify-between mb-2">
              <Badge variant="secondary">{product.category}</Badge>
              {product.inventory_count < 20 && (
                <Badge variant="destructive">Low Stock</Badge>
              )}
            </div>
            <h1 className="text-3xl md:text-4xl font-bold mb-2">{product.name}</h1>
            <p className="text-muted-foreground text-lg">{product.description}</p>
          </div>

          <div className="text-3xl font-bold text-primary">
            ${product.price.toFixed(2)}
          </div>

          {/* Features */}
          <Card>
            <CardHeader>
              <CardTitle>Key Features</CardTitle>
            </CardHeader>
            <CardContent>
              <ul className="space-y-2">
                {product.features.map((feature, index) => (
                  <li key={index} className="flex items-center">
                    <span className="mr-2">✓</span>
                    {feature}
                  </li>
                ))}
              </ul>
            </CardContent>
          </Card>

          {/* Quantity and Add to Cart */}
          <Card>
            <CardContent className="pt-6">
              <div className="space-y-4">
                <div>
                  <label className="text-sm font-medium mb-2 block">Quantity</label>
                  <div className="flex items-center gap-3">
                    <Button
                      variant="outline"
                      size="icon"
                      onClick={decrementQuantity}
                      disabled={quantity <= 1}
                    >
                      -
                    </Button>
                    <span className="text-xl font-semibold w-12 text-center">{quantity}</span>
                    <Button
                      variant="outline"
                      size="icon"
                      onClick={incrementQuantity}
                      disabled={quantity >= product.inventory_count}
                    >
                      +
                    </Button>
                    <span className="text-sm text-muted-foreground ml-2">
                      {product.inventory_count} available
                    </span>
                  </div>
                </div>

                <Button
                  size="lg"
                  className="w-full"
                  onClick={handleAddToCart}
                  disabled={product.inventory_count === 0}
                >
                  <ShoppingCart className="h-5 w-5 mr-2" />
                  Add to Cart
                </Button>
              </div>
            </CardContent>
          </Card>
        </div>
      </div>
    </div>
  )
}

export default ProductDetail
