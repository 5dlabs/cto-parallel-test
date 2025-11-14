use diesel::pg::PgConnection;
use diesel::r2d2::{self, ConnectionManager};
use dotenv::dotenv;
use std::env;
use std::time::Duration;

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;
pub type DbConnection = r2d2::PooledConnection<ConnectionManager<PgConnection>>;

/// Establishes a connection pool to the `PostgreSQL` database.
///
/// This function loads environment variables from a `.env` file and reads the
/// `DATABASE_URL` to create a connection pool using r2d2.
///
/// # Panics
///
/// Panics if `DATABASE_URL` is not set or if the pool cannot be created.
#[must_use]
pub fn establish_connection_pool() -> Pool {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set (e.g., postgres://user:pass@host:5432/db)");

    // Optional pool configuration via env vars (secure defaults)
    let max_size: u32 = env::var("DB_POOL_MAX_SIZE")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(15);

    let min_idle: Option<u32> = env::var("DB_POOL_MIN_IDLE")
        .ok()
        .and_then(|v| v.parse().ok());

    let conn_timeout = Duration::from_secs(
        env::var("DB_POOL_CONNECTION_TIMEOUT_SECS")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(30),
    );

    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let mut builder = r2d2::Pool::builder()
        .max_size(max_size)
        .connection_timeout(conn_timeout);

    if let Some(min_idle) = min_idle {
        builder = builder.min_idle(Some(min_idle));
    }

    builder.build(manager).expect("Failed to create pool")
}
