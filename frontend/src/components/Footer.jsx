import React from 'react';

function Footer() {
  return (
    <footer className="border-t bg-background">
      <div className="container mx-auto px-4 py-6">
        <div className="text-center text-sm text-muted-foreground">
          © {new Date().getFullYear()} E-Store. All rights reserved.
        </div>
      </div>
    </footer>
  );
}

export default Footer;
