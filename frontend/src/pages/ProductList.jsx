import { Link } from 'react-router-dom'
import { Button } from '@/components/ui/button'
import { Card, CardContent, CardDescription, CardFooter, CardHeader, CardTitle } from '@/components/ui/card'

function ProductList() {
  // Mock data - in a real app, this would come from API
  const products = [
    { id: 1, name: 'Premium Headphones', price: 299.99, description: 'High-quality wireless headphones', inStock: true },
    { id: 2, name: 'Smart Watch', price: 199.99, description: 'Feature-rich smartwatch', inStock: true },
    { id: 3, name: 'Laptop Stand', price: 49.99, description: 'Ergonomic laptop stand', inStock: true },
    { id: 4, name: 'Wireless Mouse', price: 79.99, description: 'Precision wireless mouse', inStock: false },
    { id: 5, name: 'USB-C Hub', price: 89.99, description: 'Multi-port USB-C hub', inStock: true },
    { id: 6, name: 'Mechanical Keyboard', price: 149.99, description: 'RGB mechanical keyboard', inStock: true },
  ]

  return (
    <div className="space-y-6">
      <div>
        <h1 className="text-3xl font-bold tracking-tight">Products</h1>
        <p className="text-muted-foreground mt-2">
          Browse our collection of quality products
        </p>
      </div>

      <div className="grid sm:grid-cols-2 lg:grid-cols-3 gap-6">
        {products.map((product) => (
          <Card key={product.id} className="flex flex-col">
            <CardHeader>
              <CardTitle>{product.name}</CardTitle>
              <CardDescription>{product.description}</CardDescription>
            </CardHeader>
            <CardContent className="flex-grow">
              <p className="text-2xl font-bold">${product.price}</p>
              <p className={`text-sm mt-2 ${product.inStock ? 'text-green-600' : 'text-red-600'}`}>
                {product.inStock ? 'In Stock' : 'Out of Stock'}
              </p>
            </CardContent>
            <CardFooter className="flex gap-2">
              <Link to={`/products/${product.id}`} className="flex-1">
                <Button variant="outline" className="w-full">
                  View Details
                </Button>
              </Link>
              <Button
                className="flex-1"
                disabled={!product.inStock}
              >
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
