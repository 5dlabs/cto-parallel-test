import { Link, NavLink } from "react-router-dom"
import { ShoppingCart } from "lucide-react"
import { Badge } from "@/components/ui/badge"
import { NavigationMenu, NavigationMenuList, NavigationMenuItem, NavigationMenuLink } from "@/components/ui/navigation-menu"

export default function Header({ cartCount = 0 }) {
  return (
    <header className="sticky top-0 z-40 w-full border-b bg-background/80 backdrop-blur">
      <div className="container flex h-16 items-center justify-between">
        <Link to="/" className="text-lg font-semibold">ShopSmart</Link>
        <NavigationMenu>
          <NavigationMenuList>
            <NavigationMenuItem>
              <NavLink to="/" className="px-3 py-2 text-sm font-medium" end>Home</NavLink>
            </NavigationMenuItem>
            <NavigationMenuItem>
              <NavLink to="/products" className="px-3 py-2 text-sm font-medium">Products</NavLink>
            </NavigationMenuItem>
            <NavigationMenuItem>
              <NavigationMenuLink asChild>
                <NavLink to="/login" className="px-3 py-2 text-sm font-medium">Login</NavLink>
              </NavigationMenuLink>
            </NavigationMenuItem>
          </NavigationMenuList>
        </NavigationMenu>
        <Link to="/cart" className="relative inline-flex items-center gap-2">
          <ShoppingCart className="h-5 w-5" />
          <Badge variant={cartCount ? "default" : "secondary"}>{cartCount}</Badge>
        </Link>
      </div>
    </header>
  )
}

