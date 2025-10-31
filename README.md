# CTO Parallel Test - Task Documentation

This repository contains TaskMaster documentation for the CTO parallel execution test project.

## Structure

- `docs/.taskmaster/tasks/` - Task definitions and tasks.json
- `docs/.taskmaster/docs/task-X/` - Detailed documentation for each task
  - `task.txt` - Task overview
  - `task.md` - Detailed task description
  - `prompt.md` - Agent instructions
  - `acceptance-criteria.md` - Success criteria
  - `task.xml` - Structured XML prompt

## Purpose

This documentation is used by the CTO platform to:
- Parse task structure and dependencies
- Generate execution levels for parallel processing
- Provide agents with implementation guidance
- Validate task completion

## Usage

This repository is referenced by CTO platform workflows during:
- Task intake and parsing
- Parallel task execution
- Agent task assignment
- Integration verification

## Auth Module (Task 3)

Implemented a minimal authentication module for the test API:
- JWT handling in `src/auth/jwt.rs` with 24h expiry (HS256, test-only secret)
  - Security: JWT signing key is no longer hardcoded. Provide `JWT_SECRET` in the environment (minimum 32 bytes) for non-test builds.
- User model and Argon2 password utilities in `src/auth/models.rs`
- Re-exports via `src/auth/mod.rs` for `create_token`, `validate_token`, and `User`

Example usage:

```
use cto_parallel_test::auth::{create_token, validate_token, User};

let hash = User::hash_password("secret");
let user = User { id: 1, username: "alice".into(), email: "a@example.com".into(), password_hash: hash };
assert!(user.verify_password("secret"));

// Set a sufficiently long secret in your environment before using JWTs
// export JWT_SECRET="your-32+byte-random-secret-here"

let token = create_token("1").expect("token");
let claims = validate_token(&token).expect("claims");
assert_eq!(claims.sub, "1");
```
