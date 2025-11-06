import React from 'react';

function Footer() {
  const currentYear = new Date().getFullYear();

  return (
    <footer className="border-t bg-gray-50 mt-auto">
      <div className="container mx-auto px-4 py-6">
        <div className="text-center text-sm text-gray-600">
          <p>&copy; {currentYear} E-Shop. All rights reserved.</p>
          <p className="mt-2 text-xs text-gray-500">
            Built with React and shadcn/ui
          </p>
        </div>
      </div>
    </footer>
  );
}

export default Footer;
