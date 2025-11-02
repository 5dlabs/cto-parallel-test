use cto_parallel_test::config::db::establish_connection_pool;
use cto_parallel_test::schema::{cart_items, carts, products, users};
use diesel::prelude::*;

#[test]
fn test_connection_pool() {
    // This test verifies that we can create a database connection pool
    let pool = establish_connection_pool();
    let mut conn = pool.get().expect("Failed to get connection from pool");

    // Simple query to verify connection works
    let result = diesel::sql_query("SELECT 1 as value")
        .execute(&mut conn)
        .expect("Failed to execute query");

    // Result should be successful (1 row affected)
    assert_eq!(result, 1);
}

#[test]
fn test_schema_tables_exist() {
    // Verify all tables exist in the database
    let pool = establish_connection_pool();
    let mut conn = pool.get().expect("Failed to get connection from pool");

    // Check that we can query each table (even if empty)
    let users_count: i64 = users::table
        .count()
        .get_result(&mut conn)
        .expect("Failed to query users table");

    let products_count: i64 = products::table
        .count()
        .get_result(&mut conn)
        .expect("Failed to query products table");

    let carts_count: i64 = carts::table
        .count()
        .get_result(&mut conn)
        .expect("Failed to query carts table");

    let cart_items_count: i64 = cart_items::table
        .count()
        .get_result(&mut conn)
        .expect("Failed to query cart_items table");

    // All tables should be accessible (count >= 0)
    assert!(users_count >= 0);
    assert!(products_count >= 0);
    assert!(carts_count >= 0);
    assert!(cart_items_count >= 0);
}
