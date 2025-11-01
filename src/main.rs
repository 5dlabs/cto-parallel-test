use ecommerce_api::establish_connection_pool;
use tracing::{error, info};

fn main() {
    // Initialize tracing subscriber with default configuration
    tracing_subscriber::fmt::init();

    info!("E-commerce API - Database Schema Setup");

    // Test database connection
    if let Err(()) = std::panic::catch_unwind(|| {
        let pool = establish_connection_pool();
        info!("✓ Database connection pool established successfully");
        info!("✓ Pool size: {:?}", pool.max_size());
    })
    .map_err(|_| ())
    {
        error!("✗ Failed to establish database connection");
        error!("  Make sure DATABASE_URL is set and PostgreSQL is running");
        std::process::exit(1);
    }

    info!("✓ All database checks passed");
}
