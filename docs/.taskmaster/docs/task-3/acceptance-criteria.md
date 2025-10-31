# Acceptance Criteria: Task 3 - User Authentication Module

## Must Have
- [ ] Dependencies: jsonwebtoken 8.3.0, argon2 0.5.0, rand 0.8.5 added
- [ ] src/auth/mod.rs exports jwt and models modules
- [ ] src/auth/jwt.rs implements create_token and validate_token
- [ ] Claims struct with sub, exp, iat fields
- [ ] Tokens expire after 24 hours
- [ ] src/auth/models.rs implements User struct
- [ ] User.verify_password() validates passwords
- [ ] User::hash_password() uses Argon2 with random salt
- [ ] password_hash field excluded from serialization
- [ ] cargo check passes without warnings
- [ ] Unit tests for token creation/validation pass
- [ ] Unit tests for password hashing/verification pass

## Validation Commands
```bash
cargo test auth::jwt::tests
cargo test auth::models::tests
```

## Definition of Done
- ✅ All tests pass
- ✅ JWT tokens work for authentication
- ✅ Passwords securely hashed
- ✅ Ready for Task 5 integration
