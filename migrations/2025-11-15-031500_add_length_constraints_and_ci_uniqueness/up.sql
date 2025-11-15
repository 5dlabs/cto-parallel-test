-- Strengthen data integrity and security defaults with length constraints
-- and case-insensitive uniqueness on user identifiers.

-- Username/email length constraints to mitigate oversized input attacks
ALTER TABLE users
    ADD CONSTRAINT users_username_length CHECK (char_length(username) BETWEEN 3 AND 64),
    ADD CONSTRAINT users_email_length CHECK (char_length(email) BETWEEN 3 AND 254);

-- Ensure product name is not empty and reasonably bounded
ALTER TABLE products
    ADD CONSTRAINT products_name_length CHECK (char_length(name) BETWEEN 1 AND 255);

-- Enforce case-insensitive uniqueness for username and email to avoid
-- duplicate accounts differing only by case
CREATE UNIQUE INDEX IF NOT EXISTS users_username_lower_unique ON users (LOWER(username));
CREATE UNIQUE INDEX IF NOT EXISTS users_email_lower_unique ON users (LOWER(email));

