/**
 * API Client Configuration
 * Provides centralized API communication with environment-based configuration
 */

// Use Vite env for API base; default to relative path for secure proxying
const API_BASE_URL = (import.meta as any).env?.VITE_API_BASE_URL || '/api';

export interface Product {
  id: number;
  name: string;
  price: number;
  category: string;
  description: string;
  inStock: boolean;
  details?: string;
  specs?: string[];
  inventory_count?: number;
}

export interface CartItem {
  id: number;
  productId: number;
  name: string;
  price: number;
  quantity: number;
}

export interface Cart {
  id: number;
  userId: number;
  items: CartItem[];
  createdAt: string;
}

export interface LoginRequest {
  username: string;
  password: string;
}

export interface RegisterRequest {
  username: string;
  email: string;
  password: string;
}

export interface AuthResponse {
  token: string;
  userId: number;
  username: string;
}

/**
 * Generic API request handler with error handling
 */
async function apiRequest<T>(endpoint: string, options: RequestInit = {}): Promise<T> {
  const headers: Record<string, string> = {
    'Content-Type': 'application/json',
    ...(options.headers as Record<string, string>),
  };

  // Use cookie-based auth; never persist tokens in localStorage
  const response = await fetch(`${API_BASE_URL}${endpoint}`, {
    credentials: 'include',
    ...options,
    headers,
  });

  if (!response.ok) {
    const error = await response.text();
    throw new Error(error || `API request failed: ${response.statusText}`);
  }

  return response.json();
}

/**
 * Product API
 */
export const productsApi = {
  /**
   * Get all products from the API
   */
  async getAll(): Promise<Product[]> {
    return apiRequest<Product[]>('/products');
  },

  /**
   * Get a single product by ID
   */
  async getById(id: number): Promise<Product> {
    return apiRequest<Product>(`/products/${id}`);
  },

  /**
   * Search/filter products
   */
  async search(params?: { category?: string; inStock?: boolean }): Promise<Product[]> {
    const queryString = params
      ? '?' + new URLSearchParams(params as Record<string, string>).toString()
      : '';
    return apiRequest<Product[]>(`/products/search${queryString}`);
  },
};

/**
 * Cart API
 */
export const cartApi = {
  /**
   * Get current user's cart
   */
  async get(): Promise<Cart> {
    return apiRequest<Cart>('/cart');
  },

  /**
   * Add item to cart
   */
  async addItem(productId: number, quantity: number): Promise<Cart> {
    return apiRequest<Cart>('/cart/add', {
      method: 'POST',
      body: JSON.stringify({ product_id: productId, quantity }),
    });
  },

  /**
   * Remove item from cart
   */
  async removeItem(productId: number): Promise<Cart> {
    return apiRequest<Cart>(`/cart/remove/${productId}`, {
      method: 'DELETE',
    });
  },

  /**
   * Clear entire cart
   */
  async clear(): Promise<void> {
    return apiRequest<void>('/cart/clear', {
      method: 'POST',
    });
  },
};

/**
 * Authentication API
 */
export const authApi = {
  /**
   * Login user
   */
  async login(credentials: LoginRequest): Promise<AuthResponse> {
    const response = await apiRequest<AuthResponse>('/auth/login', {
      method: 'POST',
      body: JSON.stringify(credentials),
    });

    // Server should set httpOnly cookie; do not store tokens client-side
    return response;
  },

  /**
   * Register new user
   */
  async register(userData: RegisterRequest): Promise<AuthResponse> {
    const response = await apiRequest<AuthResponse>('/auth/register', {
      method: 'POST',
      body: JSON.stringify(userData),
    });

    // Server should set httpOnly cookie; do not store tokens client-side
    return response;
  },

  /**
   * Logout user
   */
  logout(): void {
    // Server should clear httpOnly cookie; nothing to purge client-side
  },

  /**
   * Check if user is authenticated
   */
  isAuthenticated(): boolean {
    // Without client-side tokens, authenticated state is derived server-side
    return false;
  },

  /**
   * Get current user info from localStorage
   */
  getCurrentUser(): { userId: number; username: string } | null {
    // Fetch from API when needed; no client-side persistence of secrets
    return null;
  },
};

/**
 * Health check
 */
export const healthApi = {
  async check(): Promise<{ status: string }> {
    return apiRequest<{ status: string }>('/health');
  },
};
