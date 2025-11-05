pub mod catalog;

use catalog::ProductService;

fn main() {
    println!("Hello, world!");

    // Example usage of the product catalog
    let _catalog = ProductService::new();
    println!("Product catalog initialized successfully!");
}
