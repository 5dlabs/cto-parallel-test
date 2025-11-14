// Cart data configuration
// This file contains mock cart data that will be replaced with state management later
// Following coding guidelines: no hardcoded data in components, externalized configuration

export interface CartItem {
  id: number;
  name: string;
  price: number;
  quantity: number;
  image: string;
}

// No mock data exports; cart state is managed client-side (e.g., localStorage or API)

// Shopping configuration
export const shippingConfig = {
  freeShippingThreshold: 50,
  standardShippingCost: 9.99,
};

// Helper functions for cart calculations
export function calculateSubtotal(items: CartItem[]): number {
  return items.reduce((sum, item) => sum + item.price * item.quantity, 0);
}

export function calculateShipping(subtotal: number): number {
  return subtotal >= shippingConfig.freeShippingThreshold
    ? 0
    : shippingConfig.standardShippingCost;
}

export function calculateTotal(items: CartItem[]): number {
  const subtotal = calculateSubtotal(items);
  const shipping = calculateShipping(subtotal);
  return subtotal + shipping;
}
