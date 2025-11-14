#!/usr/bin/env bash
set -euo pipefail

# Query GitHub Code Scanning alerts for the current feature branch PR.
# Requires: gh CLI authenticated (gh auth login -h github.com)

REPO_SLUG="5dlabs/cto-parallel-test"
BRANCH="${1:-$(git rev-parse --abbrev-ref HEAD)}"

echo "Repo: $REPO_SLUG" >&2
echo "Branch: $BRANCH" >&2

if ! gh auth status >/dev/null 2>&1; then
  echo "ERROR: gh CLI not authenticated for github.com" >&2
  echo "Run: gh auth login -h github.com" >&2
  exit 2
fi

PR_NUM=$(gh pr list --repo "$REPO_SLUG" --head "$BRANCH" --json number -q '.[0].number')
if [[ -z "${PR_NUM:-}" ]]; then
  echo "No PR found for branch $BRANCH in $REPO_SLUG" >&2
  exit 3
fi

echo "PR: #$PR_NUM" >&2

gh api \
  "/repos/$REPO_SLUG/code-scanning/alerts?state=open&pr=$PR_NUM" \
  --jq '.[] | {rule: .rule.id, severity: .rule.severity, path: .most_recent_instance.location.path, start: .most_recent_instance.location.start_line}'

