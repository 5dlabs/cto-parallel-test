import React from 'react';

const Footer = () => {
  const currentYear = new Date().getFullYear();

  return (
    <footer className="bg-gray-50 border-t border-gray-200 py-8 mt-auto">
      <div className="container mx-auto px-4">
        <div className="flex flex-col md:flex-row justify-between items-center">
          <div className="text-sm text-gray-600 mb-4 md:mb-0">
            © {currentYear} E-Shop. All rights reserved.
          </div>
          <div className="flex space-x-6">
            <a
              href="/privacy"
              className="text-sm text-gray-600 hover:text-primary transition-colors"
            >
              Privacy Policy
            </a>
            <a
              href="/privacy"
              className="text-sm text-gray-600 hover:text-primary transition-colors"
            >
              Terms of Service
            </a>
            <a
              href="/privacy"
              className="text-sm text-gray-600 hover:text-primary transition-colors"
            >
              Contact Us
            </a>
          </div>
        </div>
      </div>
    </footer>
  );
};

export default Footer;
