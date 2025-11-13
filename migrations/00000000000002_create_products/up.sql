-- Create products table
-- This table stores product catalog information

CREATE TABLE products (
  id SERIAL PRIMARY KEY,
  name VARCHAR NOT NULL,
  description TEXT,
  price NUMERIC NOT NULL CHECK (price >= 0),
  inventory_count INTEGER NOT NULL DEFAULT 0 CHECK (inventory_count >= 0)
);

-- Create index for product name searches
CREATE INDEX idx_products_name ON products(name);
