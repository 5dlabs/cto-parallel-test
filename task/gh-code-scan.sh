#!/usr/bin/env bash
set -euo pipefail

# Query GitHub Code Scanning alerts for the current feature branch PR.
# Requires: gh CLI authenticated (gh auth login -h github.com)
# Usage:
#   bash task/gh-code-scan.sh [branch] [--update-docs]
# Env:
#   UPDATE_DOCS=1  # same effect as passing --update-docs

REPO_SLUG="5dlabs/cto-parallel-test"
BRANCH="${1:-$(git rev-parse --abbrev-ref HEAD)}"
UPDATE_FLAG="${2:-}"

UPDATE_DOCS=${UPDATE_DOCS:-0}
if [[ "$UPDATE_FLAG" == "--update-docs" ]]; then
  UPDATE_DOCS=1
fi

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

# Fetch alerts
ALERTS_JSON=$(gh api \
  "/repos/$REPO_SLUG/code-scanning/alerts?state=open&pr=$PR_NUM" \
  --jq '.')

# Print concise list to stdout for CLI users
echo "$ALERTS_JSON" | jq -r '.[] | {rule: .rule.id, severity: .rule.severity, path: .most_recent_instance.location.path, start: .most_recent_instance.location.start_line}' 2>/dev/null || echo "$ALERTS_JSON"

# Optionally update docs with a snapshot
if [[ "$UPDATE_DOCS" == "1" ]]; then
  DOC="docs/task-6-security-review.md"
  TS=$(date -u +"%Y-%m-%dT%H:%M:%SZ")
  mkdir -p "$(dirname "$DOC")"

  {
    echo "\n## GitHub Code Scanning Snapshot ($TS)"
    echo
    echo "Branch: \`$BRANCH\`  PR: \`#$PR_NUM\`"
    COUNT=$(echo "$ALERTS_JSON" | jq 'length' 2>/dev/null || echo 0)
    if [[ "$COUNT" == "0" ]]; then
      echo "\nNo open code scanning alerts for this PR."
    else
      echo "\nOpen alerts: $COUNT"
      echo
      echo '```json'
      echo "$ALERTS_JSON"
      echo '```'
    fi
  } >> "$DOC"

  echo "Updated $DOC with code scanning snapshot ($COUNT alerts)." >&2
fi
