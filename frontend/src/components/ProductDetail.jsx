import React, { useState, useEffect } from 'react';
import { useParams, useNavigate } from 'react-router-dom';
import { Card, CardContent, CardDescription, CardFooter, CardHeader, CardTitle } from './ui/card';
import { Button } from './ui/button';
import { Badge } from './ui/badge';
import { ArrowLeft } from 'lucide-react';

function ProductDetail() {
  const { id } = useParams();
  const navigate = useNavigate();
  const [product, setProduct] = useState(null);
  const [loading, setLoading] = useState(true);
  const [quantity, setQuantity] = useState(1);

  useEffect(() => {
    fetchProduct();
  }, [id]);

  const fetchProduct = async () => {
    try {
      setLoading(true);
      // Mock data - will be replaced with API call
      const mockProducts = {
        1: {
          id: 1,
          name: 'Wireless Headphones',
          description: 'High-quality wireless headphones with noise cancellation',
          price: 99.99,
          inventory_count: 50,
          details: 'Premium wireless headphones featuring active noise cancellation, 30-hour battery life, and superior sound quality. Perfect for music lovers and professionals alike.'
        },
        2: {
          id: 2,
          name: 'Smart Watch',
          description: 'Feature-rich smartwatch with health tracking',
          price: 199.99,
          inventory_count: 30,
          details: 'Advanced smartwatch with heart rate monitoring, GPS, sleep tracking, and multiple workout modes. Stay connected and healthy with this versatile device.'
        },
        3: {
          id: 3,
          name: 'Laptop Stand',
          description: 'Ergonomic aluminum laptop stand',
          price: 49.99,
          inventory_count: 100,
          details: 'Sturdy aluminum laptop stand that improves posture and increases airflow. Compatible with all laptop sizes up to 17 inches.'
        },
        4: {
          id: 4,
          name: 'USB-C Hub',
          description: 'Multi-port USB-C hub with HDMI and USB 3.0',
          price: 39.99,
          inventory_count: 75,
          details: '7-in-1 USB-C hub featuring HDMI 4K output, 3x USB 3.0 ports, SD/microSD card readers, and USB-C power delivery.'
        },
        5: {
          id: 5,
          name: 'Mechanical Keyboard',
          description: 'RGB mechanical keyboard with blue switches',
          price: 129.99,
          inventory_count: 25,
          details: 'Full-size mechanical keyboard with customizable RGB lighting, Cherry MX Blue switches, and programmable macros for gaming and productivity.'
        },
        6: {
          id: 6,
          name: 'Wireless Mouse',
          description: 'Ergonomic wireless mouse with precision tracking',
          price: 59.99,
          inventory_count: 60,
          details: 'Ergonomic wireless mouse with adjustable DPI up to 4000, rechargeable battery lasting up to 3 months, and silent click technology.'
        }
      };

      const foundProduct = mockProducts[id];
      if (foundProduct) {
        setProduct(foundProduct);
      }
      setLoading(false);
    } catch (err) {
      setLoading(false);
    }
  };

  const addToCart = () => {
    if (!product) return;

    const cart = JSON.parse(localStorage.getItem('cart') || '[]');
    const existingItemIndex = cart.findIndex(item => item.id === product.id);

    if (existingItemIndex >= 0) {
      cart[existingItemIndex].quantity += quantity;
    } else {
      cart.push({ ...product, quantity });
    }

    localStorage.setItem('cart', JSON.stringify(cart));
    window.dispatchEvent(new Event('cartUpdated'));

    // Navigate to cart
    navigate('/cart');
  };

  if (loading) {
    return (
      <div className="text-center py-12">
        <p className="text-lg text-muted-foreground">Loading product...</p>
      </div>
    );
  }

  if (!product) {
    return (
      <div className="text-center py-12 space-y-4">
        <p className="text-lg text-destructive">Product not found</p>
        <Button onClick={() => navigate('/products')}>
          <ArrowLeft className="mr-2 h-4 w-4" />
          Back to Products
        </Button>
      </div>
    );
  }

  return (
    <div className="space-y-6">
      <Button variant="ghost" onClick={() => navigate('/products')}>
        <ArrowLeft className="mr-2 h-4 w-4" />
        Back to Products
      </Button>

      <Card>
        <CardHeader>
          <CardTitle className="text-3xl">{product.name}</CardTitle>
          <CardDescription className="text-lg">{product.description}</CardDescription>
        </CardHeader>
        <CardContent className="space-y-6">
          <div className="flex items-center gap-4">
            <p className="text-4xl font-bold">${product.price.toFixed(2)}</p>
            {product.inventory_count > 0 ? (
              <Badge variant="secondary" className="text-base px-3 py-1">
                {product.inventory_count} in stock
              </Badge>
            ) : (
              <Badge variant="destructive" className="text-base px-3 py-1">
                Out of stock
              </Badge>
            )}
          </div>

          <div className="prose max-w-none">
            <h3 className="text-xl font-semibold mb-2">Product Details</h3>
            <p className="text-muted-foreground">{product.details}</p>
          </div>

          <div className="flex items-center gap-4">
            <label className="text-sm font-medium">Quantity:</label>
            <div className="flex items-center gap-2">
              <Button
                variant="outline"
                size="sm"
                onClick={() => setQuantity(Math.max(1, quantity - 1))}
              >
                -
              </Button>
              <span className="w-12 text-center font-medium">{quantity}</span>
              <Button
                variant="outline"
                size="sm"
                onClick={() => setQuantity(Math.min(product.inventory_count, quantity + 1))}
                disabled={quantity >= product.inventory_count}
              >
                +
              </Button>
            </div>
          </div>
        </CardContent>
        <CardFooter>
          <Button
            size="lg"
            className="w-full md:w-auto"
            onClick={addToCart}
            disabled={product.inventory_count === 0}
          >
            Add to Cart
          </Button>
        </CardFooter>
      </Card>
    </div>
  );
}

export default ProductDetail;
