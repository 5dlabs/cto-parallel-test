import React from 'react';

function Footer() {
  const currentYear = new Date().getFullYear();

  return (
    <footer className="border-t mt-auto">
      <div className="container mx-auto px-4 py-6">
        <div className="text-center text-sm text-muted-foreground">
          © {currentYear} E-Commerce App. All rights reserved.
        </div>
      </div>
    </footer>
  );
}

export default Footer;
