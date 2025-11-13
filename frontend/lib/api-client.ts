/**
 * API Client for backend communication
 * Supports both live API and mock data fallback
 */

const API_BASE_URL = process.env.NEXT_PUBLIC_API_URL || 'http://localhost:8080/api';
const USE_MOCK_DATA = process.env.NEXT_PUBLIC_USE_MOCK_DATA === 'true';

export interface Product {
  id: number;
  name: string;
  price: number;
  category: string;
  inStock: boolean;
  image: string;
  description?: string;
  rating?: number;
  reviews?: number;
}

export interface CartItem {
  id: number;
  name: string;
  price: number;
  quantity: number;
  image: string;
}

// Mock data - only used when USE_MOCK_DATA is true
const mockProducts: Product[] = [
  {
    id: 1,
    name: "Wireless Headphones",
    price: 99.99,
    category: "Electronics",
    inStock: true,
    image: "https://placehold.co/400x300/e2e8f0/475569?text=Headphones",
    description: "Premium wireless headphones with noise cancellation and 30-hour battery life. Experience superior sound quality and comfort.",
    rating: 4.5,
    reviews: 128,
  },
  {
    id: 2,
    name: "Smart Watch",
    price: 249.99,
    category: "Electronics",
    inStock: true,
    image: "https://placehold.co/400x300/e2e8f0/475569?text=Smart+Watch",
    description: "Feature-rich smartwatch with health tracking, GPS, and 5-day battery life. Stay connected and active.",
    rating: 4.8,
    reviews: 256,
  },
  {
    id: 3,
    name: "Laptop Backpack",
    price: 49.99,
    category: "Accessories",
    inStock: true,
    image: "https://placehold.co/400x300/e2e8f0/475569?text=Backpack",
    description: "Durable laptop backpack with multiple compartments and water-resistant material. Perfect for daily commute.",
    rating: 4.3,
    reviews: 89,
  },
  {
    id: 4,
    name: "Portable Charger",
    price: 29.99,
    category: "Electronics",
    inStock: false,
    image: "https://placehold.co/400x300/e2e8f0/475569?text=Charger",
    description: "High-capacity portable charger with fast charging support. Keep your devices powered on the go.",
    rating: 4.6,
    reviews: 432,
  },
  {
    id: 5,
    name: "Bluetooth Speaker",
    price: 79.99,
    category: "Electronics",
    inStock: true,
    image: "https://placehold.co/400x300/e2e8f0/475569?text=Speaker",
    description: "Portable Bluetooth speaker with 360-degree sound and waterproof design. Perfect for outdoor adventures.",
    rating: 4.7,
    reviews: 312,
  },
  {
    id: 6,
    name: "Phone Case",
    price: 19.99,
    category: "Accessories",
    inStock: true,
    image: "https://placehold.co/400x300/e2e8f0/475569?text=Phone+Case",
    description: "Slim protective phone case with military-grade drop protection. Available in multiple colors.",
    rating: 4.4,
    reviews: 567,
  },
];

const mockCartItems: CartItem[] = [
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

/**
 * Fetch all products
 * @returns Promise resolving to array of products
 */
export async function getProducts(): Promise<Product[]> {
  if (USE_MOCK_DATA) {
    // Simulate network delay
    await new Promise(resolve => setTimeout(resolve, 100));
    return mockProducts;
  }

  try {
    const response = await fetch(`${API_BASE_URL}/products`);
    if (!response.ok) {
      throw new Error(`HTTP error! status: ${response.status}`);
    }
    return await response.json();
  } catch (error) {
    console.error('Failed to fetch products:', error);
    // Fallback to mock data on error
    return mockProducts;
  }
}

/**
 * Fetch a single product by ID
 * @param id - Product ID
 * @returns Promise resolving to product or null if not found
 */
export async function getProductById(id: number): Promise<Product | null> {
  if (USE_MOCK_DATA) {
    await new Promise(resolve => setTimeout(resolve, 100));
    return mockProducts.find(p => p.id === id) || null;
  }

  try {
    const response = await fetch(`${API_BASE_URL}/products/${id}`);
    if (!response.ok) {
      if (response.status === 404) return null;
      throw new Error(`HTTP error! status: ${response.status}`);
    }
    return await response.json();
  } catch (error) {
    console.error(`Failed to fetch product ${id}:`, error);
    return mockProducts.find(p => p.id === id) || null;
  }
}

/**
 * Fetch cart items for the current user
 * @returns Promise resolving to array of cart items
 */
export async function getCartItems(): Promise<CartItem[]> {
  if (USE_MOCK_DATA) {
    await new Promise(resolve => setTimeout(resolve, 100));
    return mockCartItems;
  }

  try {
    const response = await fetch(`${API_BASE_URL}/cart`, {
      credentials: 'include', // Include cookies for authentication
    });
    if (!response.ok) {
      throw new Error(`HTTP error! status: ${response.status}`);
    }
    return await response.json();
  } catch (error) {
    console.error('Failed to fetch cart:', error);
    return mockCartItems;
  }
}

/**
 * Add an item to the cart
 * @param productId - Product ID to add
 * @param quantity - Quantity to add (default: 1)
 * @returns Promise resolving to updated cart items
 */
export async function addToCart(productId: number, quantity: number = 1): Promise<CartItem[]> {
  if (USE_MOCK_DATA) {
    await new Promise(resolve => setTimeout(resolve, 100));
    console.log(`Mock: Added product ${productId} (qty: ${quantity}) to cart`);
    return mockCartItems;
  }

  try {
    const response = await fetch(`${API_BASE_URL}/cart/add`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      credentials: 'include',
      body: JSON.stringify({ product_id: productId, quantity }),
    });
    if (!response.ok) {
      throw new Error(`HTTP error! status: ${response.status}`);
    }
    return await response.json();
  } catch (error) {
    console.error('Failed to add to cart:', error);
    throw error;
  }
}

/**
 * Remove an item from the cart
 * @param productId - Product ID to remove
 * @returns Promise resolving to updated cart items
 */
export async function removeFromCart(productId: number): Promise<CartItem[]> {
  if (USE_MOCK_DATA) {
    await new Promise(resolve => setTimeout(resolve, 100));
    console.log(`Mock: Removed product ${productId} from cart`);
    return mockCartItems.filter(item => item.id !== productId);
  }

  try {
    const response = await fetch(`${API_BASE_URL}/cart/remove/${productId}`, {
      method: 'DELETE',
      credentials: 'include',
    });
    if (!response.ok) {
      throw new Error(`HTTP error! status: ${response.status}`);
    }
    return await response.json();
  } catch (error) {
    console.error('Failed to remove from cart:', error);
    throw error;
  }
}
