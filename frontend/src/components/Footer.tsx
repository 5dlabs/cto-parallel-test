import Link from 'next/link';

export function Footer() {
  const currentYear = new Date().getFullYear();

  return (
    <footer className="mt-auto border-t bg-background">
      <div className="container mx-auto px-4 py-6 md:px-6 md:py-8">
        <div className="grid gap-8 md:grid-cols-3">
          {/* Company Info */}
          <div>
            <h3 className="mb-2 text-lg font-semibold">E-Commerce</h3>
            <p className="text-sm text-muted-foreground">
              Your trusted online shopping destination for quality products.
            </p>
          </div>

          {/* Quick Links */}
          <div>
            <h3 className="mb-2 text-lg font-semibold">Quick Links</h3>
            <nav aria-label="Footer navigation">
              <ul className="space-y-2 text-sm">
                <li>
                  <Link href="/" className="text-muted-foreground hover:text-foreground">
                    Home
                  </Link>
                </li>
                <li>
                  <Link href="/products" className="text-muted-foreground hover:text-foreground">
                    Products
                  </Link>
                </li>
                <li>
                  <Link href="/cart" className="text-muted-foreground hover:text-foreground">
                    Cart
                  </Link>
                </li>
              </ul>
            </nav>
          </div>

          {/* Account */}
          <div>
            <h3 className="mb-2 text-lg font-semibold">Account</h3>
            <nav aria-label="Account navigation">
              <ul className="space-y-2 text-sm">
                <li>
                  <Link href="/login" className="text-muted-foreground hover:text-foreground">
                    Login
                  </Link>
                </li>
                <li>
                  <Link href="/register" className="text-muted-foreground hover:text-foreground">
                    Register
                  </Link>
                </li>
              </ul>
            </nav>
          </div>
        </div>

        <div className="mt-8 border-t pt-6 text-center text-sm text-muted-foreground">
          <p>&copy; {currentYear} E-Commerce. All rights reserved.</p>
        </div>
      </div>
    </footer>
  );
}
