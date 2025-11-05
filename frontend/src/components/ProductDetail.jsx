import React, { useState, useEffect } from 'react';
import { useParams, Link, useNavigate } from 'react-router-dom';
import { Card, CardHeader, CardTitle, CardContent } from './ui/card';
import { Button } from './ui/button';
import { Badge } from './ui/badge';
import { ArrowLeft } from 'lucide-react';
import { productsApi, cartApi } from '../services/api';
import { useCart } from '../context/CartContext';

function ProductDetail() {
  const { id } = useParams();
  const navigate = useNavigate();
  const [product, setProduct] = useState(null);
  const [quantity, setQuantity] = useState(1);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState(null);
  const [addingToCart, setAddingToCart] = useState(false);
  const { refreshCart } = useCart();

  useEffect(() => {
    const fetchProduct = async () => {
      try {
        setLoading(true);
        setError(null);
        const response = await productsApi.getById(id);
        setProduct(response.data);
      } catch (err) {
        setError(err.response?.data?.message || 'Failed to load product');
        console.error('Error fetching product:', err);
      } finally {
        setLoading(false);
      }
    };

    fetchProduct();
  }, [id]);

  const handleAddToCart = async () => {
    try {
      setAddingToCart(true);
      await cartApi.addItem(product.id, quantity);
      refreshCart(); // Update header cart count
      alert('Product added to cart successfully!');
    } catch (err) {
      const errorMsg = err.response?.data?.message || 'Failed to add item to cart';
      alert(errorMsg);
      console.error('Error adding to cart:', err);
    } finally {
      setAddingToCart(false);
    }
  };

  const handleBuyNow = async () => {
    await handleAddToCart();
    navigate('/cart');
  };

  if (loading) {
    return (
      <div className="container mx-auto px-4 py-8">
        <div className="flex justify-center items-center min-h-[400px]">
          <p className="text-lg text-muted-foreground">Loading product...</p>
        </div>
      </div>
    );
  }

  if (error || !product) {
    return (
      <div className="container mx-auto px-4 py-8">
        <div className="flex flex-col justify-center items-center min-h-[400px] space-y-4">
          <p className="text-lg text-destructive">{error || 'Product not found'}</p>
          <Link to="/products">
            <Button variant="outline">
              <ArrowLeft className="mr-2 h-4 w-4" />
              Back to Products
            </Button>
          </Link>
        </div>
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
              ${typeof product.price === 'string' ? parseFloat(product.price).toFixed(2) : product.price.toFixed(2)}
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
                disabled={product.inventory_count === 0 || addingToCart}
                onClick={handleBuyNow}
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
