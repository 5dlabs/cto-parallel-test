import React, { useState } from 'react';
import { Link } from 'react-router-dom';
import { Card, CardContent, CardDescription, CardFooter, CardHeader, CardTitle } from './ui/card';
import { Button } from './ui/button';
import { Badge } from './ui/badge';
import { ShoppingCart } from 'lucide-react';

const ProductList = () => {
  // This would normally come from an API or global state
  const [products] = useState([
    {
      id: 1,
      name: 'Wireless Headphones',
      description: 'Premium noise-canceling wireless headphones',
      price: 199.99,
      inventory: 25,
      image: 'https://via.placeholder.com/300x200?text=Headphones'
    },
    {
      id: 2,
      name: 'Smart Watch',
      description: 'Feature-rich smartwatch with health tracking',
      price: 299.99,
      inventory: 15,
      image: 'https://via.placeholder.com/300x200?text=Smart+Watch'
    },
    {
      id: 3,
      name: 'Laptop Stand',
      description: 'Ergonomic aluminum laptop stand',
      price: 49.99,
      inventory: 50,
      image: 'https://via.placeholder.com/300x200?text=Laptop+Stand'
    },
    {
      id: 4,
      name: 'Mechanical Keyboard',
      description: 'RGB backlit mechanical gaming keyboard',
      price: 129.99,
      inventory: 30,
      image: 'https://via.placeholder.com/300x200?text=Keyboard'
    },
    {
      id: 5,
      name: 'Wireless Mouse',
      description: 'Precision wireless gaming mouse',
      price: 79.99,
      inventory: 40,
      image: 'https://via.placeholder.com/300x200?text=Mouse'
    },
    {
      id: 6,
      name: 'USB-C Hub',
      description: 'Multi-port USB-C hub with HDMI and USB 3.0',
      price: 59.99,
      inventory: 60,
      image: 'https://via.placeholder.com/300x200?text=USB-C+Hub'
    },
  ]);

  const handleAddToCart = (productId) => {
    // This would normally dispatch to a cart state management system
    console.log(`Added product ${productId} to cart`);
  };

  return (
    <div className="container mx-auto px-4 py-8">
      <div className="mb-8">
        <h1 className="text-4xl font-bold mb-2">Our Products</h1>
        <p className="text-gray-600">
          Browse our selection of quality products
        </p>
      </div>

      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
        {products.map((product) => (
          <Card key={product.id} className="flex flex-col hover:shadow-lg transition-shadow">
            <CardHeader className="p-0">
              <img
                src={product.image}
                alt={product.name}
                className="w-full h-48 object-cover rounded-t-lg"
              />
            </CardHeader>
            <CardContent className="flex-grow pt-6">
              <div className="flex justify-between items-start mb-2">
                <CardTitle className="text-xl">{product.name}</CardTitle>
                {product.inventory < 20 && (
                  <Badge variant="destructive" className="text-xs">
                    Low Stock
                  </Badge>
                )}
              </div>
              <CardDescription className="mb-4">
                {product.description}
              </CardDescription>
              <div className="text-2xl font-bold text-primary">
                ${product.price.toFixed(2)}
              </div>
              <div className="text-sm text-gray-500 mt-1">
                {product.inventory} in stock
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
                className="flex-1 gap-2"
                disabled={product.inventory === 0}
              >
                <ShoppingCart className="h-4 w-4" />
                Add to Cart
              </Button>
            </CardFooter>
          </Card>
        ))}
      </div>
    </div>
  );
};

export default ProductList;
