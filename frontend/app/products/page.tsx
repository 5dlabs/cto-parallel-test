import Link from "next/link";
import { Button } from "@/components/ui/button";
import { Card, CardContent, CardDescription, CardFooter, CardHeader, CardTitle } from "@/components/ui/card";
import { Badge } from "@/components/ui/badge";
import { Header } from "@/components/Header";
import { Footer } from "@/components/Footer";

// Sample product data - In production, this would come from an API
const products = [
  {
    id: 1,
    name: "Premium Wireless Headphones",
    description: "High-quality sound with active noise cancellation",
    price: 299.99,
    category: "Electronics",
    inStock: true,
    image: "https://images.unsplash.com/photo-1505740420928-5e560c06d30e?w=400&h=400&fit=crop",
  },
  {
    id: 2,
    name: "Smart Watch Pro",
    description: "Track your fitness and stay connected",
    price: 399.99,
    category: "Electronics",
    inStock: true,
    image: "https://images.unsplash.com/photo-1523275335684-37898b6baf30?w=400&h=400&fit=crop",
  },
  {
    id: 3,
    name: "Leather Messenger Bag",
    description: "Handcrafted genuine leather bag",
    price: 189.99,
    category: "Accessories",
    inStock: true,
    image: "https://images.unsplash.com/photo-1553062407-98eeb64c6a62?w=400&h=400&fit=crop",
  },
  {
    id: 4,
    name: "Running Shoes",
    description: "Comfortable and stylish athletic shoes",
    price: 129.99,
    category: "Footwear",
    inStock: true,
    image: "https://images.unsplash.com/photo-1542291026-7eec264c27ff?w=400&h=400&fit=crop",
  },
  {
    id: 5,
    name: "Minimalist Wallet",
    description: "Slim design with RFID protection",
    price: 49.99,
    category: "Accessories",
    inStock: false,
    image: "https://images.unsplash.com/photo-1627123424574-724758594e93?w=400&h=400&fit=crop",
  },
  {
    id: 6,
    name: "Portable Speaker",
    description: "Waterproof Bluetooth speaker",
    price: 79.99,
    category: "Electronics",
    inStock: true,
    image: "https://images.unsplash.com/photo-1608043152269-423dbba4e7e1?w=400&h=400&fit=crop",
  },
  {
    id: 7,
    name: "Designer Sunglasses",
    description: "UV protection with polarized lenses",
    price: 159.99,
    category: "Accessories",
    inStock: true,
    image: "https://images.unsplash.com/photo-1572635196237-14b3f281503f?w=400&h=400&fit=crop",
  },
  {
    id: 8,
    name: "Backpack Pro",
    description: "Durable backpack with laptop compartment",
    price: 89.99,
    category: "Accessories",
    inStock: true,
    image: "https://images.unsplash.com/photo-1553062407-98eeb64c6a62?w=400&h=400&fit=crop",
  },
];

export default function ProductsPage() {
  return (
    <>
      <Header />
      <main className="flex-1">
        <div className="container mx-auto px-4 py-8">
          <div className="mb-8">
            <h1 className="text-4xl font-bold tracking-tight mb-2">Our Products</h1>
            <p className="text-muted-foreground">
              Discover our curated collection of premium products
            </p>
          </div>

          <div className="grid gap-6 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4">
            {products.map((product) => (
              <Card key={product.id} className="flex flex-col overflow-hidden hover:shadow-lg transition-shadow">
                <div className="aspect-square relative bg-slate-100">
                  <img
                    src={product.image}
                    alt={product.name}
                    className="object-cover w-full h-full"
                  />
                  <div className="absolute top-2 right-2">
                    <Badge variant={product.inStock ? "default" : "secondary"}>
                      {product.inStock ? "In Stock" : "Out of Stock"}
                    </Badge>
                  </div>
                </div>
                <CardHeader>
                  <CardTitle className="line-clamp-1">{product.name}</CardTitle>
                  <CardDescription className="line-clamp-2">
                    {product.description}
                  </CardDescription>
                </CardHeader>
                <CardContent className="flex-1">
                  <p className="text-2xl font-bold">${product.price.toFixed(2)}</p>
                  <p className="text-sm text-muted-foreground mt-1">{product.category}</p>
                </CardContent>
                <CardFooter className="flex gap-2">
                  <Link href={`/products/${product.id}`} className="flex-1">
                    <Button variant="outline" className="w-full">
                      View Details
                    </Button>
                  </Link>
                  <Button 
                    className="flex-1" 
                    disabled={!product.inStock}
                  >
                    Add to Cart
                  </Button>
                </CardFooter>
              </Card>
            ))}
          </div>
        </div>
      </main>
      <Footer />
    </>
  );
}
