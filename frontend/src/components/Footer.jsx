import React from 'react';

export function Footer() {
  return (
    <footer className="border-t mt-auto">
      <div className="container mx-auto px-4 py-6">
        <p className="text-center text-sm text-muted-foreground">
          © {new Date().getFullYear()} E-Commerce Store. All rights reserved.
        </p>
      </div>
    </footer>
  );
}
