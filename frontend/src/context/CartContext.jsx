import React, { createContext, useContext, useState, useEffect } from 'react';
import { cartApi } from '../services/api';

const CartContext = createContext();

export const useCart = () => {
  const context = useContext(CartContext);
  if (!context) {
    throw new Error('useCart must be used within a CartProvider');
  }
  return context;
};

export const CartProvider = ({ children }) => {
  const [cartItemCount, setCartItemCount] = useState(0);
  const [loading, setLoading] = useState(false);

  const fetchCartCount = async () => {
    try {
      setLoading(true);
      const response = await cartApi.get();
      const items = response.data.items || [];
      const totalCount = items.reduce((sum, item) => sum + item.quantity, 0);
      setCartItemCount(totalCount);
    } catch (err) {
      // If not authenticated or cart doesn't exist, set count to 0
      setCartItemCount(0);
      console.error('Error fetching cart count:', err);
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    // Only fetch cart if user is authenticated
    const token = localStorage.getItem('authToken');
    if (token) {
      fetchCartCount();
    }
  }, []);

  const value = {
    cartItemCount,
    loading,
    refreshCart: fetchCartCount,
  };

  return <CartContext.Provider value={value}>{children}</CartContext.Provider>;
};
