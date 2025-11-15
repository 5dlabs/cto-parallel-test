Task 1: Diesel/Postgres schema + security hardening (cto-parallel-test)

Summary
- Implemented Diesel ORM + PostgreSQL database layer
- Added integrity constraints and case-insensitive uniqueness for users
- Verified with migrations, fmt, clippy, tests
- Performed local security scans (cargo-audit, gitleaks)

Security Changes
- CHECK constraints: username/email/product name lengths
- UNIQUE indexes on LOWER(username/email) to prevent case-only duplicates
- Existing constraints retained: NUMERIC price, non-negative amounts, FK cascades, cart item uniqueness

Quality Gates
- cargo fmt --all -- --check: pass
- cargo clippy --workspace --all-targets --all-features -- -D warnings -W clippy::pedantic: pass
- cargo test --workspace --all-features: pass
- diesel migration redo: pass
- cargo-audit: clean
- gitleaks (working tree only): no leaks

GitHub Code Scanning / PR (auth blocked here)
The environment lacked a GH_TOKEN for GitHub CLI. Use the exact commands below locally/CI:

1) Ensure token is available to gh and curl
   export GH_TOKEN=<github_app_token>

2) Push branch (already pushed by the agent)
   git checkout feature/task-1-implementation
   git push -u origin feature/task-1-implementation

3) Create PR with required labels
   gh pr create \
     --title "Task 1: Diesel/Postgres schema + security hardening" \
     --body-file docs/run-logs/task-1-gh7g9-complete.md \
     --label task-1 \
     --label service-cto-parallel-test \
     --label run-play-task-1-gh7g9

4) Discover PR number for branch and check code scanning alerts
   PR_NUMBER=$(gh pr list --head feature/task-1-implementation --json number -q '.[0].number')
   OWNER_REPO=5dlabs/cto-parallel-test

   gh api -H "Authorization: Bearer ${GH_TOKEN}" \
     "/repos/${OWNER_REPO}/code-scanning/alerts?state=open&pr=${PR_NUMBER}"

   # Fallback to curl if needed
   curl -sfL -H "Authorization: Bearer ${GH_TOKEN}" -H "Accept: application/vnd.github+json" \
     "https://api.github.com/repos/${OWNER_REPO}/code-scanning/alerts?state=open&pr=${PR_NUMBER}" | jq

Notes
- No MEDIUM/HIGH/CRITICAL issues found in local scans. CI CodeQL and cargo-audit will re-verify on PR.
- All configs and endpoints are env-driven. No hardcoded secrets committed.

