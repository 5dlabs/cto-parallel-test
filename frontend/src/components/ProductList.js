import React, { useState, useEffect } from 'react';
import { Link } from 'react-router-dom';
import { ShoppingCart } from 'lucide-react';
import { Card, CardContent, CardDescription, CardFooter, CardHeader, CardTitle } from './ui/card';
import { Button } from './ui/button';
import { Badge } from './ui/badge';

function ProductList() {
  const [products, setProducts] = useState([]);
  const [loading, setLoading] = useState(true);

  // Mock products for demonstration - will be replaced with API call
  useEffect(() => {
    const mockProducts = [
      {
        id: 1,
        name: 'Wireless Headphones',
        description: 'Premium noise-canceling headphones with 30-hour battery life',
        price: 199.99,
        inventory_count: 50,
        image: 'https://via.placeholder.com/300x300?text=Headphones',
      },
      {
        id: 2,
        name: 'Smart Watch',
        description: 'Fitness tracking with heart rate monitor and GPS',
        price: 299.99,
        inventory_count: 30,
        image: 'https://via.placeholder.com/300x300?text=Smart+Watch',
      },
      {
        id: 3,
        name: 'Laptop Stand',
        description: 'Ergonomic aluminum laptop stand with adjustable height',
        price: 49.99,
        inventory_count: 100,
        image: 'https://via.placeholder.com/300x300?text=Laptop+Stand',
      },
      {
        id: 4,
        name: 'USB-C Hub',
        description: '7-in-1 USB-C hub with HDMI, USB 3.0, and SD card reader',
        price: 39.99,
        inventory_count: 75,
        image: 'https://via.placeholder.com/300x300?text=USB-C+Hub',
      },
      {
        id: 5,
        name: 'Mechanical Keyboard',
        description: 'RGB backlit mechanical keyboard with blue switches',
        price: 129.99,
        inventory_count: 45,
        image: 'https://via.placeholder.com/300x300?text=Keyboard',
      },
      {
        id: 6,
        name: 'Wireless Mouse',
        description: 'Ergonomic wireless mouse with precision tracking',
        price: 59.99,
        inventory_count: 60,
        image: 'https://via.placeholder.com/300x300?text=Mouse',
      },
    ];

    // Simulate API delay
    setTimeout(() => {
      setProducts(mockProducts);
      setLoading(false);
    }, 500);
  }, []);

  const handleAddToCart = (productId) => {
    // This will be replaced with actual cart functionality
    console.log('Add to cart:', productId);
    alert('Product will be added to cart (API integration pending)');
  };

  if (loading) {
    return (
      <div className="flex justify-center items-center min-h-[400px]">
        <p className="text-muted-foreground">Loading products...</p>
      </div>
    );
  }

  return (
    <div className="space-y-6">
      <div className="flex flex-col md:flex-row justify-between items-start md:items-center gap-4">
        <div>
          <h1 className="text-3xl font-bold">Products</h1>
          <p className="text-muted-foreground">Browse our collection of amazing products</p>
        </div>
      </div>

      <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-6">
        {products.map((product) => (
          <Card key={product.id} className="flex flex-col">
            <CardHeader className="p-0">
              <img
                src={product.image}
                alt={product.name}
                className="w-full h-48 object-cover rounded-t-lg"
              />
            </CardHeader>
            <CardContent className="flex-grow pt-6">
              <div className="space-y-2">
                <div className="flex items-start justify-between">
                  <CardTitle className="text-lg">{product.name}</CardTitle>
                  {product.inventory_count > 0 ? (
                    <Badge variant="outline" className="text-xs">
                      In Stock
                    </Badge>
                  ) : (
                    <Badge variant="destructive" className="text-xs">
                      Out of Stock
                    </Badge>
                  )}
                </div>
                <CardDescription className="line-clamp-2">
                  {product.description}
                </CardDescription>
                <p className="text-2xl font-bold text-primary">
                  ${product.price.toFixed(2)}
                </p>
              </div>
            </CardContent>
            <CardFooter className="flex gap-2">
              <Link to={`/products/${product.id}`} className="flex-1">
                <Button variant="outline" className="w-full">
                  View Details
                </Button>
              </Link>
              <Button
                onClick={() => handleAddToCart(product.id)}
                disabled={product.inventory_count === 0}
                className="flex-1"
              >
                <ShoppingCart className="mr-2 h-4 w-4" />
                Add to Cart
              </Button>
            </CardFooter>
          </Card>
        ))}
      </div>

      {products.length === 0 && !loading && (
        <div className="text-center py-12">
          <p className="text-muted-foreground">No products found</p>
        </div>
      )}
    </div>
  );
}

export default ProductList;
