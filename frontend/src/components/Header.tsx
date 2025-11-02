"use client";

import Link from "next/link";
import { ShoppingCart, User } from "lucide-react";
import { Badge } from "@/components/ui/badge";
import { Button } from "@/components/ui/button";
import { useState } from "react";

export function Header() {
  const [cartCount] = useState(0); // This will be replaced with actual cart state

  return (
    <header className="sticky top-0 z-50 w-full border-b bg-background/95 backdrop-blur supports-[backdrop-filter]:bg-background/60">
      <div className="container mx-auto px-4 sm:px-6 lg:px-8">
        <div className="flex h-16 items-center justify-between">
          {/* Logo */}
          <Link href="/" className="flex items-center space-x-2">
            <span className="text-2xl font-bold">E-Shop</span>
          </Link>

          {/* Navigation - Desktop */}
          <nav className="hidden md:flex items-center space-x-6">
            <Link
              href="/"
              className="text-sm font-medium transition-colors hover:text-primary"
            >
              Home
            </Link>
            <Link
              href="/products"
              className="text-sm font-medium transition-colors hover:text-primary"
            >
              Products
            </Link>
          </nav>

          {/* Actions */}
          <div className="flex items-center space-x-4">
            <Link href="/cart">
              <Button variant="ghost" size="icon" className="relative">
                <ShoppingCart className="h-5 w-5" />
                {cartCount > 0 && (
                  <Badge
                    variant="destructive"
                    className="absolute -right-2 -top-2 h-5 w-5 rounded-full p-0 flex items-center justify-center text-xs"
                  >
                    {cartCount}
                  </Badge>
                )}
                <span className="sr-only">Shopping cart</span>
              </Button>
            </Link>
            <Link href="/login">
              <Button variant="ghost" size="icon">
                <User className="h-5 w-5" />
                <span className="sr-only">User account</span>
              </Button>
            </Link>
          </div>
        </div>

        {/* Navigation - Mobile */}
        <nav className="flex md:hidden items-center space-x-6 pb-4">
          <Link
            href="/"
            className="text-sm font-medium transition-colors hover:text-primary"
          >
            Home
          </Link>
          <Link
            href="/products"
            className="text-sm font-medium transition-colors hover:text-primary"
          >
            Products
          </Link>
        </nav>
      </div>
    </header>
  );
}
