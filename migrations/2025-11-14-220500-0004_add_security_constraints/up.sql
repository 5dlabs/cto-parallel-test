-- Add defensive constraints to enforce basic data validation at the database layer
-- These checks help mitigate abuse (oversized inputs) and ensure sane values

-- Users constraints
ALTER TABLE users
    ADD CONSTRAINT users_username_len CHECK (char_length(username) BETWEEN 3 AND 64);

ALTER TABLE users
    ADD CONSTRAINT users_email_len CHECK (char_length(email) BETWEEN 3 AND 254);

-- Most modern password hash encodings (bcrypt, argon2) are <= 255 chars
ALTER TABLE users
    ADD CONSTRAINT users_password_hash_len CHECK (char_length(password_hash) BETWEEN 60 AND 255);

-- Products constraints
ALTER TABLE products
    ADD CONSTRAINT products_name_len CHECK (char_length(name) BETWEEN 1 AND 200);

ALTER TABLE products
    ADD CONSTRAINT products_description_len CHECK (description IS NULL OR char_length(description) <= 5000);

ALTER TABLE products
    ADD CONSTRAINT products_price_non_negative CHECK (price >= 0);

ALTER TABLE products
    ADD CONSTRAINT products_inventory_non_negative CHECK (inventory_count >= 0);

-- Cart items constraints
ALTER TABLE cart_items
    ADD CONSTRAINT cart_items_quantity_positive CHECK (quantity > 0);

