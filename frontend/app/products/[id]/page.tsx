"use client";

import { useEffect, useState } from "react";
import { useParams, useRouter } from "next/navigation";
import { Button } from "@/components/ui/button";
import { Badge } from "@/components/ui/badge";
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "@/components/ui/card";
import { ArrowLeft, ShoppingCart, Loader2, AlertCircle } from "lucide-react";
import { productsApi, cartApi, type Product } from "@/lib/api";

export default function ProductDetail() {
  const params = useParams();
  const router = useRouter();
  // Explicit radix to avoid implicit octal/hex parsing
  const productId = parseInt(params.id as string, 10);

  const [product, setProduct] = useState<Product | null>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [addingToCart, setAddingToCart] = useState(false);

  useEffect(() => {
    async function fetchProduct() {
      try {
        setLoading(true);
        setError(null);
        const data = await productsApi.getById(productId);
        setProduct(data);
      } catch (err) {
        setError(err instanceof Error ? err.message : 'Failed to load product');
        console.error('Error fetching product:', err);
      } finally {
        setLoading(false);
      }
    }

    if (!isNaN(productId)) {
      fetchProduct();
    } else {
      setError('Invalid product ID');
      setLoading(false);
    }
  }, [productId]);

  const handleAddToCart = async () => {
    if (!product) return;

    try {
      setAddingToCart(true);
      await cartApi.addItem(product.id, 1);
      alert('Added to cart!');
    } catch (err) {
      const errorMsg = err instanceof Error ? err.message : 'Failed to add to cart';
      alert(errorMsg);
      console.error('Error adding to cart:', err);
    } finally {
      setAddingToCart(false);
    }
  };

  if (loading) {
    return (
      <div className="container mx-auto px-4 py-8 sm:px-6 lg:px-8">
        <div className="flex flex-col items-center justify-center py-16 space-y-4">
          <Loader2 className="h-12 w-12 animate-spin text-primary" aria-hidden="true" />
          <p className="text-muted-foreground">Loading product...</p>
        </div>
      </div>
    );
  }

  if (error || !product) {
    return (
      <div className="container mx-auto px-4 py-8 sm:px-6 lg:px-8">
        <div className="flex flex-col items-center justify-center space-y-4 py-16">
          <AlertCircle className="h-12 w-12 text-destructive" aria-hidden="true" />
          <h1 className="text-2xl font-bold">Product Not Found</h1>
          <p className="text-muted-foreground">{error || "The product you're looking for doesn't exist."}</p>
          <Button onClick={() => router.push("/products")}>
            <ArrowLeft className="mr-2 h-4 w-4" aria-hidden="true" />
            Back to Products
          </Button>
        </div>
      </div>
    );
  }

  return (
    <div className="container mx-auto px-4 py-8 sm:px-6 lg:px-8">
      <Button
        variant="ghost"
        onClick={() => router.back()}
        className="mb-6"
        aria-label="Go back"
      >
        <ArrowLeft className="mr-2 h-4 w-4" aria-hidden="true" />
        Back
      </Button>

      <div className="grid gap-8 md:grid-cols-2">
        {/* Product Image Placeholder */}
        <div className="aspect-square rounded-lg border bg-muted flex items-center justify-center">
          <span className="text-4xl text-muted-foreground">{product.name[0]}</span>
        </div>

        {/* Product Details */}
        <div className="flex flex-col space-y-4">
          <div>
            <div className="flex items-center gap-2 mb-2">
              <Badge variant="outline">{product.category}</Badge>
              {!product.inStock && (
                <Badge variant="secondary">Out of Stock</Badge>
              )}
            </div>
            <h1 className="text-3xl font-bold tracking-tight">{product.name}</h1>
            <p className="mt-2 text-xl font-semibold">${product.price.toFixed(2)}</p>
          </div>

          <p className="text-muted-foreground">{product.description}</p>

          {(product.details || product.specs) && (
            <Card>
              <CardHeader>
                <CardTitle className="text-lg">Product Details</CardTitle>
                {product.details && <CardDescription>{product.details}</CardDescription>}
              </CardHeader>
              {product.specs && product.specs.length > 0 && (
                <CardContent>
                  <h4 className="font-semibold mb-2">Specifications:</h4>
                  <ul className="list-disc list-inside space-y-1 text-sm text-muted-foreground">
                    {product.specs.map((spec, index) => (
                      <li key={index}>{spec}</li>
                    ))}
                  </ul>
                </CardContent>
              )}
            </Card>
          )}

          <div className="flex gap-4 pt-4">
            <Button
              size="lg"
              className="flex-1 gap-2"
              disabled={!product.inStock || addingToCart}
              onClick={handleAddToCart}
            >
              {addingToCart ? (
                <>
                  <Loader2 className="h-4 w-4 animate-spin" aria-hidden="true" />
                  Adding...
                </>
              ) : (
                <>
                  <ShoppingCart className="h-4 w-4" aria-hidden="true" />
                  Add to Cart
                </>
              )}
            </Button>
          </div>
        </div>
      </div>
    </div>
  );
}
