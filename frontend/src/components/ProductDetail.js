import React, { useState, useEffect } from 'react';
import { useParams, Link } from 'react-router-dom';
import { Card, CardContent, CardFooter, CardHeader, CardTitle } from './ui/card';
import { Button } from './ui/button';
import { Badge } from './ui/badge';
import { ShoppingCart, ArrowLeft } from 'lucide-react';

function ProductDetail() {
  const { id } = useParams();
  const [product, setProduct] = useState(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState(null);
  const [quantity, setQuantity] = useState(1);

  useEffect(() => {
    fetchProduct();
  }, [id]);

  const fetchProduct = async () => {
    try {
      // Mock product data
      const mockProducts = {
        '1': {
          id: 1,
          name: 'Wireless Headphones',
          description: 'High-quality wireless headphones with noise cancellation',
          fullDescription: 'Experience premium sound quality with our wireless headphones. Features include active noise cancellation, 30-hour battery life, comfortable ear cushions, and Bluetooth 5.0 connectivity. Perfect for music lovers and professionals alike.',
          price: 99.99,
          inventory_count: 15,
          features: ['Active Noise Cancellation', '30-hour Battery Life', 'Bluetooth 5.0', 'Comfortable Fit']
        },
        '2': {
          id: 2,
          name: 'Smartphone Stand',
          description: 'Adjustable aluminum stand for all smartphones',
          fullDescription: 'Keep your phone at the perfect viewing angle with our premium aluminum stand. Adjustable design works with all smartphone sizes, non-slip base ensures stability, and elegant finish complements any desk setup.',
          price: 24.99,
          inventory_count: 50,
          features: ['Adjustable Angle', 'Aluminum Construction', 'Non-slip Base', 'Universal Compatibility']
        },
        '3': {
          id: 3,
          name: 'USB-C Cable',
          description: 'Durable braided USB-C cable, 6ft length',
          fullDescription: 'Our premium USB-C cable features durable braided construction for long-lasting use. Supports fast charging and data transfer, compatible with all USB-C devices.',
          price: 12.99,
          inventory_count: 100,
          features: ['6ft Length', 'Braided Design', 'Fast Charging', 'Data Transfer']
        }
      };

      await new Promise(resolve => setTimeout(resolve, 300));

      const foundProduct = mockProducts[id];
      if (foundProduct) {
        setProduct(foundProduct);
      } else {
        setError('Product not found');
      }
      setLoading(false);
    } catch (err) {
      setError('Failed to load product');
      setLoading(false);
    }
  };

  const addToCart = () => {
    if (product) {
      alert(`Added ${quantity} x ${product.name} to cart!`);
      const currentCount = parseInt(localStorage.getItem('cartItemCount') || '0');
      localStorage.setItem('cartItemCount', (currentCount + quantity).toString());
    }
  };

  if (loading) {
    return (
      <div className="container mx-auto px-4 py-12">
        <div className="text-center">
          <p className="text-lg text-muted-foreground">Loading product...</p>
        </div>
      </div>
    );
  }

  if (error || !product) {
    return (
      <div className="container mx-auto px-4 py-12">
        <div className="text-center">
          <p className="text-lg text-destructive">{error || 'Product not found'}</p>
          <Link to="/products" className="mt-4 inline-block">
            <Button variant="outline">
              <ArrowLeft className="h-4 w-4 mr-2" />
              Back to Products
            </Button>
          </Link>
        </div>
      </div>
    );
  }

  return (
    <div className="container mx-auto px-4 py-12">
      <Link to="/products" className="inline-flex items-center text-muted-foreground hover:text-foreground mb-8">
        <ArrowLeft className="h-4 w-4 mr-2" />
        Back to Products
      </Link>

      <div className="grid md:grid-cols-2 gap-8">
        {/* Product Image Placeholder */}
        <div className="bg-muted rounded-lg aspect-square flex items-center justify-center">
          <p className="text-muted-foreground">Product Image</p>
        </div>

        {/* Product Info */}
        <div>
          <div className="mb-4">
            <h1 className="text-4xl font-bold mb-2">{product.name}</h1>
            {product.inventory_count < 10 && (
              <Badge variant="destructive">Only {product.inventory_count} left!</Badge>
            )}
          </div>

          <p className="text-3xl font-bold text-primary mb-6">
            ${product.price.toFixed(2)}
          </p>

          <p className="text-muted-foreground mb-6">
            {product.fullDescription || product.description}
          </p>

          {product.features && (
            <div className="mb-6">
              <h3 className="font-semibold mb-3">Key Features:</h3>
              <ul className="space-y-2">
                {product.features.map((feature, index) => (
                  <li key={index} className="flex items-center text-muted-foreground">
                    <span className="mr-2">•</span>
                    {feature}
                  </li>
                ))}
              </ul>
            </div>
          )}

          <div className="mb-6">
            <label className="block text-sm font-medium mb-2">Quantity:</label>
            <div className="flex items-center space-x-3">
              <Button
                variant="outline"
                size="icon"
                onClick={() => setQuantity(Math.max(1, quantity - 1))}
              >
                -
              </Button>
              <span className="text-lg font-semibold w-12 text-center">{quantity}</span>
              <Button
                variant="outline"
                size="icon"
                onClick={() => setQuantity(Math.min(product.inventory_count, quantity + 1))}
                disabled={quantity >= product.inventory_count}
              >
                +
              </Button>
            </div>
          </div>

          <div className="flex flex-col sm:flex-row gap-3">
            <Button
              size="lg"
              className="flex-1"
              onClick={addToCart}
              disabled={product.inventory_count === 0}
            >
              <ShoppingCart className="h-5 w-5 mr-2" />
              Add to Cart
            </Button>
            <Button size="lg" variant="outline" className="flex-1">
              Buy Now
            </Button>
          </div>

          <p className="text-sm text-muted-foreground mt-4">
            {product.inventory_count > 0 ? (
              `${product.inventory_count} items in stock`
            ) : (
              'Out of stock'
            )}
          </p>
        </div>
      </div>
    </div>
  );
}

export default ProductDetail;
