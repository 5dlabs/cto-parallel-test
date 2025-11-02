use diesel::pg::PgConnection;
use diesel::r2d2::{self, ConnectionManager};
use dotenv::dotenv;
use std::env;

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;
pub type DbConnection = r2d2::PooledConnection<ConnectionManager<PgConnection>>;

/// Establishes a connection pool to the PostgreSQL database.
///
/// # Panics
///
/// Panics if DATABASE_URL environment variable is not set or if pool creation fails.
pub fn establish_connection_pool() -> Pool {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let manager = ConnectionManager::<PgConnection>::new(database_url);
    r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool")
}
