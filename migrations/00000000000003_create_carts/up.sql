-- Create carts table
-- This table stores shopping carts associated with users

CREATE TABLE carts (
  id SERIAL PRIMARY KEY,
  user_id INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Create index for user cart lookups
CREATE INDEX idx_carts_user_id ON carts(user_id);
