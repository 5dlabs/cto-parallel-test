// Cart configuration and helpers
// Following coding guidelines: no hardcoded thresholds; all values are parameterized via env/config

export interface CartItem {
  id: number;
  name: string;
  price: number;
  quantity: number;
  image: string;
}

// No mock data exports; cart state is managed client-side (e.g., localStorage or API)

// Shopping configuration
const parseNumber = (val: unknown, fallback: number): number => {
  const n = Number(val)
  return Number.isFinite(n) && n >= 0 ? n : fallback
}

// Prefer NEXT_PUBLIC_* for Next.js, but allow VITE_* to keep configuration consistent across apps
const FREE_SHIPPING_DEFAULT = 50
const SHIPPING_COST_DEFAULT = 9.99

export const shippingConfig = {
  freeShippingThreshold: parseNumber(
    process.env.NEXT_PUBLIC_FREE_SHIPPING_THRESHOLD ?? process.env.VITE_FREE_SHIPPING_THRESHOLD,
    FREE_SHIPPING_DEFAULT,
  ),
  standardShippingCost: parseNumber(
    process.env.NEXT_PUBLIC_STANDARD_SHIPPING_COST ?? process.env.VITE_STANDARD_SHIPPING_COST,
    SHIPPING_COST_DEFAULT,
  ),
}

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
