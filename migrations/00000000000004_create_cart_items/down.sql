-- Drop indexes first
DROP INDEX IF EXISTS idx_cart_items_cart_product;
DROP INDEX IF EXISTS idx_cart_items_product_id;
DROP INDEX IF EXISTS idx_cart_items_cart_id;

-- Drop cart_items table
DROP TABLE cart_items;
