import React, { useState } from 'react'
import { Link } from 'react-router-dom'
import { ShoppingCart, Menu, X } from 'lucide-react'
import { Button } from './ui/button'
import { Badge } from './ui/badge'

const Header = () => {
  const [mobileMenuOpen, setMobileMenuOpen] = useState(false)
  const cartItemCount = 0 // This will be connected to cart state later

  return (
    <header className="sticky top-0 z-50 w-full border-b bg-background/95 backdrop-blur supports-[backdrop-filter]:bg-background/60">
      <div className="container mx-auto px-4">
        <div className="flex h-16 items-center justify-between">
          {/* Logo */}
          <Link to="/" className="flex items-center space-x-2">
            <span className="text-2xl font-bold text-primary">E-Shop</span>
          </Link>

          {/* Desktop Navigation */}
          <nav className="hidden md:flex items-center space-x-6">
            <Link to="/" className="text-sm font-medium transition-colors hover:text-primary">
              Home
            </Link>
            <Link to="/products" className="text-sm font-medium transition-colors hover:text-primary">
              Products
            </Link>
          </nav>

          {/* Right side actions */}
          <div className="flex items-center space-x-4">
            {/* Cart Button */}
            <Link to="/cart">
              <Button variant="ghost" size="icon" className="relative">
                <ShoppingCart className="h-5 w-5" />
                {cartItemCount > 0 && (
                  <Badge
                    className="absolute -top-1 -right-1 h-5 w-5 flex items-center justify-center p-0"
                    variant="destructive"
                  >
                    {cartItemCount}
                  </Badge>
                )}
              </Button>
            </Link>

            {/* Auth Buttons - Desktop */}
            <div className="hidden md:flex items-center space-x-2">
              <Link to="/login">
                <Button variant="ghost">Login</Button>
              </Link>
              <Link to="/register">
                <Button>Sign Up</Button>
              </Link>
            </div>

            {/* Mobile menu button */}
            <Button
              variant="ghost"
              size="icon"
              className="md:hidden"
              onClick={() => setMobileMenuOpen(!mobileMenuOpen)}
            >
              {mobileMenuOpen ? <X className="h-5 w-5" /> : <Menu className="h-5 w-5" />}
            </Button>
          </div>
        </div>

        {/* Mobile Navigation */}
        {mobileMenuOpen && (
          <div className="md:hidden py-4 space-y-4 border-t">
            <Link
              to="/"
              className="block py-2 text-sm font-medium transition-colors hover:text-primary"
              onClick={() => setMobileMenuOpen(false)}
            >
              Home
            </Link>
            <Link
              to="/products"
              className="block py-2 text-sm font-medium transition-colors hover:text-primary"
              onClick={() => setMobileMenuOpen(false)}
            >
              Products
            </Link>
            <div className="flex flex-col space-y-2 pt-4 border-t">
              <Link to="/login" onClick={() => setMobileMenuOpen(false)}>
                <Button variant="ghost" className="w-full">Login</Button>
              </Link>
              <Link to="/register" onClick={() => setMobileMenuOpen(false)}>
                <Button className="w-full">Sign Up</Button>
              </Link>
            </div>
          </div>
        )}
      </div>
    </header>
  )
}

export default Header
