import React, { useState } from 'react';
import { useParams, Link } from 'react-router-dom';
import { Button } from './ui/button';
import { Badge } from './ui/badge';
import { Card, CardContent } from './ui/card';
import { ArrowLeft, Minus, Plus } from 'lucide-react';

function ProductDetail() {
  const { id } = useParams();
  const [quantity, setQuantity] = useState(1);

  // Mock product data - would be fetched from API in a real app
  const product = {
    id: parseInt(id),
    name: 'Wireless Headphones',
    description: 'Premium noise-cancelling wireless headphones with superior sound quality and comfort. Features include active noise cancellation, 30-hour battery life, and premium audio drivers.',
    price: 99.99,
    inventory: 15,
    image: 'https://via.placeholder.com/600x400?text=Wireless+Headphones',
    features: [
      'Active Noise Cancellation',
      '30-hour battery life',
      'Bluetooth 5.0 connectivity',
      'Premium audio drivers',
      'Comfortable ear cushions',
      'Foldable design'
    ]
  };

  const handleQuantityChange = (delta) => {
    const newQuantity = quantity + delta;
    if (newQuantity >= 1 && newQuantity <= product.inventory) {
      setQuantity(newQuantity);
    }
  };

  const handleAddToCart = () => {
    // This would add to cart in a real app
    console.log(`Adding ${quantity} of product ${product.id} to cart`);
  };

  return (
    <div className="container mx-auto px-4 py-8">
      <Link to="/products" className="inline-flex items-center text-primary hover:underline mb-6">
        <ArrowLeft className="h-4 w-4 mr-2" />
        Back to Products
      </Link>

      <div className="grid grid-cols-1 lg:grid-cols-2 gap-8">
        {/* Product Image */}
        <div>
          <img
            src={product.image}
            alt={product.name}
            className="w-full rounded-lg shadow-lg"
          />
        </div>

        {/* Product Info */}
        <div>
          <div className="mb-4">
            <h1 className="text-4xl font-bold mb-2">{product.name}</h1>
            {product.inventory < 10 && (
              <Badge variant="destructive">Only {product.inventory} left in stock</Badge>
            )}
          </div>

          <div className="text-3xl font-bold text-primary mb-6">
            ${product.price.toFixed(2)}
          </div>

          <p className="text-lg text-muted-foreground mb-6">
            {product.description}
          </p>

          <Card className="mb-6">
            <CardContent className="pt-6">
              <h3 className="font-semibold mb-3">Key Features:</h3>
              <ul className="space-y-2">
                {product.features.map((feature, index) => (
                  <li key={index} className="flex items-center">
                    <span className="text-primary mr-2">✓</span>
                    {feature}
                  </li>
                ))}
              </ul>
            </CardContent>
          </Card>

          {/* Quantity Selector */}
          <div className="mb-6">
            <label className="block text-sm font-medium mb-2">Quantity</label>
            <div className="flex items-center gap-3">
              <Button
                variant="outline"
                size="icon"
                onClick={() => handleQuantityChange(-1)}
                disabled={quantity <= 1}
              >
                <Minus className="h-4 w-4" />
              </Button>
              <span className="text-xl font-semibold w-12 text-center">{quantity}</span>
              <Button
                variant="outline"
                size="icon"
                onClick={() => handleQuantityChange(1)}
                disabled={quantity >= product.inventory}
              >
                <Plus className="h-4 w-4" />
              </Button>
            </div>
          </div>

          {/* Add to Cart Button */}
          <Button
            size="lg"
            className="w-full mb-4"
            onClick={handleAddToCart}
            disabled={product.inventory === 0}
          >
            {product.inventory === 0 ? 'Out of Stock' : 'Add to Cart'}
          </Button>

          <p className="text-sm text-muted-foreground text-center">
            Free shipping on orders over $100
          </p>
        </div>
      </div>
    </div>
  );
}

export default ProductDetail;
