import React from 'react'
import { Link } from 'react-router-dom'

function Footer() {
  const currentYear = new Date().getFullYear()

  return (
    <footer className="border-t bg-background">
      <div className="container mx-auto px-4 py-8">
        <div className="grid grid-cols-1 md:grid-cols-3 gap-8">
          {/* About */}
          <div>
            <h3 className="font-semibold text-lg mb-4">ShopHub</h3>
            <p className="text-sm text-muted-foreground">
              Your one-stop shop for quality products at great prices.
            </p>
          </div>

          {/* Quick Links */}
          <div>
            <h3 className="font-semibold text-lg mb-4">Quick Links</h3>
            <nav className="flex flex-col space-y-2">
              <Link to="/" className="text-sm text-muted-foreground hover:text-primary">
                Home
              </Link>
              <Link to="/products" className="text-sm text-muted-foreground hover:text-primary">
                Products
              </Link>
              <Link to="/cart" className="text-sm text-muted-foreground hover:text-primary">
                Cart
              </Link>
            </nav>
          </div>

          {/* Account */}
          <div>
            <h3 className="font-semibold text-lg mb-4">Account</h3>
            <nav className="flex flex-col space-y-2">
              <Link to="/login" className="text-sm text-muted-foreground hover:text-primary">
                Sign In
              </Link>
              <Link to="/register" className="text-sm text-muted-foreground hover:text-primary">
                Register
              </Link>
            </nav>
          </div>
        </div>

        <div className="mt-8 pt-8 border-t text-center text-sm text-muted-foreground">
          <p>&copy; {currentYear} ShopHub. All rights reserved.</p>
        </div>
      </div>
    </footer>
  )
}

export default Footer
