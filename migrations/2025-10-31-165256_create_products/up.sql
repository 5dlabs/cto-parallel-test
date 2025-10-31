-- Create products table for product catalog
CREATE TABLE products (
  id SERIAL PRIMARY KEY,
  name VARCHAR NOT NULL,
  description TEXT,
  price NUMERIC NOT NULL CHECK (price >= 0),
  inventory_count INTEGER NOT NULL CHECK (inventory_count >= 0)
);

-- Create indexes for better query performance
CREATE INDEX idx_products_name ON products(name);
CREATE INDEX idx_products_price ON products(price);
