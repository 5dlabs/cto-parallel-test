import React from 'react';
import { useParams, Link } from 'react-router-dom';

const products = [
  {
    id: 1,
    name: "Premium Wireless Headphones",
    price: 299.99,
    description: "High-quality sound with active noise cancellation",
    category: "Electronics",
    inStock: true,
    features: [
      "Active Noise Cancellation",
      "40-hour battery life",
      "Bluetooth 5.0",
      "Premium comfort fit",
      "Built-in microphone",
    ],
  },
  {
    id: 2,
    name: "Smart Fitness Watch",
    price: 199.99,
    description: "Track your fitness goals with style",
    category: "Wearables",
    inStock: true,
    features: [
      "Heart rate monitor",
      "GPS tracking",
      "Water resistant",
      "7-day battery life",
      "Sleep tracking",
    ],
  },
  {
    id: 3,
    name: "Laptop Stand",
    price: 49.99,
    description: "Ergonomic aluminum laptop stand",
    category: "Accessories",
    inStock: true,
    features: [
      "Aluminum construction",
      "Adjustable height",
      "Non-slip pads",
      "Compatible with all laptops",
      "Portable design",
    ],
  },
  {
    id: 4,
    name: "Mechanical Keyboard",
    price: 149.99,
    description: "RGB backlit mechanical gaming keyboard",
    category: "Electronics",
    inStock: false,
    features: [
      "Cherry MX switches",
      "RGB backlighting",
      "Programmable keys",
      "USB-C connection",
      "Durable construction",
    ],
  },
  {
    id: 5,
    name: "Wireless Mouse",
    price: 79.99,
    description: "Precision wireless mouse with ergonomic design",
    category: "Electronics",
    inStock: true,
    features: [
      "Ergonomic design",
      "High precision sensor",
      "Long battery life",
      "Multiple DPI settings",
      "Wireless connectivity",
    ],
  },
  {
    id: 6,
    name: "USB-C Hub",
    price: 59.99,
    description: "7-in-1 USB-C hub with HDMI and ethernet",
    category: "Accessories",
    inStock: true,
    features: [
      "7 ports in one",
      "4K HDMI output",
      "Gigabit ethernet",
      "USB 3.0 ports",
      "Compact design",
    ],
  },
];

function ProductDetail() {
  const { id } = useParams();
  const product = products.find((p) => p.id === parseInt(id));

  if (!product) {
    return (
      <div className="container mx-auto px-4 py-16 text-center">
        <h1 className="text-2xl font-bold mb-4">Product Not Found</h1>
        <Link to="/products" className="text-blue-600 hover:underline">
          Back to Products
        </Link>
      </div>
    );
  }

  return (
    <div className="container mx-auto px-4 py-8">
      <Link to="/products" className="inline-flex items-center text-gray-600 hover:text-gray-900 mb-6">
        <svg className="mr-2 h-4 w-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M15 19l-7-7 7-7" />
        </svg>
        Back to Products
      </Link>

      <div className="grid grid-cols-1 lg:grid-cols-2 gap-8 mt-6">
        {/* Product Image Placeholder */}
        <div className="aspect-square bg-gray-100 rounded-lg flex items-center justify-center">
          <div className="text-center text-gray-500">
            <svg className="h-24 w-24 mx-auto mb-4 opacity-20" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M16 11V7a4 4 0 00-8 0v4M5 9h14l1 12H4L5 9z" />
            </svg>
            <p>Product Image</p>
          </div>
        </div>

        {/* Product Details */}
        <div className="space-y-6">
          <div>
            <div className="flex items-center gap-2 mb-2">
              <span className="px-2 py-1 bg-gray-100 text-gray-700 text-sm rounded">
                {product.category}
              </span>
              {!product.inStock && (
                <span className="px-2 py-1 bg-red-100 text-red-700 text-sm rounded">
                  Out of Stock
                </span>
              )}
            </div>
            <h1 className="text-3xl md:text-4xl font-bold mb-2">{product.name}</h1>
            <p className="text-lg text-gray-600">{product.description}</p>
          </div>

          <div>
            <p className="text-3xl font-bold">${product.price}</p>
          </div>

          <div className="bg-white border rounded-lg p-6">
            <h3 className="text-xl font-semibold mb-4">Features</h3>
            <ul className="space-y-2">
              {product.features.map((feature, index) => (
                <li key={index} className="flex items-center">
                  <span className="mr-2">•</span>
                  {feature}
                </li>
              ))}
            </ul>
          </div>

          <div className="flex flex-col sm:flex-row gap-4">
            <button 
              className={`flex-1 px-6 py-3 rounded-lg flex items-center justify-center transition-colors ${
                product.inStock 
                  ? 'bg-blue-600 text-white hover:bg-blue-700' 
                  : 'bg-gray-200 text-gray-500 cursor-not-allowed'
              }`}
              disabled={!product.inStock}
            >
              <svg className="mr-2 h-5 w-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M16 11V7a4 4 0 00-8 0v4M5 9h14l1 12H4L5 9z" />
              </svg>
              {product.inStock ? "Add to Cart" : "Out of Stock"}
            </button>
            <button className="flex-1 px-6 py-3 border border-gray-300 rounded-lg hover:bg-gray-50 transition-colors">
              Add to Wishlist
            </button>
          </div>

          <div className="bg-white border rounded-lg p-6">
            <h3 className="text-xl font-semibold mb-4">Product Information</h3>
            <dl className="space-y-2">
              <div className="flex justify-between">
                <dt className="text-gray-600">Category:</dt>
                <dd className="font-medium">{product.category}</dd>
              </div>
              <div className="flex justify-between">
                <dt className="text-gray-600">Availability:</dt>
                <dd className="font-medium">
                  {product.inStock ? "In Stock" : "Out of Stock"}
                </dd>
              </div>
              <div className="flex justify-between">
                <dt className="text-gray-600">Shipping:</dt>
                <dd className="font-medium">Free shipping on orders over $50</dd>
              </div>
            </dl>
          </div>
        </div>
      </div>
    </div>
  );
}

export default ProductDetail;
