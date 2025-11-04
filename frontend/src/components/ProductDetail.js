import React, { useState, useEffect } from 'react';
import { useParams, useNavigate, Link } from 'react-router-dom';
import { ArrowLeft, ShoppingCart, Minus, Plus } from 'lucide-react';
import { Card, CardContent } from './ui/card';
import { Button } from './ui/button';
import { Badge } from './ui/badge';

function ProductDetail() {
  const { id } = useParams();
  const navigate = useNavigate();
  const [product, setProduct] = useState(null);
  const [loading, setLoading] = useState(true);
  const [quantity, setQuantity] = useState(1);

  // Mock product data - will be replaced with API call
  useEffect(() => {
    const mockProducts = {
      1: {
        id: 1,
        name: 'Wireless Headphones',
        description: 'Premium noise-canceling headphones with 30-hour battery life',
        price: 199.99,
        inventory_count: 50,
        image: 'https://via.placeholder.com/600x400?text=Headphones',
        features: [
          'Active Noise Cancellation',
          '30-hour battery life',
          'Bluetooth 5.0',
          'Comfortable over-ear design',
          'Foldable for easy storage',
        ],
      },
      2: {
        id: 2,
        name: 'Smart Watch',
        description: 'Fitness tracking with heart rate monitor and GPS',
        price: 299.99,
        inventory_count: 30,
        image: 'https://via.placeholder.com/600x400?text=Smart+Watch',
        features: [
          'Heart rate monitoring',
          'GPS tracking',
          'Water resistant',
          '5-day battery life',
          'Multiple sport modes',
        ],
      },
      3: {
        id: 3,
        name: 'Laptop Stand',
        description: 'Ergonomic aluminum laptop stand with adjustable height',
        price: 49.99,
        inventory_count: 100,
        image: 'https://via.placeholder.com/600x400?text=Laptop+Stand',
        features: [
          'Adjustable height',
          'Aluminum construction',
          'Improves posture',
          'Compatible with all laptops',
          'Non-slip rubber pads',
        ],
      },
    };

    // Simulate API delay
    setTimeout(() => {
      const foundProduct = mockProducts[id];
      if (foundProduct) {
        setProduct(foundProduct);
      }
      setLoading(false);
    }, 500);
  }, [id]);

  const handleQuantityChange = (delta) => {
    const newQuantity = quantity + delta;
    if (newQuantity >= 1 && newQuantity <= (product?.inventory_count || 1)) {
      setQuantity(newQuantity);
    }
  };

  const handleAddToCart = () => {
    // This will be replaced with actual cart functionality
    console.log('Add to cart:', { productId: product.id, quantity });
    alert(`Adding ${quantity} item(s) to cart (API integration pending)`);
  };

  if (loading) {
    return (
      <div className="flex justify-center items-center min-h-[400px]">
        <p className="text-muted-foreground">Loading product...</p>
      </div>
    );
  }

  if (!product) {
    return (
      <div className="text-center py-12">
        <h2 className="text-2xl font-bold mb-4">Product Not Found</h2>
        <p className="text-muted-foreground mb-6">
          The product you're looking for doesn't exist.
        </p>
        <Link to="/products">
          <Button>
            <ArrowLeft className="mr-2 h-4 w-4" />
            Back to Products
          </Button>
        </Link>
      </div>
    );
  }

  return (
    <div className="space-y-6">
      <Button
        variant="ghost"
        onClick={() => navigate('/products')}
        className="mb-4"
      >
        <ArrowLeft className="mr-2 h-4 w-4" />
        Back to Products
      </Button>

      <div className="grid grid-cols-1 lg:grid-cols-2 gap-8">
        {/* Product Image */}
        <div>
          <Card>
            <CardContent className="p-0">
              <img
                src={product.image}
                alt={product.name}
                className="w-full rounded-lg object-cover"
              />
            </CardContent>
          </Card>
        </div>

        {/* Product Details */}
        <div className="space-y-6">
          <div>
            <div className="flex items-start justify-between mb-2">
              <h1 className="text-3xl font-bold">{product.name}</h1>
              {product.inventory_count > 0 ? (
                <Badge variant="outline">In Stock</Badge>
              ) : (
                <Badge variant="destructive">Out of Stock</Badge>
              )}
            </div>
            <p className="text-3xl font-bold text-primary mb-4">
              ${product.price.toFixed(2)}
            </p>
            <p className="text-muted-foreground">{product.description}</p>
          </div>

          {/* Features */}
          {product.features && (
            <div>
              <h3 className="text-lg font-semibold mb-3">Features</h3>
              <ul className="space-y-2">
                {product.features.map((feature, index) => (
                  <li key={index} className="flex items-center text-sm">
                    <span className="mr-2 text-primary">•</span>
                    {feature}
                  </li>
                ))}
              </ul>
            </div>
          )}

          {/* Quantity and Add to Cart */}
          <div className="space-y-4">
            <div className="flex items-center space-x-4">
              <span className="text-sm font-medium">Quantity:</span>
              <div className="flex items-center border border-border rounded-md">
                <Button
                  variant="ghost"
                  size="icon"
                  onClick={() => handleQuantityChange(-1)}
                  disabled={quantity <= 1}
                >
                  <Minus className="h-4 w-4" />
                </Button>
                <span className="px-4 py-2 min-w-[3rem] text-center">{quantity}</span>
                <Button
                  variant="ghost"
                  size="icon"
                  onClick={() => handleQuantityChange(1)}
                  disabled={quantity >= product.inventory_count}
                >
                  <Plus className="h-4 w-4" />
                </Button>
              </div>
              <span className="text-sm text-muted-foreground">
                {product.inventory_count} available
              </span>
            </div>

            <Button
              className="w-full"
              size="lg"
              onClick={handleAddToCart}
              disabled={product.inventory_count === 0}
            >
              <ShoppingCart className="mr-2 h-5 w-5" />
              Add to Cart
            </Button>
          </div>
        </div>
      </div>
    </div>
  );
}

export default ProductDetail;
