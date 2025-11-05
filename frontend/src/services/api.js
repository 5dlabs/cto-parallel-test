import axios from 'axios';

// Read API base URL from environment variable or use default
const API_BASE_URL = process.env.REACT_APP_API_BASE_URL || 'http://localhost:8080/api';
const API_TIMEOUT = parseInt(process.env.REACT_APP_API_TIMEOUT || '5000', 10);

// Create axios instance with configuration
const apiClient = axios.create({
  baseURL: API_BASE_URL,
  timeout: API_TIMEOUT,
  headers: {
    'Content-Type': 'application/json',
  },
});

// Request interceptor to add auth token
apiClient.interceptors.request.use(
  (config) => {
    const token = localStorage.getItem('authToken');
    if (token) {
      config.headers.Authorization = `Bearer ${token}`;
    }
    return config;
  },
  (error) => Promise.reject(error)
);

// Response interceptor for error handling
apiClient.interceptors.response.use(
  (response) => response,
  (error) => {
    if (error.response?.status === 401) {
      // Clear auth token on unauthorized
      localStorage.removeItem('authToken');
      localStorage.removeItem('user');
      window.location.href = '/login';
    }
    return Promise.reject(error);
  }
);

// Products API
export const productsApi = {
  getAll: () => apiClient.get('/products'),
  getById: (id) => apiClient.get(`/products/${id}`),
  search: (params) => apiClient.get('/products/search', { params }),
};

// Cart API
export const cartApi = {
  get: () => apiClient.get('/cart'),
  addItem: (productId, quantity) =>
    apiClient.post('/cart/add', { product_id: productId, quantity }),
  removeItem: (productId) =>
    apiClient.delete(`/cart/remove/${productId}`),
  updateQuantity: (productId, quantity) =>
    apiClient.put(`/cart/update`, { product_id: productId, quantity }),
  clear: () => apiClient.post('/cart/clear'),
};

// Auth API
export const authApi = {
  register: (userData) => apiClient.post('/auth/register', userData),
  login: (credentials) => apiClient.post('/auth/login', credentials),
  logout: () => {
    localStorage.removeItem('authToken');
    localStorage.removeItem('user');
    return Promise.resolve();
  },
};

export default apiClient;
