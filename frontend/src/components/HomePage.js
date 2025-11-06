import React from 'react';
import { Link } from 'react-router-dom';

function HomePage() {
  return (
    <div className="flex flex-col">
      {/* Hero Section */}
      <section className="bg-gradient-to-b from-slate-50 to-white dark:from-slate-950 dark:to-slate-900 py-12 md:py-20 lg:py-28">
        <div className="container mx-auto px-4">
          <div className="flex flex-col items-center text-center space-y-6">
            <h1 className="text-4xl md:text-5xl lg:text-6xl font-bold tracking-tight">
              Welcome to ShopHub
            </h1>
            <p className="text-lg md:text-xl text-gray-600 max-w-2xl">
              Discover amazing products at unbeatable prices. Shop from thousands of items
              across all categories.
            </p>
            <div className="flex flex-col sm:flex-row gap-4 mt-8">
              <Link to="/products">
                <button className="px-8 py-3 bg-blue-600 text-white rounded-lg hover:bg-blue-700 transition-colors">
                  Browse Products
                </button>
              </Link>
              <Link to="/register">
                <button className="px-8 py-3 border border-gray-300 rounded-lg hover:bg-gray-50 transition-colors">
                  Sign Up Now
                </button>
              </Link>
            </div>
          </div>
        </div>
      </section>

      {/* Features Section */}
      <section className="py-12 md:py-20">
        <div className="container mx-auto px-4">
          <h2 className="text-3xl font-bold text-center mb-12">Why Choose ShopHub?</h2>
          <div className="grid grid-cols-1 md:grid-cols-3 gap-6">
            <div className="bg-white border rounded-lg p-6 shadow-sm">
              <div className="flex justify-center mb-4">
                <svg className="h-12 w-12 text-blue-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M16 11V7a4 4 0 00-8 0v4M5 9h14l1 12H4L5 9z" />
                </svg>
              </div>
              <h3 className="text-xl font-semibold text-center mb-2">Wide Selection</h3>
              <p className="text-center text-gray-600">
                Browse thousands of products from top brands
              </p>
            </div>

            <div className="bg-white border rounded-lg p-6 shadow-sm">
              <div className="flex justify-center mb-4">
                <svg className="h-12 w-12 text-blue-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M13 7h8m0 0v8m0-8l-8 8-4-4-6 6" />
                </svg>
              </div>
              <h3 className="text-xl font-semibold text-center mb-2">Best Prices</h3>
              <p className="text-center text-gray-600">
                Get the best deals and exclusive discounts
              </p>
            </div>

            <div className="bg-white border rounded-lg p-6 shadow-sm">
              <div className="flex justify-center mb-4">
                <svg className="h-12 w-12 text-blue-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M9 12l2 2 4-4m5.618-4.016A11.955 11.955 0 0112 2.944a11.955 11.955 0 01-8.618 3.04A12.02 12.02 0 003 9c0 5.591 3.824 10.29 9 11.622 5.176-1.332 9-6.03 9-11.622 0-1.042-.133-2.052-.382-3.016z" />
                </svg>
              </div>
              <h3 className="text-xl font-semibold text-center mb-2">Secure Shopping</h3>
              <p className="text-center text-gray-600">
                Safe and secure payment methods
              </p>
            </div>
          </div>
        </div>
      </section>

      {/* CTA Section */}
      <section className="bg-gray-50 dark:bg-gray-900 py-12 md:py-20">
        <div className="container mx-auto px-4">
          <div className="flex flex-col items-center text-center space-y-6">
            <h2 className="text-3xl font-bold">Ready to Start Shopping?</h2>
            <p className="text-lg text-gray-600 max-w-2xl">
              Join thousands of satisfied customers and find your perfect products today.
            </p>
            <Link to="/products">
              <button className="px-8 py-3 bg-blue-600 text-white rounded-lg hover:bg-blue-700 transition-colors">
                Explore Products
              </button>
            </Link>
          </div>
        </div>
      </section>
    </div>
  );
}

export default HomePage;
