// This is a minimal binary stub for Docker health checks
// In production, this would use a proper web framework with tracing
#![allow(clippy::disallowed_macros)] // Allow println! for simple binary stub

use std::io::Write;

/// Simple placeholder main for authentication module
///
/// This binary provides a basic health check endpoint for Docker deployments.
/// In production, this would integrate with a full web framework (Actix-web, etc.)
fn main() {
    println!("User Authentication Module - Running");
    println!("Health endpoint available on port 8080");
    println!("This is a library-focused module - see lib.rs for core functionality");

    // Simple HTTP server simulation for Docker health check
    let listener =
        std::net::TcpListener::bind("0.0.0.0:8080").expect("Failed to bind to port 8080");

    println!("Listening on 0.0.0.0:8080");

    for mut stream in listener.incoming().flatten() {
        // Read request (we don't parse it, just respond with 200 OK)
        let mut buffer = [0; 1024];
        let _ = stream.peek(&mut buffer);

        // Simple HTTP response
        let response = "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: 2\r\n\r\nOK";
        let _ = stream.write_all(response.as_bytes());
    }
}
