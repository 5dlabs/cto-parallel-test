# Security Overview

This repository enforces secure defaults and integrates multiple scanners to prevent security regressions.

- Argon2 (v0.5) is used for password hashing with per-password random salts.
- JWT tokens require a strong secret via `JWT_SECRET` and include `sub`, `iat`, and `exp` claims.
- Token lifetime defaults to 24 hours and can be overridden using `JWT_EXP_HOURS`.
- No hardcoded credentials or secrets are committed to the repository.
- CI runs CodeQL, gitleaks, and cargo-audit on pull requests.

## Required Environment Variables

- `JWT_SECRET` (required): HMAC secret for signing/validating JWTs. Must be at least 32 bytes.
- `JWT_EXP_HOURS` (optional): Token expiration window in hours (default `24`).

## Threat Model Considerations

- Password verification uses constant-time operations via the Argon2 crate.
- Tokens are validated for signature and expiration; tampered or expired tokens are rejected.
- Environment access is limited to the specific variables above; no fallback secrets are used.

## Development Notes

- Never commit real secrets. Use `.env.example` as a template and set real values in your environment or secret manager.
- If you need to test locally, export `JWT_SECRET` to a sufficiently long random value.

