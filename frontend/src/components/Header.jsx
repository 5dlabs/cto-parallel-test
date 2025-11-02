import React from 'react'
import { Link } from 'react-router-dom'
import { ShoppingCart, User, Menu, X } from 'lucide-react'
import { Button } from './ui/button'
import { Badge } from './ui/badge'

function Header() {
  const [mobileMenuOpen, setMobileMenuOpen] = React.useState(false)
  const [cartItemCount] = React.useState(0) // This will be connected to cart state later

  const toggleMobileMenu = () => {
    setMobileMenuOpen(!mobileMenuOpen)
  }

  return (
    <header className="sticky top-0 z-50 w-full border-b bg-background/95 backdrop-blur supports-[backdrop-filter]:bg-background/60">
      <div className="container mx-auto px-4">
        <div className="flex h-16 items-center justify-between">
          {/* Logo */}
          <Link to="/" className="flex items-center space-x-2">
            <span className="text-2xl font-bold">ShopHub</span>
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

          {/* Actions */}
          <div className="flex items-center space-x-4">
            <Link to="/cart" className="relative">
              <Button variant="ghost" size="icon">
                <ShoppingCart className="h-5 w-5" />
                {cartItemCount > 0 && (
                  <Badge
                    variant="destructive"
                    className="absolute -top-1 -right-1 h-5 w-5 flex items-center justify-center p-0 text-xs"
                  >
                    {cartItemCount}
                  </Badge>
                )}
              </Button>
            </Link>

            <Link to="/login" className="hidden md:inline-flex">
              <Button variant="ghost" size="icon">
                <User className="h-5 w-5" />
              </Button>
            </Link>

            <Link to="/login" className="hidden md:inline-flex">
              <Button>Sign In</Button>
            </Link>

            {/* Mobile Menu Button */}
            <Button
              variant="ghost"
              size="icon"
              className="md:hidden"
              onClick={toggleMobileMenu}
            >
              {mobileMenuOpen ? <X className="h-5 w-5" /> : <Menu className="h-5 w-5" />}
            </Button>
          </div>
        </div>

        {/* Mobile Menu */}
        {mobileMenuOpen && (
          <div className="md:hidden border-t py-4">
            <nav className="flex flex-col space-y-3">
              <Link
                to="/"
                className="text-sm font-medium transition-colors hover:text-primary"
                onClick={toggleMobileMenu}
              >
                Home
              </Link>
              <Link
                to="/products"
                className="text-sm font-medium transition-colors hover:text-primary"
                onClick={toggleMobileMenu}
              >
                Products
              </Link>
              <Link
                to="/login"
                className="text-sm font-medium transition-colors hover:text-primary"
                onClick={toggleMobileMenu}
              >
                Sign In
              </Link>
              <Link
                to="/register"
                className="text-sm font-medium transition-colors hover:text-primary"
                onClick={toggleMobileMenu}
              >
                Register
              </Link>
            </nav>
          </div>
        )}
      </div>
    </header>
  )
}

export default Header
