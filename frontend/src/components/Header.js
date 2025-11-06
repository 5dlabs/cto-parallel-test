import React from 'react';
import { Link } from 'react-router-dom';

function Header() {
  const cartItemCount = 2; // This will be connected to state management later

  return (
    <header className="sticky top-0 z-50 w-full border-b bg-white">
      <div className="container mx-auto flex h-16 items-center justify-between px-4">
        {/* Logo */}
        <Link to="/" className="flex items-center space-x-2">
          <span className="text-2xl font-bold">ShopHub</span>
        </Link>

        {/* Navigation */}
        <nav className="hidden md:flex items-center space-x-6 text-sm font-medium">
          <Link
            to="/"
            className="transition-colors hover:text-gray-600"
          >
            Home
          </Link>
          <Link
            to="/products"
            className="transition-colors hover:text-gray-600"
          >
            Products
          </Link>
        </nav>

        {/* Actions */}
        <div className="flex items-center space-x-2">
          <Link to="/cart">
            <button className="relative p-2 hover:bg-gray-100 rounded-lg transition-colors" aria-label="Shopping cart">
              <svg className="h-5 w-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M16 11V7a4 4 0 00-8 0v4M5 9h14l1 12H4L5 9z" />
              </svg>
              {cartItemCount > 0 && (
                <span className="absolute -top-1 -right-1 h-5 w-5 flex items-center justify-center bg-red-500 text-white text-xs rounded-full">
                  {cartItemCount}
                </span>
              )}
            </button>
          </Link>
          <Link to="/login">
            <button className="p-2 hover:bg-gray-100 rounded-lg transition-colors" aria-label="User account">
              <svg className="h-5 w-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M16 7a4 4 0 11-8 0 4 4 0 018 0zM12 14a7 7 0 00-7 7h14a7 7 0 00-7-7z" />
              </svg>
            </button>
          </Link>
        </div>
      </div>

      {/* Mobile Navigation */}
      <nav className="md:hidden border-t px-4 py-2">
        <div className="flex items-center justify-around text-sm">
          <Link
            to="/"
            className="transition-colors hover:text-gray-600 py-2"
          >
            Home
          </Link>
          <Link
            to="/products"
            className="transition-colors hover:text-gray-600 py-2"
          >
            Products
          </Link>
        </div>
      </nav>
    </header>
  );
}

export default Header;
