import Link from "next/link";
import Image from "next/image";
import { Card, CardContent, CardFooter, CardHeader, CardTitle } from "@/components/ui/card";
import { Button } from "@/components/ui/button";
import { Badge } from "@/components/ui/badge";
import { getAllProducts } from "@/lib/products";

export default async function ProductsPage() {
  const products = await getAllProducts();
  return (
    <div className="container py-8">
      <div className="mb-8">
        <h1 className="mb-2 text-3xl font-bold md:text-4xl">Products</h1>
        <p className="text-muted-foreground">
          Browse our collection of quality products
        </p>
      </div>

      <div className="grid gap-6 sm:grid-cols-2 lg:grid-cols-3">
        {products.map((product) => (
          <Card key={product.id} className="flex flex-col overflow-hidden">
            <CardHeader className="p-0">
              <div className="relative aspect-[4/3] w-full overflow-hidden bg-muted">
                <Image
                  src={String(product.image || '')}
                  alt={String(product.title || product.name || 'Product')}
                  width={400}
                  height={300}
                  className="h-full w-full object-cover transition-transform hover:scale-105"
                  unoptimized
                />
                {product.inStock === false && (
                  <Badge
                    variant="destructive"
                    className="absolute right-2 top-2"
                  >
                    Out of Stock
                  </Badge>
                )}
              </div>
            </CardHeader>
            <CardContent className="flex-1 p-4">
              <Badge variant="secondary" className="mb-2">
                {product.category || 'General'}
              </Badge>
              <CardTitle className="mb-2 text-xl">{product.title || product.name}</CardTitle>
              <p className="text-2xl font-bold text-primary">
                ${Number(product.price || 0).toFixed(2)}
              </p>
            </CardContent>
            <CardFooter className="p-4 pt-0">
              <Link href={`/products/${encodeURIComponent(String(product.id))}`} className="w-full">
                <Button className="w-full" disabled={product.inStock === false}>
                  {product.inStock === false ? "Out of Stock" : "View Details"}
                </Button>
              </Link>
            </CardFooter>
          </Card>
        ))}
      </div>
    </div>
  );
}
