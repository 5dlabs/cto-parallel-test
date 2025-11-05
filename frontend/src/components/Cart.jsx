import React, { useState, useEffect } from 'react';
import { Link, useNavigate } from 'react-router-dom';
import { Card, CardHeader, CardTitle, CardContent } from './ui/card';
import { Button } from './ui/button';
import { Trash2, Plus, Minus, ShoppingBag } from 'lucide-react';
import { cartAPI } from '../services/api';

function Cart() {
  const navigate = useNavigate();
  const [cartItems, setCartItems] = useState([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState(null);

  useEffect(() => {
    fetchCart();
  }, []);

  const fetchCart = async () => {
    try {
      setLoading(true);
      setError(null);
      const response = await cartAPI.get();
      setCartItems(response.data.items || []);
    } catch (err) {
      console.error('Failed to fetch cart:', err);
      if (err.response?.status === 401) {
        setError('Please login to view your cart');
      } else {
        setError('Failed to load cart. Please try again.');
      }
    } finally {
      setLoading(false);
    }
  };

  const updateQuantity = async (productId, newQuantity) => {
    if (newQuantity < 1) return;

    try {
      // Optimistic update
      setCartItems(items =>
        items.map(item =>
          item.product_id === productId
            ? { ...item, quantity: newQuantity }
            : item
        )
      );

      // Update on server
      await cartAPI.addItem(productId, newQuantity);
    } catch (err) {
      console.error('Failed to update quantity:', err);
      // Revert on error
      fetchCart();
      alert('Failed to update quantity. Please try again.');
    }
  };

  const removeItem = async (productId) => {
    try {
      // Optimistic update
      setCartItems(items => items.filter(item => item.product_id !== productId));

      // Remove from server
      await cartAPI.removeItem(productId);
    } catch (err) {
      console.error('Failed to remove item:', err);
      // Revert on error
      fetchCart();
      alert('Failed to remove item. Please try again.');
    }
  };

  const calculateSubtotal = () => {
    return cartItems.reduce((sum, item) => sum + (item.price * item.quantity), 0);
  };

  const subtotal = calculateSubtotal();
  const tax = subtotal * 0.08;
  const total = subtotal + tax;

  if (loading) {
    return (
      <div className="container mx-auto px-4 py-8">
        <div className="text-center">
          <p className="text-xl">Loading cart...</p>
        </div>
      </div>
    );
  }

  if (error) {
    return (
      <div className="container mx-auto px-4 py-16">
        <Card className="max-w-md mx-auto text-center">
          <CardContent className="py-16">
            <ShoppingBag className="h-16 w-16 mx-auto mb-4 text-muted-foreground" />
            <p className="text-xl text-destructive mb-4">{error}</p>
            {error.includes('login') ? (
              <Link to="/login">
                <Button size="lg">Login</Button>
              </Link>
            ) : (
              <Button onClick={fetchCart} size="lg">
                Retry
              </Button>
            )}
          </CardContent>
        </Card>
      </div>
    );
  }

  if (cartItems.length === 0) {
    return (
      <div className="container mx-auto px-4 py-16">
        <Card className="max-w-md mx-auto text-center">
          <CardContent className="py-16">
            <ShoppingBag className="h-16 w-16 mx-auto mb-4 text-muted-foreground" />
            <h2 className="text-2xl font-bold mb-2">Your cart is empty</h2>
            <p className="text-muted-foreground mb-6">
              Add some products to get started
            </p>
            <Link to="/products">
              <Button size="lg">Browse Products</Button>
            </Link>
          </CardContent>
        </Card>
      </div>
    );
  }

  return (
    <div className="container mx-auto px-4 py-8">
      <h1 className="text-4xl font-bold mb-8">Shopping Cart</h1>

      <div className="grid lg:grid-cols-3 gap-8">
        <div className="lg:col-span-2 space-y-4">
          {cartItems.map((item) => (
            <Card key={item.id}>
              <CardContent className="p-6">
                <div className="flex items-center space-x-4">
                  <div className="w-24 h-24 bg-muted rounded-lg flex items-center justify-center flex-shrink-0">
                    <span className="text-xs text-muted-foreground">Image</span>
                  </div>

                  <div className="flex-grow">
                    <Link
                      to={'/products/' + item.product_id}
                      className="text-xl font-semibold hover:text-primary transition-colors"
                    >
                      {item.product_name || item.name}
                    </Link>
                    <p className="text-lg text-primary font-semibold mt-1">
                      ${item.price.toFixed(2)}
                    </p>
                  </div>

                  <div className="flex items-center space-x-2">
                    <Button
                      variant="outline"
                      size="icon"
                      onClick={() => updateQuantity(item.product_id, item.quantity - 1)}
                    >
                      <Minus className="h-4 w-4" />
                    </Button>
                    <span className="w-12 text-center font-medium">
                      {item.quantity}
                    </span>
                    <Button
                      variant="outline"
                      size="icon"
                      onClick={() => updateQuantity(item.product_id, item.quantity + 1)}
                    >
                      <Plus className="h-4 w-4" />
                    </Button>
                  </div>

                  <div className="text-right min-w-24">
                    <p className="text-xl font-bold">
                      ${(item.price * item.quantity).toFixed(2)}
                    </p>
                  </div>

                  <Button
                    variant="ghost"
                    size="icon"
                    onClick={() => removeItem(item.product_id)}
                    className="text-destructive hover:text-destructive"
                  >
                    <Trash2 className="h-5 w-5" />
                  </Button>
                </div>
              </CardContent>
            </Card>
          ))}
        </div>

        <div>
          <Card className="sticky top-4">
            <CardHeader>
              <CardTitle>Order Summary</CardTitle>
            </CardHeader>
            <CardContent className="space-y-4">
              <div className="flex justify-between">
                <span className="text-muted-foreground">Subtotal</span>
                <span className="font-semibold">${subtotal.toFixed(2)}</span>
              </div>
              <div className="flex justify-between">
                <span className="text-muted-foreground">Tax (8%)</span>
                <span className="font-semibold">${tax.toFixed(2)}</span>
              </div>
              <div className="border-t pt-4">
                <div className="flex justify-between items-center">
                  <span className="text-lg font-bold">Total</span>
                  <span className="text-2xl font-bold text-primary">
                    ${total.toFixed(2)}
                  </span>
                </div>
              </div>
              <Button className="w-full" size="lg">
                Proceed to Checkout
              </Button>
              <Link to="/products">
                <Button variant="outline" className="w-full">
                  Continue Shopping
                </Button>
              </Link>
            </CardContent>
          </Card>
        </div>
      </div>
    </div>
  );
}

export default Cart;
