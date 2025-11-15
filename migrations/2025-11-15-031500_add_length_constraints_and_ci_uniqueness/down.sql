-- Revert length constraints and case-insensitive uniqueness indexes

-- Drop case-insensitive unique indexes if present
DROP INDEX IF EXISTS users_username_lower_unique;
DROP INDEX IF EXISTS users_email_lower_unique;

-- Drop length constraints
ALTER TABLE users
    DROP CONSTRAINT IF EXISTS users_username_length,
    DROP CONSTRAINT IF EXISTS users_email_length;

ALTER TABLE products
    DROP CONSTRAINT IF EXISTS products_name_length;

