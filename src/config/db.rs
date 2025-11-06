use diesel::pg::PgConnection;
use diesel::r2d2::{self, ConnectionManager, PoolError};
use dotenv::dotenv;
use std::env;

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;
pub type DbConnection = r2d2::PooledConnection<ConnectionManager<PgConnection>>;

/// Error type for database connection pool initialization.
#[derive(Debug)]
pub enum ConnectionPoolError {
    /// The `DATABASE_URL` environment variable is not set.
    MissingDatabaseUrl(env::VarError),
    /// Failed to create the connection pool.
    PoolCreationFailed(PoolError),
}

impl std::fmt::Display for ConnectionPoolError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MissingDatabaseUrl(e) => {
                write!(f, "DATABASE_URL environment variable not set: {e}")
            }
            Self::PoolCreationFailed(e) => write!(f, "Failed to create connection pool: {e}"),
        }
    }
}

impl std::error::Error for ConnectionPoolError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::MissingDatabaseUrl(e) => Some(e),
            Self::PoolCreationFailed(e) => Some(e),
        }
    }
}

/// Establishes a connection pool to the `PostgreSQL` database.
///
/// # Errors
///
/// This function will return an error if:
/// - The `DATABASE_URL` environment variable is not set
/// - The connection pool cannot be created
///
/// # Examples
///
/// ```no_run
/// use ecommerce_api::config::db::establish_connection_pool;
///
/// let pool = establish_connection_pool().expect("Failed to create pool");
/// ```
pub fn establish_connection_pool() -> Result<Pool, ConnectionPoolError> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").map_err(ConnectionPoolError::MissingDatabaseUrl)?;

    let manager = ConnectionManager::<PgConnection>::new(database_url);
    r2d2::Pool::builder()
        .build(manager)
        .map_err(ConnectionPoolError::PoolCreationFailed)
}
