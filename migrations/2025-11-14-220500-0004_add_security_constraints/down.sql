-- Drop constraints added in the corresponding up.sql

ALTER TABLE cart_items
    DROP CONSTRAINT IF EXISTS cart_items_quantity_positive;

ALTER TABLE products
    DROP CONSTRAINT IF EXISTS products_inventory_non_negative,
    DROP CONSTRAINT IF EXISTS products_price_non_negative,
    DROP CONSTRAINT IF EXISTS products_description_len,
    DROP CONSTRAINT IF EXISTS products_name_len;

ALTER TABLE users
    DROP CONSTRAINT IF EXISTS users_password_hash_len,
    DROP CONSTRAINT IF EXISTS users_email_len,
    DROP CONSTRAINT IF EXISTS users_username_len;

