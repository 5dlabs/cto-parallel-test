import Link from "next/link";

export default function Footer() {
  const currentYear = new Date().getFullYear();

  return (
    <footer className="mt-auto border-t bg-background">
      <div className="container mx-auto px-4 py-6 md:px-6">
        <div className="flex flex-col items-center justify-between gap-4 md:flex-row">
          <p className="text-center text-sm text-muted-foreground">
            © {currentYear} E-Commerce. All rights reserved.
          </p>
          <nav className="flex gap-4">
            <Link
              href="/products"
              className="text-sm text-muted-foreground transition-colors hover:text-foreground"
            >
              Products
            </Link>
            <Link
              href="/cart"
              className="text-sm text-muted-foreground transition-colors hover:text-foreground"
            >
              Cart
            </Link>
          </nav>
        </div>
      </div>
    </footer>
  );
}
