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

## CI Scanning

The repository includes automated security scanning in GitHub Actions:

- Gitleaks: Scans for committed secrets on push/PR to `main` and weekly. Results are uploaded as SARIF to GitHub code scanning. Local runs generate `gitleaks.sarif` which is ignored via `.gitignore`. Configuration in `.gitleaks.toml` allowlists known test-only patterns. Fails CI on findings to block merges.
- Cargo Audit: Checks Rust dependencies for known vulnerabilities on push/PR to `main` and weekly using `rustsec/audit-check`. Fails CI on advisories to block merges.
- CodeQL: Runs code scanning for supported languages on push/PR to `main` and weekly. Only initializes if supported languages are detected.
- Code Scanning Gate: Enforces zero open MEDIUM/HIGH/CRITICAL code scanning alerts on PRs to `main` using the GitHub CLI (`gh api`). If any such alerts exist for the PR, the workflow fails to block merging. The gate prefers `rule.security_severity_level` when present and maps legacy severities (`error`→`high`, `warning`→`medium`, `note`→`low`) to ensure strict blocking.

Note: Scanners are configured to fail CI on findings to block merges by default. If you prefer to surface results without blocking, add `continue-on-error: true` to the relevant steps and adjust branch protection accordingly.

### Branch protection (recommended)

To enforce blocking in GitHub, enable required status checks on the `main` branch for:

- `Gitleaks / gitleaks scan`
- `Cargo Audit / cargo audit`
- `Code Scanning Gate / Enforce Code Scanning Gate`

In GitHub: Settings → Branches → Branch protection rules → Edit `main` → Require status checks → search and select the checks above.

## Local Code Scanning Gate

To check GitHub Code Scanning alerts for the current PR locally and fail on any MEDIUM/HIGH/CRITICAL severities, use:

`scripts/codescan-gate.sh`

Requirements: `gh` (authenticated) and `jq`.
