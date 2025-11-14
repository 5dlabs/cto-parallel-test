import { Link, NavLink } from 'react-router-dom'
import { ShoppingCart } from 'lucide-react'
import { Badge } from '@/components/ui/badge'
import { NavigationMenu, NavigationMenuList, NavigationMenuItem, NavigationMenuLink } from '@/components/ui/navigation-menu'
import { useCart } from '@/context/CartContext'

export default function Header() {
  const { count } = useCart()

  return (
    <header className="border-b bg-background sticky top-0 z-40">
      <div className="container mx-auto px-4 py-3 flex items-center justify-between">
        <Link to="/" className="text-xl font-bold">Cipher Shop</Link>

        <NavigationMenu>
          <NavigationMenuList>
            <NavigationMenuItem>
              <NavigationMenuLink asChild>
                <NavLink to="/" className={({isActive}) => `px-3 py-2 rounded-md ${isActive ? 'bg-accent' : ''}`}>Home</NavLink>
              </NavigationMenuLink>
            </NavigationMenuItem>
            <NavigationMenuItem>
              <NavigationMenuLink asChild>
                <NavLink to="/products" className={({isActive}) => `px-3 py-2 rounded-md ${isActive ? 'bg-accent' : ''}`}>Products</NavLink>
              </NavigationMenuLink>
            </NavigationMenuItem>
          </NavigationMenuList>
        </NavigationMenu>

        <Link to="/cart" aria-label="Cart" className="relative inline-flex items-center">
          <ShoppingCart className="h-6 w-6" />
          <Badge className="absolute -top-2 -right-2" variant="secondary">{count}</Badge>
        </Link>
      </div>
    </header>
  )
}

