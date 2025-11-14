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

export const mockCartItems: CartItem[] = [
  {
    id: 1,
    name: "Wireless Headphones",
    price: 99.99,
    quantity: 1,
    image: "https://placehold.co/200x150/e2e8f0/475569?text=Headphones",
  },
  {
    id: 2,
    name: "Smart Watch",
    price: 249.99,
    quantity: 1,
    image: "https://placehold.co/200x150/e2e8f0/475569?text=Smart+Watch",
  },
  {
    id: 5,
    name: "Bluetooth Speaker",
    price: 79.99,
    quantity: 1,
    image: "https://placehold.co/200x150/e2e8f0/475569?text=Speaker",
  },
];

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
