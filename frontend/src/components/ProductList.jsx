import React from 'react'
import { Link } from 'react-router-dom'
import { Card, CardContent, CardDescription, CardFooter, CardHeader, CardTitle } from './ui/card'
import { Button } from './ui/button'
import { Badge } from './ui/badge'
import { ShoppingCart } from 'lucide-react'

function ProductList() {
  // Mock product data - in real app, this would come from API
  const [products, setProducts] = React.useState([
    {
      id: 1,
      name: 'Wireless Headphones',
      description: 'High-quality wireless headphones with noise cancellation',
      price: 99.99,
      inventory: 15,
      image: 'https://via.placeholder.com/300x200?text=Headphones'
    },
    {
      id: 2,
      name: 'Smart Watch',
      description: 'Feature-rich smartwatch with health tracking',
      price: 199.99,
      inventory: 8,
      image: 'https://via.placeholder.com/300x200?text=Smart+Watch'
    },
    {
      id: 3,
      name: 'Laptop Stand',
      description: 'Ergonomic aluminum laptop stand',
      price: 49.99,
      inventory: 23,
      image: 'https://via.placeholder.com/300x200?text=Laptop+Stand'
    },
    {
      id: 4,
      name: 'Mechanical Keyboard',
      description: 'RGB mechanical keyboard with custom switches',
      price: 129.99,
      inventory: 12,
      image: 'https://via.placeholder.com/300x200?text=Keyboard'
    },
    {
      id: 5,
      name: 'Wireless Mouse',
      description: 'Precision wireless mouse with ergonomic design',
      price: 39.99,
      inventory: 30,
      image: 'https://via.placeholder.com/300x200?text=Mouse'
    },
    {
      id: 6,
      name: 'USB-C Hub',
      description: 'Multi-port USB-C hub with power delivery',
      price: 59.99,
      inventory: 18,
      image: 'https://via.placeholder.com/300x200?text=USB-C+Hub'
    }
  ])

  const handleAddToCart = (productId) => {
    console.log(`Adding product ${productId} to cart`)
    // In real app, this would call API endpoint
  }

  return (
    <div className="space-y-6">
      <div className="flex items-center justify-between">
        <h1 className="text-3xl font-bold">Our Products</h1>
        <p className="text-muted-foreground">{products.length} products available</p>
      </div>

      <div className="grid md:grid-cols-2 lg:grid-cols-3 gap-6">
        {products.map((product) => (
          <Card key={product.id} className="flex flex-col">
            <CardHeader>
              <div className="aspect-video bg-muted rounded-md mb-4 overflow-hidden">
                <img
                  src={product.image}
                  alt={product.name}
                  className="w-full h-full object-cover"
                />
              </div>
              <div className="flex items-start justify-between gap-2">
                <CardTitle className="line-clamp-1">{product.name}</CardTitle>
                <Badge variant={product.inventory > 10 ? "secondary" : "destructive"}>
                  {product.inventory} in stock
                </Badge>
              </div>
              <CardDescription className="line-clamp-2">{product.description}</CardDescription>
            </CardHeader>

            <CardContent className="flex-grow">
              <p className="text-2xl font-bold">${product.price.toFixed(2)}</p>
            </CardContent>

            <CardFooter className="flex gap-2">
              <Link to={`/products/${product.id}`} className="flex-1">
                <Button variant="outline" className="w-full">
                  View Details
                </Button>
              </Link>
              <Button
                onClick={() => handleAddToCart(product.id)}
                disabled={product.inventory === 0}
                className="flex-1"
              >
                <ShoppingCart className="w-4 h-4 mr-2" />
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
