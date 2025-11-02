# Security Overview

This repository enforces secure defaults and follows best practices outlined in our coding and GitHub guidelines.

## Authentication

- Password hashing uses Argon2 with a unique random salt per password.
- JWT tokens require a strong secret via `JWT_SECRET` (minimum 32 bytes).
- Tokens include `sub`, `iat`, and `exp` claims; expiration defaults to 24 hours.
- Validation applies a small, configurable leeway (`JWT_LEEWAY_SECS`) to tolerate minor clock skew.

## Configuration

- `JWT_SECRET` (required): HMAC secret used to sign/validate tokens.
- `JWT_EXP_HOURS` (optional): Token lifetime in hours (default: 24).
- `JWT_LEEWAY_SECS` (optional): Validation leeway in seconds (default: 60, max: 300).

## Threat Model Considerations

- Password verification uses constant-time operations via the Argon2 crate.
- Tokens are validated for signature and expiration; tampered or expired tokens are rejected.
- No hardcoded credentials or secrets exist in the codebase.

## Development Notes

- Never commit real secrets. Use environment variables or a secret manager.
- Unit tests generate random secrets and serialize environment changes under a global test lock to prevent races.

