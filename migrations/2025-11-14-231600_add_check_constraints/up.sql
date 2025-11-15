-- Harden database with integrity constraints for safer defaults
-- Ensures non-negative monetary/quantity values and prevents duplicate cart lines

-- Non-negative prices (financial domain)
ALTER TABLE products
    ADD CONSTRAINT products_price_nonnegative CHECK (price >= 0);

-- Inventory cannot be negative
ALTER TABLE products
    ADD CONSTRAINT products_inventory_nonnegative CHECK (inventory_count >= 0);

-- Cart item quantity must be positive
ALTER TABLE cart_items
    ADD CONSTRAINT cart_items_quantity_positive CHECK (quantity > 0);

-- Avoid duplicate entries of the same product in a single cart
ALTER TABLE cart_items
    ADD CONSTRAINT cart_items_unique UNIQUE (cart_id, product_id);

