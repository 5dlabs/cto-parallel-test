import React from 'react'
import { Link } from 'react-router-dom'
import { Card, CardContent, CardDescription, CardFooter, CardHeader, CardTitle } from './ui/card'
import { Button } from './ui/button'
import { Badge } from './ui/badge'
import { ShoppingCart } from 'lucide-react'

// Mock product data - will be replaced with API calls
const mockProducts = [
  {
    id: 1,
    name: 'Wireless Headphones',
    description: 'Premium noise-cancelling wireless headphones',
    price: 199.99,
    inventory_count: 25,
    category: 'Electronics'
  },
  {
    id: 2,
    name: 'Smart Watch',
    description: 'Fitness tracking smart watch with heart rate monitor',
    price: 299.99,
    inventory_count: 15,
    category: 'Electronics'
  },
  {
    id: 3,
    name: 'Laptop Backpack',
    description: 'Durable laptop backpack with USB charging port',
    price: 49.99,
    inventory_count: 50,
    category: 'Accessories'
  },
  {
    id: 4,
    name: 'Bluetooth Speaker',
    description: 'Portable waterproof Bluetooth speaker',
    price: 79.99,
    inventory_count: 30,
    category: 'Electronics'
  },
  {
    id: 5,
    name: 'Yoga Mat',
    description: 'Non-slip eco-friendly yoga mat',
    price: 34.99,
    inventory_count: 40,
    category: 'Sports'
  },
  {
    id: 6,
    name: 'Coffee Maker',
    description: 'Programmable drip coffee maker',
    price: 89.99,
    inventory_count: 20,
    category: 'Home'
  }
]

function ProductList() {
  const [products] = React.useState(mockProducts)

  const handleAddToCart = (productId) => {
    // This will be connected to cart state management later
    console.log('Add to cart:', productId)
    alert('Product added to cart!')
  }

  return (
    <div className="container mx-auto px-4 py-8">
      <div className="mb-8">
        <h1 className="text-3xl md:text-4xl font-bold mb-2">Our Products</h1>
        <p className="text-muted-foreground">
          Browse our collection of quality products
        </p>
      </div>

      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
        {products.map((product) => (
          <Card key={product.id} className="flex flex-col">
            <CardHeader>
              <div className="flex justify-between items-start mb-2">
                <Badge variant="secondary">{product.category}</Badge>
                {product.inventory_count < 20 && (
                  <Badge variant="destructive">Low Stock</Badge>
                )}
              </div>
              <CardTitle className="line-clamp-1">{product.name}</CardTitle>
              <CardDescription className="line-clamp-2">
                {product.description}
              </CardDescription>
            </CardHeader>
            <CardContent className="flex-1">
              <div className="text-2xl font-bold text-primary">
                ${product.price.toFixed(2)}
              </div>
              <div className="text-sm text-muted-foreground mt-1">
                {product.inventory_count} in stock
              </div>
            </CardContent>
            <CardFooter className="flex gap-2">
              <Link to={`/products/${product.id}`} className="flex-1">
                <Button variant="outline" className="w-full">
                  View Details
                </Button>
              </Link>
              <Button onClick={() => handleAddToCart(product.id)}>
                <ShoppingCart className="h-4 w-4 mr-2" />
                Add
              </Button>
            </CardFooter>
          </Card>
        ))}
      </div>

      {products.length === 0 && (
        <div className="text-center py-12">
          <p className="text-muted-foreground text-lg">No products found</p>
        </div>
      )}
    </div>
  )
}

export default ProductList
