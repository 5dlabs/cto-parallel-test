Task 1 – Attempt 2 Security Fix (cto-parallel-test)

Summary
- Replaced unmaintained dotenv crate with dotenvy to satisfy cargo-audit and security best practices (RUSTSEC-2021-0141).
- Re-validated local quality gates and security scans; all pass.
- Pushed changes to feature/task-1-implementation.

Local Quality Gates
- cargo fmt --all -- --check: pass
- cargo clippy --workspace --all-targets --all-features -- -D warnings -W clippy::pedantic: pass
- cargo test --workspace --all-features: pass
- cargo audit: pass (no advisories)
- gitleaks detect --no-banner --no-git --source .: pass (no leaks)

GitHub Code Scanning
- gh CLI not authenticated in this environment; alerts query is blocked locally.
- To check open alerts for the current PR once GH_TOKEN is available:
  export GH_TOKEN=<app_token>
  export GITHUB_TOKEN="$GH_TOKEN"
  BRANCH=$(git rev-parse --abbrev-ref HEAD)
  PR=$(gh pr list --head "$BRANCH" --json number -q '.[0].number')
  gh api "/repos/5dlabs/cto-parallel-test/code-scanning/alerts?state=open&pr=${PR}" | jq

PR Creation (labels)
- Branch: feature/task-1-implementation
- Create PR with labels task-1, service-cto-parallel-test, run-play-task-1-gh7g9:
  gh pr create \
    --title "feat: Task 1 – Diesel/Postgres DB layer, migrations, models, pool" \
    --body-file docs/run-logs/task-1-attempt-2-sec-fix.md \
    --base main --head feature/task-1-implementation \
    --label task-1 --label service-cto-parallel-test --label run-play-task-1-gh7g9

Notes
- dotenvy is a maintained drop-in replacement for dotenv’s env loading.
- No changes to runtime behavior other than using the maintained crate.

