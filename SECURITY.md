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
