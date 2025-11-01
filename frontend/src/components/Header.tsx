"use client";

import Link from "next/link";
import { ShoppingCart, User } from "lucide-react";
import { Button } from "@/components/ui/button";
import { Badge } from "@/components/ui/badge";

export default function Header() {
  // In a real app, this would come from cart state management
  const cartItemCount = 0;

  return (
    <header className="border-b sticky top-0 bg-white z-50">
      <div className="container mx-auto px-4 py-4 flex items-center justify-between">
        <Link href="/" className="text-2xl font-bold text-primary">
          E-Commerce
        </Link>
        
        <nav className="hidden md:flex items-center gap-6">
          <Link href="/" className="text-sm font-medium hover:text-primary transition-colors">
            Home
          </Link>
          <Link href="/products" className="text-sm font-medium hover:text-primary transition-colors">
            Products
          </Link>
          <Link href="/cart" className="text-sm font-medium hover:text-primary transition-colors">
            Cart
          </Link>
        </nav>

        <div className="flex items-center gap-4">
          <Link href="/cart" className="relative">
            <Button variant="ghost" size="icon" aria-label="Shopping cart">
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
          
          <Link href="/login">
            <Button variant="outline" size="sm" className="gap-2">
              <User className="h-4 w-4" />
              <span className="hidden sm:inline">Login</span>
            </Button>
          </Link>
        </div>
      </div>

      {/* Mobile navigation */}
      <nav className="md:hidden border-t px-4 py-2 flex justify-around">
        <Link href="/" className="text-xs font-medium hover:text-primary transition-colors py-2">
          Home
        </Link>
        <Link href="/products" className="text-xs font-medium hover:text-primary transition-colors py-2">
          Products
        </Link>
        <Link href="/cart" className="text-xs font-medium hover:text-primary transition-colors py-2">
          Cart
        </Link>
      </nav>
    </header>
  );
}
