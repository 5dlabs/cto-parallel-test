pub mod config;
pub mod models;
pub mod schema;

// Re-export commonly used items for convenience
pub use config::db::{establish_connection_pool, DbConnection, Pool};
pub use models::{Cart, CartItem, NewCart, NewCartItem, NewProduct, NewUser, Product, User};
