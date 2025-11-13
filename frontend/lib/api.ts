/**
 * API Client Configuration
 * Provides centralized API communication with environment-based configuration
 */

const API_BASE_URL = process.env.NEXT_PUBLIC_API_URL || 'http://localhost:8080/api';

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
async function apiRequest<T>(
  endpoint: string,
  options: RequestInit = {}
): Promise<T> {
  const token = typeof window !== 'undefined' ? localStorage.getItem('auth_token') : null;

  const headers: HeadersInit = {
    'Content-Type': 'application/json',
    ...options.headers,
  };

  if (token) {
    headers['Authorization'] = `Bearer ${token}`;
  }

  const response = await fetch(`${API_BASE_URL}${endpoint}`, {
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

    // Store token in localStorage
    if (typeof window !== 'undefined') {
      localStorage.setItem('auth_token', response.token);
      localStorage.setItem('user_id', response.userId.toString());
      localStorage.setItem('username', response.username);
    }

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

    // Store token in localStorage
    if (typeof window !== 'undefined') {
      localStorage.setItem('auth_token', response.token);
      localStorage.setItem('user_id', response.userId.toString());
      localStorage.setItem('username', response.username);
    }

    return response;
  },

  /**
   * Logout user
   */
  logout(): void {
    if (typeof window !== 'undefined') {
      localStorage.removeItem('auth_token');
      localStorage.removeItem('user_id');
      localStorage.removeItem('username');
    }
  },

  /**
   * Check if user is authenticated
   */
  isAuthenticated(): boolean {
    if (typeof window === 'undefined') return false;
    return !!localStorage.getItem('auth_token');
  },

  /**
   * Get current user info from localStorage
   */
  getCurrentUser(): { userId: number; username: string } | null {
    if (typeof window === 'undefined') return null;
    const userId = localStorage.getItem('user_id');
    const username = localStorage.getItem('username');
    if (!userId || !username) return null;
    return { userId: parseInt(userId), username };
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
