//! Configuration module for database and application settings

pub mod db;

// Re-export commonly used types
pub use db::{establish_connection_pool, DbConnection, Pool};
