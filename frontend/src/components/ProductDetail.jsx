import React, { useState } from 'react';
import { useParams, useNavigate } from 'react-router-dom';
import { Button } from './ui/button';
import { Badge } from './ui/badge';
import { Card, CardContent } from './ui/card';
import { ShoppingCart, ArrowLeft, Package, Truck, Shield } from 'lucide-react';

const ProductDetail = () => {
  const { id } = useParams();
  const navigate = useNavigate();
  const [quantity, setQuantity] = useState(1);

  // Mock product data - would normally come from API
  const products = {
    1: {
      id: 1,
      name: 'Wireless Headphones',
      description: 'Premium noise-canceling wireless headphones with superior sound quality',
      longDescription: 'Experience crystal-clear audio with our premium wireless headphones. Featuring advanced noise-canceling technology, 30-hour battery life, and comfortable over-ear design. Perfect for music lovers, travelers, and professionals.',
      price: 199.99,
      inventory: 25,
      image: 'https://via.placeholder.com/600x400?text=Headphones',
      features: ['Noise Canceling', '30hr Battery', 'Bluetooth 5.0', 'Comfortable Design']
    },
    2: {
      id: 2,
      name: 'Smart Watch',
      description: 'Feature-rich smartwatch with health tracking',
      longDescription: 'Stay connected and healthy with our advanced smartwatch. Track your fitness, monitor your heart rate, receive notifications, and more. Water-resistant design with a vibrant AMOLED display.',
      price: 299.99,
      inventory: 15,
      image: 'https://via.placeholder.com/600x400?text=Smart+Watch',
      features: ['Heart Rate Monitor', 'GPS Tracking', 'Water Resistant', '5-day Battery']
    },
    3: {
      id: 3,
      name: 'Laptop Stand',
      description: 'Ergonomic aluminum laptop stand',
      longDescription: 'Improve your posture and productivity with our premium aluminum laptop stand. Adjustable height, ventilated design, and sturdy construction for laptops up to 17 inches.',
      price: 49.99,
      inventory: 50,
      image: 'https://via.placeholder.com/600x400?text=Laptop+Stand',
      features: ['Adjustable Height', 'Aluminum Build', 'Ventilated Design', 'Anti-Slip Pads']
    },
  };

  const product = products[id] || products[1];

  const handleAddToCart = () => {
    console.log(`Added ${quantity} of product ${id} to cart`);
    // Would normally update cart state
    navigate('/cart');
  };

  const handleQuantityChange = (change) => {
    const newQuantity = quantity + change;
    if (newQuantity >= 1 && newQuantity <= product.inventory) {
      setQuantity(newQuantity);
    }
  };

  return (
    <div className="container mx-auto px-4 py-8">
      <Button
        variant="ghost"
        onClick={() => navigate('/products')}
        className="mb-6 gap-2"
      >
        <ArrowLeft className="h-4 w-4" />
        Back to Products
      </Button>

      <div className="grid grid-cols-1 lg:grid-cols-2 gap-8">
        {/* Product Image */}
        <div className="space-y-4">
          <img
            src={product.image}
            alt={product.name}
            className="w-full rounded-lg shadow-lg"
          />
        </div>

        {/* Product Details */}
        <div className="space-y-6">
          <div>
            <div className="flex items-start justify-between mb-2">
              <h1 className="text-3xl font-bold">{product.name}</h1>
              {product.inventory < 20 && (
                <Badge variant="destructive">Low Stock</Badge>
              )}
            </div>
            <p className="text-gray-600 text-lg">{product.description}</p>
          </div>

          <div className="text-4xl font-bold text-primary">
            ${product.price.toFixed(2)}
          </div>

          <div className="space-y-2">
            <h3 className="font-semibold text-lg">Description</h3>
            <p className="text-gray-700">{product.longDescription}</p>
          </div>

          <div className="space-y-2">
            <h3 className="font-semibold text-lg">Features</h3>
            <ul className="grid grid-cols-2 gap-2">
              {product.features.map((feature, index) => (
                <li key={index} className="flex items-center gap-2 text-gray-700">
                  <div className="w-2 h-2 bg-primary rounded-full" />
                  {feature}
                </li>
              ))}
            </ul>
          </div>

          <Card>
            <CardContent className="pt-6">
              <div className="flex items-center justify-between mb-4">
                <span className="font-medium">Quantity:</span>
                <div className="flex items-center gap-3">
                  <Button
                    variant="outline"
                    size="sm"
                    onClick={() => handleQuantityChange(-1)}
                    disabled={quantity <= 1}
                  >
                    -
                  </Button>
                  <span className="text-lg font-semibold w-12 text-center">
                    {quantity}
                  </span>
                  <Button
                    variant="outline"
                    size="sm"
                    onClick={() => handleQuantityChange(1)}
                    disabled={quantity >= product.inventory}
                  >
                    +
                  </Button>
                </div>
              </div>
              <div className="text-sm text-gray-600 mb-4">
                {product.inventory} items available
              </div>
              <Button
                onClick={handleAddToCart}
                className="w-full gap-2"
                size="lg"
                disabled={product.inventory === 0}
              >
                <ShoppingCart className="h-5 w-5" />
                Add to Cart
              </Button>
            </CardContent>
          </Card>

          <div className="grid grid-cols-1 md:grid-cols-3 gap-4 pt-4">
            <div className="flex items-center gap-3">
              <Package className="h-8 w-8 text-primary" />
              <div>
                <div className="font-medium text-sm">Free Shipping</div>
                <div className="text-xs text-gray-600">On orders over $50</div>
              </div>
            </div>
            <div className="flex items-center gap-3">
              <Truck className="h-8 w-8 text-primary" />
              <div>
                <div className="font-medium text-sm">Fast Delivery</div>
                <div className="text-xs text-gray-600">2-3 business days</div>
              </div>
            </div>
            <div className="flex items-center gap-3">
              <Shield className="h-8 w-8 text-primary" />
              <div>
                <div className="font-medium text-sm">Secure Payment</div>
                <div className="text-xs text-gray-600">100% secure</div>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
};

export default ProductDetail;
