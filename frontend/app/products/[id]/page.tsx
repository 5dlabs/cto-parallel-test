import Link from "next/link";
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "@/components/ui/card";
import { Button } from "@/components/ui/button";
import { Badge } from "@/components/ui/badge";
import { ArrowLeft, ShoppingCart, Package, Truck, Shield } from "lucide-react";

// Sample product data - in a real app, this would come from an API
const getProduct = (id: string) => {
  const products = [
    {
      id: 1,
      name: "Premium Wireless Headphones",
      description: "High-quality sound with active noise cancellation",
      longDescription:
        "Experience superior audio quality with our Premium Wireless Headphones. Featuring advanced noise cancellation technology, these headphones provide an immersive listening experience. With 30-hour battery life and comfortable ear cushions, they're perfect for long listening sessions.",
      price: 299.99,
      category: "Electronics",
      inStock: true,
      specifications: {
        "Battery Life": "30 hours",
        "Connection": "Bluetooth 5.0",
        "Weight": "250g",
        "Warranty": "2 years",
      },
    },
    {
      id: 2,
      name: "Smart Watch Pro",
      description: "Track your fitness and stay connected",
      longDescription:
        "Stay connected and monitor your health with the Smart Watch Pro. This advanced wearable tracks your heart rate, steps, sleep patterns, and more. With a vibrant AMOLED display and week-long battery life, it's the perfect companion for your active lifestyle.",
      price: 399.99,
      category: "Electronics",
      inStock: true,
      specifications: {
        "Display": "1.4\" AMOLED",
        "Battery": "7 days",
        "Water Resistant": "5ATM",
        "Sensors": "Heart rate, GPS, SpO2",
      },
    },
    {
      id: 3,
      name: "Laptop Backpack",
      description: "Durable and spacious laptop bag",
      longDescription:
        "Protect your devices with this premium laptop backpack. Features a dedicated padded compartment for laptops up to 17 inches, multiple organizational pockets, and water-resistant material. Perfect for work, school, or travel.",
      price: 79.99,
      category: "Accessories",
      inStock: true,
      specifications: {
        "Capacity": "35L",
        "Laptop Size": "Up to 17\"",
        "Material": "Water-resistant nylon",
        "Dimensions": "45 x 30 x 15 cm",
      },
    },
  ];

  const productId = parseInt(id);
  return products.find((p) => p.id === productId) || products[0];
};

export default async function ProductDetailPage({
  params,
}: {
  params: Promise<{ id: string }>;
}) {
  const { id } = await params;
  const product = getProduct(id);

  return (
    <div className="container mx-auto px-4 py-12">
      <Link
        href="/products"
        className="inline-flex items-center gap-2 text-sm text-muted-foreground hover:text-foreground mb-8"
      >
        <ArrowLeft className="h-4 w-4" />
        Back to Products
      </Link>

      <div className="grid grid-cols-1 lg:grid-cols-2 gap-12">
        {/* Product Image Placeholder */}
        <div className="aspect-square bg-muted rounded-lg flex items-center justify-center">
          <Package className="h-32 w-32 text-muted-foreground" />
        </div>

        {/* Product Details */}
        <div className="space-y-6">
          <div>
            <div className="flex items-center gap-2 mb-2">
              <Badge variant="secondary">{product.category}</Badge>
              {!product.inStock && (
                <Badge variant="destructive">Out of Stock</Badge>
              )}
            </div>
            <h1 className="text-4xl font-bold mb-4">{product.name}</h1>
            <p className="text-xl text-muted-foreground mb-6">
              {product.description}
            </p>
            <p className="text-4xl font-bold mb-6">${product.price.toFixed(2)}</p>
          </div>

          <div className="space-y-4">
            <Button
              size="lg"
              className="w-full"
              disabled={!product.inStock}
              aria-label={`Add ${product.name} to cart`}
            >
              <ShoppingCart className="mr-2 h-5 w-5" />
              {product.inStock ? "Add to Cart" : "Out of Stock"}
            </Button>
          </div>

          {/* Product Information */}
          <Card>
            <CardHeader>
              <CardTitle>Product Information</CardTitle>
            </CardHeader>
            <CardContent className="space-y-4">
              <p className="text-muted-foreground">{product.longDescription}</p>
            </CardContent>
          </Card>

          {/* Specifications */}
          <Card>
            <CardHeader>
              <CardTitle>Specifications</CardTitle>
            </CardHeader>
            <CardContent>
              <dl className="space-y-2">
                {Object.entries(product.specifications).map(([key, value]) => (
                  <div key={key} className="flex justify-between py-2 border-b last:border-0">
                    <dt className="font-medium">{key}</dt>
                    <dd className="text-muted-foreground">{value}</dd>
                  </div>
                ))}
              </dl>
            </CardContent>
          </Card>

          {/* Features */}
          <div className="grid grid-cols-1 sm:grid-cols-3 gap-4">
            <Card>
              <CardHeader className="pb-3">
                <Truck className="h-8 w-8 mb-2 text-primary" />
                <CardTitle className="text-sm">Free Shipping</CardTitle>
                <CardDescription className="text-xs">On orders over $100</CardDescription>
              </CardHeader>
            </Card>
            <Card>
              <CardHeader className="pb-3">
                <Shield className="h-8 w-8 mb-2 text-primary" />
                <CardTitle className="text-sm">Warranty</CardTitle>
                <CardDescription className="text-xs">2 year coverage</CardDescription>
              </CardHeader>
            </Card>
            <Card>
              <CardHeader className="pb-3">
                <Package className="h-8 w-8 mb-2 text-primary" />
                <CardTitle className="text-sm">Easy Returns</CardTitle>
                <CardDescription className="text-xs">30 day guarantee</CardDescription>
              </CardHeader>
            </Card>
          </div>
        </div>
      </div>
    </div>
  );
}
