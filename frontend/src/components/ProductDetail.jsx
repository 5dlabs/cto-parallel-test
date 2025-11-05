import React, { useState, useEffect } from 'react';
import { useParams, Link, useNavigate } from 'react-router-dom';
import { Card, CardHeader, CardTitle, CardContent } from './ui/card';
import { Button } from './ui/button';
import { Badge } from './ui/badge';
import { ArrowLeft } from 'lucide-react';
import { productAPI, cartAPI } from '../services/api';

function ProductDetail() {
  const { id } = useParams();
  const navigate = useNavigate();
  const [product, setProduct] = useState(null);
  const [quantity, setQuantity] = useState(1);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState(null);
  const [addingToCart, setAddingToCart] = useState(false);

  useEffect(() => {
    const fetchProduct = async () => {
      try {
        setLoading(true);
        setError(null);
        const response = await productAPI.getById(id);
        setProduct(response.data);
      } catch (err) {
        console.error('Failed to fetch product:', err);
        setError('Failed to load product details.');
      } finally {
        setLoading(false);
      }
    };

    fetchProduct();
  }, [id]);

  const handleAddToCart = async () => {
    try {
      setAddingToCart(true);
      await cartAPI.addItem(product.id, quantity);
      alert('Product added to cart successfully!');
      navigate('/cart');
    } catch (err) {
      console.error('Failed to add to cart:', err);
      if (err.response?.status === 401) {
        alert('Please login to add items to cart');
        navigate('/login');
      } else {
        alert('Failed to add product to cart. Please try again.');
      }
    } finally {
      setAddingToCart(false);
    }
  };

  if (loading) {
    return (
      <div className="container mx-auto px-4 py-8">
        <div className="text-center">
          <p className="text-xl">Loading product details...</p>
        </div>
      </div>
    );
  }

  if (error || !product) {
    return (
      <div className="container mx-auto px-4 py-8">
        <p className="text-xl text-destructive mb-4">{error || 'Product not found'}</p>
        <Link to="/products">
          <Button variant="outline" className="mt-4">
            <ArrowLeft className="mr-2 h-4 w-4" />
            Back to Products
          </Button>
        </Link>
      </div>
    );
  }

  return (
    <div className="container mx-auto px-4 py-8">
      <Link to="/products">
        <Button variant="outline" className="mb-6">
          <ArrowLeft className="mr-2 h-4 w-4" />
          Back to Products
        </Button>
      </Link>

      <div className="grid md:grid-cols-2 gap-8">
        <div>
          <Card>
            <CardContent className="p-8">
              <div className="aspect-square bg-muted rounded-lg flex items-center justify-center">
                <p className="text-muted-foreground">Product Image</p>
              </div>
            </CardContent>
          </Card>
        </div>

        <div>
          <div className="mb-4">
            <div className="flex items-start justify-between mb-2">
              <h1 className="text-4xl font-bold">{product.name}</h1>
              {product.inventory_count === 0 && (
                <Badge variant="destructive">Out of Stock</Badge>
              )}
              {product.inventory_count > 0 && product.inventory_count < 10 && (
                <Badge variant="secondary">Low Stock</Badge>
              )}
            </div>
            <p className="text-xl text-muted-foreground mb-4">
              {product.description}
            </p>
            <p className="text-4xl font-bold text-primary mb-4">
              ${product.price.toFixed(2)}
            </p>
            <p className="text-sm text-muted-foreground">
              {product.inventory_count > 0 
                ? product.inventory_count + ' in stock'
                : 'Currently unavailable'
              }
            </p>
          </div>

          {product.features && (
            <Card className="mb-6">
              <CardHeader>
                <CardTitle>Features</CardTitle>
              </CardHeader>
              <CardContent>
                <ul className="list-disc list-inside space-y-2">
                  {product.features.map((feature, index) => (
                    <li key={index} className="text-muted-foreground">{feature}</li>
                  ))}
                </ul>
              </CardContent>
            </Card>
          )}

          <div className="space-y-4">
            <div className="flex items-center space-x-4">
              <label className="text-sm font-medium">Quantity:</label>
              <div className="flex items-center space-x-2">
                <Button
                  variant="outline"
                  size="icon"
                  onClick={() => setQuantity(Math.max(1, quantity - 1))}
                  disabled={product.inventory_count === 0}
                >
                  -
                </Button>
                <span className="w-12 text-center">{quantity}</span>
                <Button
                  variant="outline"
                  size="icon"
                  onClick={() => setQuantity(Math.min(product.inventory_count, quantity + 1))}
                  disabled={product.inventory_count === 0}
                >
                  +
                </Button>
              </div>
            </div>

            <div className="flex space-x-4">
              <Button
                className="flex-1"
                size="lg"
                disabled={product.inventory_count === 0 || addingToCart}
                onClick={handleAddToCart}
              >
                {addingToCart ? 'Adding...' : 'Add to Cart'}
              </Button>
              <Button
                variant="outline"
                size="lg"
                disabled={product.inventory_count === 0}
              >
                Buy Now
              </Button>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
}

export default ProductDetail;
