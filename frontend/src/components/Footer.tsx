import Link from "next/link";
import { Separator } from "@/components/ui/separator";

export default function Footer() {
  const currentYear = new Date().getFullYear();

  return (
    <footer className="border-t mt-auto bg-zinc-50">
      <div className="container mx-auto px-4 py-8">
        <div className="grid grid-cols-1 md:grid-cols-3 gap-8">
          <div>
            <h3 className="font-bold text-lg mb-4">E-Commerce</h3>
            <p className="text-sm text-zinc-600">
              Your trusted online shopping destination for quality products.
            </p>
          </div>
          
          <div>
            <h4 className="font-semibold mb-4">Quick Links</h4>
            <ul className="space-y-2 text-sm">
              <li>
                <Link href="/" className="text-zinc-600 hover:text-primary transition-colors">
                  Home
                </Link>
              </li>
              <li>
                <Link href="/products" className="text-zinc-600 hover:text-primary transition-colors">
                  Products
                </Link>
              </li>
              <li>
                <Link href="/cart" className="text-zinc-600 hover:text-primary transition-colors">
                  Shopping Cart
                </Link>
              </li>
            </ul>
          </div>
          
          <div>
            <h4 className="font-semibold mb-4">Account</h4>
            <ul className="space-y-2 text-sm">
              <li>
                <Link href="/login" className="text-zinc-600 hover:text-primary transition-colors">
                  Login
                </Link>
              </li>
              <li>
                <Link href="/register" className="text-zinc-600 hover:text-primary transition-colors">
                  Register
                </Link>
              </li>
            </ul>
          </div>
        </div>
        
        <Separator className="my-6" />
        
        <div className="text-center text-sm text-zinc-600">
          <p>&copy; {currentYear} E-Commerce. All rights reserved.</p>
        </div>
      </div>
    </footer>
  );
}
