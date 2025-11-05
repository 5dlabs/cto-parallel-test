"use client";

import { useParams, useRouter } from "next/navigation";
import { Button } from "@/components/ui/button";
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "@/components/ui/card";
import { Badge } from "@/components/ui/badge";
import { ArrowLeft, ShoppingCart } from "lucide-react";

interface Product {
  id: number;
  name: string;
  description: string;
  price: number;
  category: string;
  inStock: boolean;
  fullDescription: string;
  specifications: { label: string; value: string }[];
}

export default function ProductDetailPage() {
  const params = useParams();
  const router = useRouter();
  const productId = Number(params.id);

  // In a real app, this would come from an API
  const products: Record<number, Product> = {
    1: {
      id: 1,
      name: "Wireless Headphones",
      description: "Premium noise-cancelling wireless headphones",
      price: 299.99,
      category: "Electronics",
      inStock: true,
      fullDescription:
        "Experience exceptional sound quality with our premium wireless headphones. Featuring advanced noise-cancelling technology, these headphones deliver crystal-clear audio whether you're working, traveling, or relaxing. With up to 30 hours of battery life and comfortable ear cushions, you can enjoy your music all day long.",
      specifications: [
        { label: "Battery Life", value: "30 hours" },
        { label: "Connectivity", value: "Bluetooth 5.0" },
        { label: "Weight", value: "250g" },
        { label: "Warranty", value: "2 years" },
      ],
    },
    2: {
      id: 2,
      name: "Smart Watch",
      description: "Fitness tracker with heart rate monitor",
      price: 199.99,
      category: "Electronics",
      inStock: true,
      fullDescription:
        "Track your fitness goals with our advanced smart watch. Monitor your heart rate, steps, calories, and sleep patterns. Stay connected with notifications and control your music right from your wrist. Water-resistant design makes it perfect for all activities.",
      specifications: [
        { label: "Display", value: "1.4\" AMOLED" },
        { label: "Battery Life", value: "7 days" },
        { label: "Water Resistance", value: "5 ATM" },
        { label: "Connectivity", value: "Bluetooth 5.0" },
      ],
    },
    3: {
      id: 3,
      name: "Laptop Backpack",
      description: "Durable backpack with padded laptop compartment",
      price: 79.99,
      category: "Accessories",
      inStock: true,
      fullDescription:
        "Protect your laptop and carry all your essentials with this durable backpack. Features a padded compartment for laptops up to 15.6 inches, multiple organizational pockets, and comfortable padded straps. Made from water-resistant material to keep your belongings safe in any weather.",
      specifications: [
        { label: "Capacity", value: "25L" },
        { label: "Laptop Size", value: "Up to 15.6\"" },
        { label: "Material", value: "Water-resistant polyester" },
        { label: "Dimensions", value: "18\" x 12\" x 7\"" },
      ],
    },
    4: {
      id: 4,
      name: "Mechanical Keyboard",
      description: "RGB backlit mechanical gaming keyboard",
      price: 149.99,
      category: "Electronics",
      inStock: false,
      fullDescription:
        "Elevate your gaming and typing experience with this premium mechanical keyboard. Featuring customizable RGB backlighting, programmable keys, and responsive mechanical switches. Built with a durable aluminum frame and includes a detachable wrist rest for comfort during long sessions.",
      specifications: [
        { label: "Switch Type", value: "Cherry MX Blue" },
        { label: "Backlighting", value: "RGB (16.8M colors)" },
        { label: "Connection", value: "USB-C wired" },
        { label: "Key Rollover", value: "N-key" },
      ],
    },
    5: {
      id: 5,
      name: "USB-C Hub",
      description: "7-in-1 USB-C hub with HDMI and USB 3.0",
      price: 49.99,
      category: "Accessories",
      inStock: true,
      fullDescription:
        "Expand your laptop's connectivity with this versatile USB-C hub. Features HDMI output supporting 4K@30Hz, three USB 3.0 ports for fast data transfer, SD and microSD card readers, and USB-C power delivery pass-through. Compact and portable design perfect for work and travel.",
      specifications: [
        { label: "Ports", value: "7 (HDMI, 3x USB-A, SD, microSD, USB-C PD)" },
        { label: "HDMI Output", value: "4K@30Hz" },
        { label: "USB Speed", value: "USB 3.0 (5Gbps)" },
        { label: "Power Delivery", value: "100W pass-through" },
      ],
    },
    6: {
      id: 6,
      name: "Wireless Mouse",
      description: "Ergonomic wireless mouse with precision tracking",
      price: 39.99,
      category: "Electronics",
      inStock: true,
      fullDescription:
        "Designed for comfort and precision, this wireless mouse features an ergonomic shape that reduces wrist strain. With adjustable DPI settings up to 3200, it's perfect for both work and gaming. Long battery life and silent clicking make it ideal for any environment.",
      specifications: [
        { label: "DPI Range", value: "800-3200" },
        { label: "Battery Life", value: "12 months" },
        { label: "Connectivity", value: "2.4GHz wireless" },
        { label: "Buttons", value: "6 programmable" },
      ],
    },
  };

  const product = products[productId];

  if (!product) {
    return (
      <div className="container mx-auto px-4 py-8 md:px-6">
        <div className="text-center">
          <h1 className="text-2xl font-bold">Product Not Found</h1>
          <p className="mt-2 text-muted-foreground">
            The product you&apos;re looking for doesn&apos;t exist.
          </p>
          <Button className="mt-4" onClick={() => router.push("/products")}>
            Back to Products
          </Button>
        </div>
      </div>
    );
  }

  return (
    <div className="container mx-auto px-4 py-8 md:px-6">
      <Button
        variant="ghost"
        className="mb-6"
        onClick={() => router.push("/products")}
      >
        <ArrowLeft className="mr-2 h-4 w-4" />
        Back to Products
      </Button>

      <div className="grid gap-8 lg:grid-cols-2">
        {/* Product Image Placeholder */}
        <div className="aspect-square rounded-lg bg-muted flex items-center justify-center">
          <span className="text-muted-foreground text-lg">Product Image</span>
        </div>

        {/* Product Info */}
        <div className="flex flex-col gap-6">
          <div>
            <div className="flex items-start justify-between gap-4">
              <h1 className="text-3xl font-bold md:text-4xl">{product.name}</h1>
              {!product.inStock && (
                <Badge variant="secondary">Out of Stock</Badge>
              )}
            </div>
            <div className="mt-2 flex items-center gap-2">
              <Badge variant="outline">{product.category}</Badge>
            </div>
            <p className="mt-4 text-muted-foreground">{product.description}</p>
          </div>

          <div className="flex items-baseline gap-2">
            <span className="text-4xl font-bold">
              ${product.price.toFixed(2)}
            </span>
          </div>

          <Button
            size="lg"
            disabled={!product.inStock}
            className="w-full md:w-auto"
          >
            <ShoppingCart className="mr-2 h-5 w-5" />
            {product.inStock ? "Add to Cart" : "Out of Stock"}
          </Button>

          <Card>
            <CardHeader>
              <CardTitle>About this product</CardTitle>
            </CardHeader>
            <CardContent>
              <p className="text-sm text-muted-foreground">
                {product.fullDescription}
              </p>
            </CardContent>
          </Card>

          <Card>
            <CardHeader>
              <CardTitle>Specifications</CardTitle>
            </CardHeader>
            <CardContent>
              <dl className="grid gap-3">
                {product.specifications.map((spec, index) => (
                  <div
                    key={index}
                    className="flex justify-between border-b pb-2 last:border-0 last:pb-0"
                  >
                    <dt className="font-medium text-sm">{spec.label}</dt>
                    <dd className="text-sm text-muted-foreground">
                      {spec.value}
                    </dd>
                  </div>
                ))}
              </dl>
            </CardContent>
          </Card>
        </div>
      </div>
    </div>
  );
}
