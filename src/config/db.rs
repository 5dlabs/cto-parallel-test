use diesel::pg::PgConnection;
use diesel::r2d2::{self, ConnectionManager};
use dotenv::dotenv;
use std::env;

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;
pub type DbConnection = r2d2::PooledConnection<ConnectionManager<PgConnection>>;

/// Establishes a connection pool to the `PostgreSQL` database.
///
/// This function reads the `DATABASE_URL` environment variable from the `.env` file
/// and creates a connection pool using r2d2.
///
/// # Panics
///
/// Panics if:
/// - `DATABASE_URL` environment variable is not set
/// - Connection pool creation fails
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

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env file");

    let manager = ConnectionManager::<PgConnection>::new(database_url);
    r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create database connection pool")
}
