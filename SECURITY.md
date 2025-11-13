# Security Policy

We take the security of our users and their data seriously. Please follow the guidance below when reporting a vulnerability or handling secrets.

## Supported Versions

This repository is continuously delivered from the default branch. Unless otherwise stated in release notes, we support the latest commit on the default branch and the latest tagged release. Older releases are not maintained unless explicitly noted.

## Reporting a Vulnerability

- Please report vulnerabilities privately via GitHub Security Advisories (Security > Advisories > Report a vulnerability) or email security@5dlabs.io.
- Provide detailed steps to reproduce, the impacted files/paths, and any proof of exploitability.
- We aim to acknowledge new reports within 72 hours and provide status updates weekly until resolution.

## Handling Secrets

- Never commit secrets. Use environment variables or secret managers.
- Client applications must not store tokens in localStorage/sessionStorage. Prefer httpOnly cookies managed by the backend.
- Review `coding-guidelines.md` for additional security requirements and `github-guidelines.md` for PR and branch policies.

## Frontend Security Controls

- Content Security Policy (CSP) is enforced in `frontend/index.html` to mitigate XSS. Scripts are restricted to `self`; styles allow `'unsafe-inline'` only to support Tailwind during dev. Images and fonts are limited to `self` and `data:`.
- All API endpoints are parameterized via environment (`VITE_API_BASE_URL`). Default is a relative path (`/api`) to enable reverse-proxying and keep a strict CSP.
- Route parameters and query strings are encoded with `encodeURIComponent` before use to prevent injection into URLs/paths.
- No credentials or tokens are stored client-side. Authentication assumes server-managed httpOnly cookies.

## Code Scanning Process

- GitHub Code Scanning alerts are queried for the current PR via `tooling/gh-code-scanning.sh` which calls `gh api "/repos/<owner>/<repo>/code-scanning/alerts?state=open&pr=<number>"`.
- All MEDIUM, HIGH, and CRITICAL alerts are treated as blockers. Fixes must address root causes; suppression is not allowed.
- The helper `tooling/pr-and-scan.sh` parameterizes labels and body via env vars (`PR_LABELS`, `PR_BODY_FILE`, `PR_TITLE`) and creates/updates a PR before scanning.
