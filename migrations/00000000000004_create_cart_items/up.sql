-- Create cart_items table with foreign keys to carts and products
CREATE TABLE cart_items (
  id SERIAL PRIMARY KEY,
  cart_id INTEGER NOT NULL REFERENCES carts(id) ON DELETE CASCADE,
  product_id INTEGER NOT NULL REFERENCES products(id) ON DELETE CASCADE,
  quantity INTEGER NOT NULL CHECK (quantity > 0)
);

-- Create indexes for cart item lookups
CREATE INDEX idx_cart_items_cart_id ON cart_items(cart_id);
CREATE INDEX idx_cart_items_product_id ON cart_items(product_id);

-- Create unique constraint to prevent duplicate products in same cart
CREATE UNIQUE INDEX idx_cart_items_cart_product ON cart_items(cart_id, product_id);
