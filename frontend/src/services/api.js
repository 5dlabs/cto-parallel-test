import axios from 'axios';

const API_BASE_URL = process.env.REACT_APP_API_URL || 'http://localhost:8080/api';

const api = axios.create({
  baseURL: API_BASE_URL,
  headers: {
    'Content-Type': 'application/json',
  },
});

// Add JWT token to requests if available
api.interceptors.request.use((config) => {
  const token = localStorage.getItem('token');
  if (token) {
    config.headers.Authorization = `Bearer ${token}`;
  }
  return config;
});

// Product API
export const productAPI = {
  getAll: () => api.get('/products'),
  getById: (id) => api.get(`/products/${id}`),
  search: (params) => api.get('/products/search', { params }),
};

// Cart API
export const cartAPI = {
  getCart: () => api.get('/cart'),
  addItem: (productId, quantity) => api.post('/cart/add', { product_id: productId, quantity }),
  removeItem: (productId) => api.delete(`/cart/remove/${productId}`),
  clearCart: () => api.post('/cart/clear'),
};

// Auth API
export const authAPI = {
  login: (username, password) => api.post('/auth/login', { username, password }),
  register: (userData) => api.post('/auth/register', userData),
  logout: () => {
    localStorage.removeItem('token');
    localStorage.removeItem('user');
  },
};

export default api;
