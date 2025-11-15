#!/usr/bin/env bash
set -euo pipefail

# Post a PR comment with current Code Scanning alerts for the PR of a branch.
# Usage:
#   bash task/gh-pr-comment-scan.sh [branch]
# Defaults to current branch if not provided.

REPO_SLUG="5dlabs/cto-parallel-test"
BRANCH="${1:-$(git rev-parse --abbrev-ref HEAD)}"

if ! command -v gh >/dev/null 2>&1; then
  echo "ERROR: gh CLI is not installed." >&2
  exit 1
fi

if ! gh auth status >/dev/null 2>&1; then
  echo "ERROR: gh CLI not authenticated for github.com" >&2
  echo "Run: gh auth login -h github.com" >&2
  exit 2
fi

echo "Repo: $REPO_SLUG" >&2
echo "Branch: $BRANCH" >&2

PR_NUM=$(gh pr list --repo "$REPO_SLUG" --head "$BRANCH" --json number -q '.[0].number')
if [[ -z "${PR_NUM:-}" ]]; then
  echo "No PR found for branch $BRANCH in $REPO_SLUG" >&2
  exit 3
fi
echo "PR: #$PR_NUM" >&2

TMP_JSON=$(mktemp)
TMP_MD=$(mktemp)
trap 'rm -f "$TMP_JSON" "$TMP_MD"' EXIT

# Fetch alerts as JSON
gh api \
  "/repos/$REPO_SLUG/code-scanning/alerts?state=open&pr=$PR_NUM" \
  > "$TMP_JSON"

TOTAL=$(jq 'length' "$TMP_JSON")
DATE_ISO=$(date -u +"%Y-%m-%dT%H:%M:%SZ")

{
  echo "## Code Scanning Summary ($DATE_ISO UTC)"
  if [[ "$TOTAL" -eq 0 ]]; then
    echo "\nNo open code scanning alerts for this PR. ✅"
  else
    echo "\nOpen alerts: $TOTAL\n"
    echo "| Rule | Severity | Path | Line |"
    echo "| ---- | -------- | ---- | ---- |"
    jq -r '.[] | "| \(.rule.id) | \(.rule.severity) | \(.most_recent_instance.location.path) | \(.most_recent_instance.location.start_line) |"' "$TMP_JSON"
  fi
} > "$TMP_MD"

gh pr comment "$PR_NUM" --repo "$REPO_SLUG" --body-file "$TMP_MD"

echo "Posted code scanning summary comment to PR #$PR_NUM" >&2

