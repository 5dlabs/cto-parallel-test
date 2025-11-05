import React, { useState, useEffect } from 'react';
import { Link } from 'react-router-dom';
import { Card, CardHeader, CardTitle, CardDescription, CardContent, CardFooter } from './ui/card';
import { Button } from './ui/button';
import { Badge } from './ui/badge';
import { productsApi } from '../services/api';

function ProductList() {
  const [products, setProducts] = useState([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState(null);

  useEffect(() => {
    const fetchProducts = async () => {
      try {
        setLoading(true);
        setError(null);
        const response = await productsApi.getAll();
        setProducts(response.data);
      } catch (err) {
        setError(err.response?.data?.message || 'Failed to load products');
        console.error('Error fetching products:', err);
      } finally {
        setLoading(false);
      }
    };

    fetchProducts();
  }, []);

  const formatPrice = (price) => {
    const numPrice = typeof price === 'string' ? parseFloat(price) : price;
    return numPrice.toFixed(2);
  };

  if (loading) {
    return (
      <div className="container mx-auto px-4 py-8">
        <h1 className="text-4xl font-bold mb-8">Our Products</h1>
        <div className="flex justify-center items-center min-h-[400px]">
          <p className="text-lg text-muted-foreground">Loading products...</p>
        </div>
      </div>
    );
  }

  if (error) {
    return (
      <div className="container mx-auto px-4 py-8">
        <h1 className="text-4xl font-bold mb-8">Our Products</h1>
        <div className="flex flex-col justify-center items-center min-h-[400px] space-y-4">
          <p className="text-lg text-destructive">{error}</p>
          <Button onClick={() => window.location.reload()}>Retry</Button>
        </div>
      </div>
    );
  }

  return (
    <div className="container mx-auto px-4 py-8">
      <h1 className="text-4xl font-bold mb-8">Our Products</h1>

      {products.length === 0 ? (
        <div className="flex justify-center items-center min-h-[400px]">
          <p className="text-lg text-muted-foreground">No products available</p>
        </div>
      ) : (
        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
          {products.map((product) => (
          <Card key={product.id} className="flex flex-col">
            <CardHeader>
              <div className="flex justify-between items-start">
                <CardTitle className="text-xl">{product.name}</CardTitle>
                {product.inventory_count === 0 && (
                  <Badge variant="destructive">Out of Stock</Badge>
                )}
                {product.inventory_count > 0 && product.inventory_count < 10 && (
                  <Badge variant="secondary">Low Stock</Badge>
                )}
              </div>
              <CardDescription>{product.description}</CardDescription>
            </CardHeader>
            <CardContent className="flex-grow">
              <p className="text-3xl font-bold text-primary">
                ${formatPrice(product.price)}
              </p>
              <p className="text-sm text-muted-foreground mt-2">
                {product.inventory_count > 0 
                  ? product.inventory_count + ' in stock'
                  : 'Currently unavailable'
                }
              </p>
            </CardContent>
            <CardFooter className="flex gap-2">
              <Link to={'/products/' + product.id} className="flex-1">
                <Button variant="outline" className="w-full">
                  View Details
                </Button>
              </Link>
              <Button 
                className="flex-1" 
                disabled={product.inventory_count === 0}
              >
                Add to Cart
              </Button>
            </CardFooter>
          </Card>
          ))}
        </div>
      )}
    </div>
  );
}

export default ProductList;
