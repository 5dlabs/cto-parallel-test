use diesel::pg::PgConnection;
use diesel::r2d2::{self, ConnectionManager};
use dotenvy::dotenv;
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
        .expect("DATABASE_URL must be set; refer to docs/db-setup.md for format guidance");

    // Optional pool configuration via env vars (secure defaults)
    // Clamp pool size to a sane, non-zero range to prevent DoS via misconfig
    let max_size: u32 = env::var("DB_POOL_MAX_SIZE")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(15)
        .clamp(1, 100);

    let min_idle: Option<u32> = env::var("DB_POOL_MIN_IDLE")
        .ok()
        .and_then(|v| v.parse().ok());

    // Enforce a reasonable timeout window [1s, 300s]
    let conn_timeout_secs: u64 = env::var("DB_POOL_CONNECTION_TIMEOUT_SECS")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(30)
        .clamp(1, 300);
    let conn_timeout = Duration::from_secs(conn_timeout_secs);

    let max_lifetime = env::var("DB_POOL_MAX_LIFETIME_SECS")
        .ok()
        .and_then(|v| v.parse::<u64>().ok())
        .map(Duration::from_secs);

    let idle_timeout = env::var("DB_POOL_IDLE_TIMEOUT_SECS")
        .ok()
        .and_then(|v| v.parse::<u64>().ok())
        .map(Duration::from_secs);

    let test_on_check_out = env::var("DB_POOL_TEST_ON_CHECK_OUT").ok().and_then(|v| {
        let normalized = v.trim().to_ascii_lowercase();
        match normalized.as_str() {
            "1" | "true" | "yes" | "on" => Some(true),
            "0" | "false" | "no" | "off" => Some(false),
            _ => None,
        }
    });

    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let mut builder = r2d2::Pool::builder()
        .max_size(max_size)
        .connection_timeout(conn_timeout);

    if let Some(min_idle) = min_idle {
        // Ensure min_idle does not exceed max_size
        builder = builder.min_idle(Some(min_idle.min(max_size)));
    }

    if let Some(max_lifetime) = max_lifetime {
        builder = builder.max_lifetime(Some(max_lifetime));
    }

    if let Some(idle_timeout) = idle_timeout {
        builder = builder.idle_timeout(Some(idle_timeout));
    }

    if let Some(test_on_check_out) = test_on_check_out {
        builder = builder.test_on_check_out(test_on_check_out);
    }

    builder.build(manager).expect("Failed to create pool")
}
