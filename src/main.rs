// Make modules public for testing
pub mod api;
pub mod auth;
pub mod cart;
pub mod catalog;
pub mod schema;

#[allow(clippy::disallowed_macros)]
fn main() {
    println!("E-commerce API server");
    println!("Modules: auth, catalog, cart, api");
}
