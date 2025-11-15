Task 1 Verification: Diesel/Postgres DB layer, security scans, and CI

Summary
- Verified Diesel ORM + PostgreSQL setup is complete: migrations, `src/schema.rs`, `src/models.rs`, and r2d2 pool in `src/config/db.rs`.
- Confirmed parameterization via environment variables; no hardcoded secrets. `.env` is gitignored; `.env.example` provided.
- Ensured secure DB constraints (non-negative price/inventory, positive cart item quantities, and unique cart line constraint).
- CI enforces formatting, clippy (pedantic), tests, cargo-audit, gitleaks (working tree), and CodeQL.

Local Quality Gates (all passed)
- cargo fmt --all -- --check
- cargo clippy --workspace --all-targets --all-features -- -D warnings -W clippy::pedantic
- cargo test --workspace --all-features
- cargo audit (no advisories)
- gitleaks detect --no-git (no leaks in working tree)

GitHub Code Scanning
- This environment does not have an authenticated GitHub CLI session. To check open Code Scanning alerts for the PR from your machine or CI runner:

```
export GH_TOKEN=<github_app_or_pat_token>
BRANCH=$(git rev-parse --abbrev-ref HEAD)
PR=$(gh pr list --head "$BRANCH" --json number -q '.[0].number')
gh api "/repos/5dlabs/cto-parallel-test/code-scanning/alerts?state=open&pr=${PR}" | jq
```

Pull Request Creation (labels required)
- Create PR from `feature/task-1-implementation` and apply labels `task-1`, `service-cto-parallel-test`, and `run-play-task-1-qwt52`:

```
export GH_TOKEN=<github_app_or_pat_token>
git push -u origin feature/task-1-implementation
gh pr create \
  --title "feat(db): Diesel/Postgres schema, models, pooling (Task 1)" \
  --body "Implements Diesel/Postgres schema, models, pooling. Passes fmt, clippy (pedantic), tests; cargo-audit and gitleaks clean. CI includes CodeQL and security scans." \
  --label task-1 \
  --label service-cto-parallel-test \
  --label run-play-task-1-qwt52
```

Status
- Security work complete: zero MEDIUM/HIGH/CRITICAL findings in local scans, quality gates passing, CI security scanning configured.

