import React from 'react';
import { Link } from 'react-router-dom';

const products = [
  {
    id: 1,
    name: "Premium Wireless Headphones",
    price: 299.99,
    description: "High-quality sound with active noise cancellation",
    category: "Electronics",
    inStock: true,
  },
  {
    id: 2,
    name: "Smart Fitness Watch",
    price: 199.99,
    description: "Track your fitness goals with style",
    category: "Wearables",
    inStock: true,
  },
  {
    id: 3,
    name: "Laptop Stand",
    price: 49.99,
    description: "Ergonomic aluminum laptop stand",
    category: "Accessories",
    inStock: true,
  },
  {
    id: 4,
    name: "Mechanical Keyboard",
    price: 149.99,
    description: "RGB backlit mechanical gaming keyboard",
    category: "Electronics",
    inStock: false,
  },
  {
    id: 5,
    name: "Wireless Mouse",
    price: 79.99,
    description: "Precision wireless mouse with ergonomic design",
    category: "Electronics",
    inStock: true,
  },
  {
    id: 6,
    name: "USB-C Hub",
    price: 59.99,
    description: "7-in-1 USB-C hub with HDMI and ethernet",
    category: "Accessories",
    inStock: true,
  },
];

function ProductList() {
  return (
    <div className="container mx-auto px-4 py-8">
      <div className="mb-8">
        <h1 className="text-3xl md:text-4xl font-bold mb-2">All Products</h1>
        <p className="text-gray-600">
          Discover our full range of products
        </p>
      </div>

      <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-6">
        {products.map((product) => (
          <div key={product.id} className="bg-white border rounded-lg shadow-sm flex flex-col">
            <div className="p-6">
              <div className="flex items-start justify-between mb-4">
                <span className="px-2 py-1 bg-gray-100 text-gray-700 text-sm rounded">
                  {product.category}
                </span>
                {!product.inStock && (
                  <span className="px-2 py-1 bg-red-100 text-red-700 text-sm rounded">
                    Out of Stock
                  </span>
                )}
              </div>
              <h3 className="text-xl font-semibold mb-2">{product.name}</h3>
              <p className="text-gray-600 mb-4">{product.description}</p>
              <p className="text-2xl font-bold">${product.price}</p>
            </div>
            <div className="p-6 pt-0 mt-auto">
              <div className="flex flex-col gap-2">
                <Link to={`/products/${product.id}`} className="w-full">
                  <button className="w-full px-4 py-2 border border-gray-300 rounded-lg hover:bg-gray-50 transition-colors">
                    View Details
                  </button>
                </Link>
                <button 
                  className={`w-full px-4 py-2 rounded-lg transition-colors ${
                    product.inStock 
                      ? 'bg-blue-600 text-white hover:bg-blue-700' 
                      : 'bg-gray-200 text-gray-500 cursor-not-allowed'
                  }`}
                  disabled={!product.inStock}
                >
                  {product.inStock ? "Add to Cart" : "Out of Stock"}
                </button>
              </div>
            </div>
          </div>
        ))}
      </div>
    </div>
  );
}

export default ProductList;
