import React, { useState } from 'react';
import { Link } from 'react-router-dom';
import { ShoppingCart, Menu, X, User } from 'lucide-react';
import { Button } from './ui/button';
import { Badge } from './ui/badge';

const Header = () => {
  const [mobileMenuOpen, setMobileMenuOpen] = useState(false);
  const [cartItemCount] = useState(0); // This would be connected to actual cart state

  const toggleMobileMenu = () => {
    setMobileMenuOpen(!mobileMenuOpen);
  };

  return (
    <header className="bg-white border-b border-gray-200 sticky top-0 z-50 shadow-sm">
      <div className="container mx-auto px-4">
        <div className="flex items-center justify-between h-16">
          {/* Logo */}
          <Link to="/" className="flex items-center space-x-2">
            <div className="text-2xl font-bold text-primary">
              E-Shop
            </div>
          </Link>

          {/* Desktop Navigation */}
          <nav className="hidden md:flex items-center space-x-8">
            <Link
              to="/"
              className="text-sm font-medium text-gray-700 hover:text-primary transition-colors"
            >
              Home
            </Link>
            <Link
              to="/products"
              className="text-sm font-medium text-gray-700 hover:text-primary transition-colors"
            >
              Products
            </Link>
          </nav>

          {/* Right side actions */}
          <div className="hidden md:flex items-center space-x-4">
            <Link to="/login">
              <Button variant="ghost" size="sm" className="gap-2">
                <User className="h-4 w-4" />
                Login
              </Button>
            </Link>
            <Link to="/cart" className="relative">
              <Button variant="outline" size="icon">
                <ShoppingCart className="h-5 w-5" />
                {cartItemCount > 0 && (
                  <Badge
                    variant="destructive"
                    className="absolute -top-2 -right-2 h-5 w-5 flex items-center justify-center p-0 text-xs"
                  >
                    {cartItemCount}
                  </Badge>
                )}
              </Button>
            </Link>
          </div>

          {/* Mobile menu button */}
          <button
            onClick={toggleMobileMenu}
            className="md:hidden p-2 rounded-md text-gray-700 hover:bg-gray-100"
            aria-label="Toggle menu"
          >
            {mobileMenuOpen ? (
              <X className="h-6 w-6" />
            ) : (
              <Menu className="h-6 w-6" />
            )}
          </button>
        </div>

        {/* Mobile Navigation */}
        {mobileMenuOpen && (
          <div className="md:hidden py-4 border-t border-gray-200">
            <nav className="flex flex-col space-y-4">
              <Link
                to="/"
                className="text-sm font-medium text-gray-700 hover:text-primary transition-colors px-2 py-1"
                onClick={toggleMobileMenu}
              >
                Home
              </Link>
              <Link
                to="/products"
                className="text-sm font-medium text-gray-700 hover:text-primary transition-colors px-2 py-1"
                onClick={toggleMobileMenu}
              >
                Products
              </Link>
              <Link
                to="/cart"
                className="text-sm font-medium text-gray-700 hover:text-primary transition-colors px-2 py-1"
                onClick={toggleMobileMenu}
              >
                Cart {cartItemCount > 0 && `(${cartItemCount})`}
              </Link>
              <Link
                to="/login"
                className="text-sm font-medium text-gray-700 hover:text-primary transition-colors px-2 py-1"
                onClick={toggleMobileMenu}
              >
                Login
              </Link>
            </nav>
          </div>
        )}
      </div>
    </header>
  );
};

export default Header;
