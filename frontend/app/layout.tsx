import type { Metadata } from "next";
import "./globals.css";

export const metadata: Metadata = {
  title: "E-Commerce Store",
  description: "Modern e-commerce application built with Next.js 15 and React 19",
};

export default function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  return (
    <html lang="en">
      <body className="antialiased min-h-screen flex flex-col">
        {children}
      </body>
    </html>
  );
}
