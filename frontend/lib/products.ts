// Product data configuration
// This file contains mock product data that will be replaced with API calls later
// Following coding guidelines: no hardcoded data in components, externalized configuration

export interface Product {
  id: number;
  name: string;
  price: number;
  category: string;
  inStock: boolean;
  image: string;
  description: string;
  rating: number;
  reviews: number;
}

export const products: Product[] = [
  {
    id: 1,
    name: "Wireless Headphones",
    price: 99.99,
    category: "Electronics",
    inStock: true,
    image: "https://placehold.co/800x600/e2e8f0/475569?text=Headphones",
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
    image: "https://placehold.co/800x600/e2e8f0/475569?text=Smart+Watch",
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
    image: "https://placehold.co/800x600/e2e8f0/475569?text=Backpack",
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
    image: "https://placehold.co/800x600/e2e8f0/475569?text=Charger",
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
    image: "https://placehold.co/800x600/e2e8f0/475569?text=Speaker",
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
    image: "https://placehold.co/800x600/e2e8f0/475569?text=Phone+Case",
    description: "Slim protective phone case with military-grade drop protection. Available in multiple colors.",
    rating: 4.4,
    reviews: 567,
  },
];

// Helper function to get product by ID
export function getProductById(id: number): Product | undefined {
  return products.find((product) => product.id === id);
}

// Helper function to get all products
export function getAllProducts(): Product[] {
  return products;
}

// Helper function to get products by category
export function getProductsByCategory(category: string): Product[] {
  return products.filter((product) => product.category === category);
}
