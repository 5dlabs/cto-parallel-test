-- Revert integrity constraints

ALTER TABLE cart_items
    DROP CONSTRAINT IF EXISTS cart_items_unique;

ALTER TABLE cart_items
    DROP CONSTRAINT IF EXISTS cart_items_quantity_positive;

ALTER TABLE products
    DROP CONSTRAINT IF EXISTS products_inventory_nonnegative;

ALTER TABLE products
    DROP CONSTRAINT IF EXISTS products_price_nonnegative;

