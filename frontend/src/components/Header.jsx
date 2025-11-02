import { Link } from 'react-router-dom'
import { Button } from './ui/button'
import { Badge } from './ui/badge'

function Header() {
  // In a real app, this would come from state/context
  const cartItemCount = 0

  return (
    <header className="border-b">
      <div className="container mx-auto px-4 py-4">
        <div className="flex items-center justify-between">
          <Link to="/" className="text-2xl font-bold">
            E-Commerce
          </Link>

          <nav className="hidden md:flex items-center space-x-6">
            <Link to="/" className="text-sm font-medium hover:text-primary transition-colors">
              Home
            </Link>
            <Link to="/products" className="text-sm font-medium hover:text-primary transition-colors">
              Products
            </Link>
          </nav>

          <div className="flex items-center space-x-4">
            <Link to="/cart" className="relative">
              <Button variant="outline" size="sm">
                Cart
                {cartItemCount > 0 && (
                  <Badge className="ml-2" variant="destructive">
                    {cartItemCount}
                  </Badge>
                )}
              </Button>
            </Link>

            <Link to="/login">
              <Button variant="ghost" size="sm">
                Login
              </Button>
            </Link>

            <Link to="/register">
              <Button size="sm">
                Sign Up
              </Button>
            </Link>
          </div>
        </div>
      </div>
    </header>
  )
}

export default Header
