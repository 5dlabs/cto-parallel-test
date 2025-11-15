import Link from "next/link";
import Image from "next/image";
import { Button } from "@/components/ui/button";
import { Badge } from "@/components/ui/badge";
import { Card, CardContent } from "@/components/ui/card";
import { ArrowLeft, ShoppingCart, Star } from "@/components/icons";
import { getProductById } from "@/lib/products";
import { safeId } from "@/lib/config";

interface Params { params: { id: string } }

export default async function ProductDetailPage({ params }: Params) {
  const id = safeId(params.id)
  const product = await getProductById(id)

  if (!product) {
    return (
      <div className="container py-16 text-center">
        <h1 className="mb-4 text-3xl font-bold">Product Not Found</h1>
        <Link href="/products">
          <Button>
            <ArrowLeft className="mr-2 h-4 w-4" />
            Back to Products
          </Button>
        </Link>
      </div>
    );
  }

  return (
    <div className="container py-8">
      <Link href="/products" className="mb-6 inline-flex items-center text-sm text-muted-foreground hover:text-foreground">
        <ArrowLeft className="mr-2 h-4 w-4" />
        Back to Products
      </Link>

      <div className="grid gap-8 md:grid-cols-2">
        {/* Product Image */}
        <div className="relative aspect-[4/3] overflow-hidden rounded-lg bg-muted">
          <Image
            src={String(product.image || '')}
            alt={String(product.title || product.name || 'Product')}
            width={800}
            height={600}
            className="h-full w-full object-cover"
            unoptimized
          />
          {product.inStock === false && (
            <Badge variant="destructive" className="absolute right-4 top-4">
              Out of Stock
            </Badge>
          )}
        </div>

        {/* Product Info */}
        <div className="flex flex-col gap-6">
          <div>
            <Badge variant="secondary" className="mb-2">
              {product.category || 'General'}
            </Badge>
            <h1 className="mb-2 text-3xl font-bold md:text-4xl">
              {product.title || product.name}
            </h1>
            
            {/* Rating */}
            <div className="flex items-center gap-2">
              <div className="flex items-center">
                {Array.from({ length: 5 }).map((_, i) => (
                  <Star
                    key={i}
                    className={`h-5 w-5 ${
                      i < Math.floor(Number(product.rating || 0))
                        ? "fill-yellow-400 text-yellow-400"
                        : "text-gray-300"
                    }`}
                  />
                ))}
              </div>
              <span className="text-sm text-muted-foreground">
                {Number(product.rating || 0)} ({Number(product.reviews || 0)} reviews)
              </span>
            </div>
          </div>

          <div className="text-4xl font-bold text-primary">
            ${Number(product.price || 0).toFixed(2)}
          </div>

          <p className="text-muted-foreground">{product.description || ''}</p>

          <Card>
            <CardContent className="pt-6">
              <div className="space-y-2">
                <div className="flex justify-between text-sm">
                  <span className="text-muted-foreground">Availability:</span>
                  <span className={product.inStock === false ? "text-red-600" : "text-green-600"}>
                    {product.inStock === false ? "Out of Stock" : "In Stock"}
                  </span>
                </div>
                <div className="flex justify-between text-sm">
                  <span className="text-muted-foreground">Category:</span>
                  <span>{product.category || 'General'}</span>
                </div>
                <div className="flex justify-between text-sm">
                  <span className="text-muted-foreground">Shipping:</span>
                  <span>Free shipping on orders over $50</span>
                </div>
              </div>
            </CardContent>
          </Card>

          <div className="flex gap-4">
            <Button
              size="lg"
              className="flex-1"
              disabled={product.inStock === false}
            >
              <ShoppingCart className="mr-2 h-5 w-5" />
              Add to Cart
            </Button>
          </div>
        </div>
      </div>
    </div>
  );
}
