// Allow println/eprintln in main for simple database verification utility
// This is a lightweight database setup check, not production logging
#![allow(clippy::disallowed_macros)]

use ecommerce_api::establish_connection_pool;

fn main() {
    println!("E-commerce API - Database Schema Setup");

    // Test database connection
    if let Err(()) = std::panic::catch_unwind(|| {
        let pool = establish_connection_pool();
        println!("✓ Database connection pool established successfully");
        println!("✓ Pool size: {:?}", pool.max_size());
    })
    .map_err(|_| ())
    {
        eprintln!("✗ Failed to establish database connection");
        eprintln!("  Make sure DATABASE_URL is set and PostgreSQL is running");
        std::process::exit(1);
    }

    println!("✓ All database checks passed");
}
