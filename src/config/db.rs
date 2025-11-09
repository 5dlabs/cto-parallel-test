/// Database connection pooling configuration
///
/// This module provides connection pool management for `PostgreSQL` using Diesel ORM.
/// It uses r2d2 for efficient connection pooling.
use diesel::pg::PgConnection;
use diesel::r2d2::{self, ConnectionManager};
use dotenvy::dotenv;
use std::env;

/// Type alias for the database connection pool
pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

/// Type alias for a pooled database connection
pub type DbConnection = r2d2::PooledConnection<ConnectionManager<PgConnection>>;

/// Establishes a connection pool to the `PostgreSQL` database
///
/// # Panics
///
/// This function will panic if:
/// - The `DATABASE_URL` environment variable is not set
/// - The connection pool cannot be created
///
/// # Examples
///
/// ```no_run
/// use cto_parallel_test::config::db::establish_connection_pool;
///
/// let pool = establish_connection_pool();
/// ```
#[must_use]
pub fn establish_connection_pool() -> Pool {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let manager = ConnectionManager::<PgConnection>::new(database_url);
    r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool")
}
