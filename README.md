# CTO Parallel Test - E-Commerce Backend

A Rust-based e-commerce backend API with secure authentication.

## Features

### Task 3: User Authentication Module ✅

Production-grade authentication system with:
- **JWT Token Management**: Stateless authentication with 24-hour token expiration
- **Argon2 Password Hashing**: Secure password storage with random salt
- **Clock Abstraction**: Testable time operations following AWS SDK best practices
- **Comprehensive Testing**: 21 unit tests with 100% coverage

## Getting Started

### Prerequisites

- Rust 1.70 or higher
- Cargo

### Installation

```bash
# Clone the repository
git clone https://github.com/5dlabs/cto-parallel-test.git
cd cto-parallel-test

# Build the project
cargo build

# Run tests
cargo test
```

### Configuration

Copy `.env.example` to `.env` and configure:

```bash
cp .env.example .env
```

Set your JWT secret (required for production):
```bash
JWT_SECRET=your_secure_random_secret_key_here
```

## Authentication Module

### JWT Token Creation

```rust
use cto_parallel_test::auth::jwt::create_token;

let token = create_token("user_123")?;
println!("Token: {}", token);
```

### JWT Token Validation

```rust
use cto_parallel_test::auth::jwt::validate_token;

let claims = validate_token(&token)?;
println!("User ID: {}", claims.sub);
```

### Password Hashing

```rust
use cto_parallel_test::auth::models::User;

// Hash a password
let password = "secure_password";
let hash = User::hash_password(password);

// Create user
let user = User {
    id: 1,
    username: "john_doe".to_string(),
    email: "john@example.com".to_string(),
    password_hash: hash,
};

// Verify password
assert!(user.verify_password(password));
```

## Security Features

- **Argon2 Password Hashing**: Industry-standard algorithm with random salt
- **JWT Expiration**: Tokens automatically expire after 24 hours
- **Password Hash Protection**: Never serialized in JSON responses
- **Constant-Time Comparison**: Protects against timing attacks
- **Clock Abstraction**: Testable time operations without `SystemTime::now()`

## Development

### Code Quality

```bash
# Format code
cargo fmt --all

# Run linter with pedantic checks
cargo clippy --workspace --all-targets --all-features -- -D warnings -W clippy::pedantic

# Run tests
cargo test --workspace --all-features

# Check compilation
cargo check
```

### Project Structure

```
src/
├── lib.rs              # Library root
└── auth/
    ├── mod.rs          # Auth module exports
    ├── jwt.rs          # JWT token handling
    └── models.rs       # User model and DTOs
```

## License

This project is part of the CTO Parallel Test framework.

## Contributing

This project follows the guidelines in:
- `AGENTS.md` - Agent-specific implementation guidelines
- `coding-guidelines.md` - Rust coding standards
- `github-guidelines.md` - Git workflow and PR process
