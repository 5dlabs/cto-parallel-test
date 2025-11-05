import axios from 'axios';

// Get API URL from environment variable with fallback
const API_BASE_URL = process.env.REACT_APP_API_URL || 'http://localhost:8080/api';

// Create axios instance with base configuration
const api = axios.create({
  baseURL: API_BASE_URL,
  timeout: 10000,
  headers: {
    'Content-Type': 'application/json',
  },
});

// Request interceptor to add auth token if available
api.interceptors.request.use(
  (config) => {
    const token = localStorage.getItem('auth_token');
    if (token) {
      config.headers.Authorization = `Bearer ${token}`;
    }
    return config;
  },
  (error) => {
    return Promise.reject(error);
  }
);

// Response interceptor for error handling
api.interceptors.response.use(
  (response) => response,
  (error) => {
    if (error.response?.status === 401) {
      // Clear invalid token and redirect to login
      localStorage.removeItem('auth_token');
      localStorage.removeItem('user');
      window.location.href = '/login';
    }
    return Promise.reject(error);
  }
);

// Product API endpoints
export const productAPI = {
  getAll: () => api.get('/products'),
  getById: (id) => api.get(`/products/${id}`),
  search: (params) => api.get('/products/search', { params }),
};

// Cart API endpoints
export const cartAPI = {
  get: () => api.get('/cart'),
  addItem: (productId, quantity) => api.post('/cart/add', { product_id: productId, quantity }),
  removeItem: (productId) => api.delete(`/cart/remove/${productId}`),
  clear: () => api.post('/cart/clear'),
};

// Auth API endpoints
export const authAPI = {
  register: (username, email, password) =>
    api.post('/auth/register', { username, email, password }),
  login: (username, password) =>
    api.post('/auth/login', { username, password }),
  logout: () => {
    localStorage.removeItem('auth_token');
    localStorage.removeItem('user');
    return Promise.resolve();
  },
};

export default api;
