#!/usr/bin/env bash
set -euo pipefail

# List GitHub Code Scanning alerts for the current PR or a provided PR number.
# Usage:
#   scripts/list_code_scanning.sh              # infers PR from current branch
#   scripts/list_code_scanning.sh <PR_NUMBER>  # explicit PR number
#
# Requires: gh CLI authenticated with a token having repo + security_events

OWNER="5dlabs"
REPO="cto-parallel-test"

if ! command -v gh >/dev/null 2>&1; then
  echo "GitHub CLI (gh) is required. Install from https://cli.github.com/" >&2
  exit 1
fi

PR_NUM="${1:-}"
if [[ -z "$PR_NUM" ]]; then
  BRANCH=$(git rev-parse --abbrev-ref HEAD)
  PR_NUM=$(gh pr list --head "$BRANCH" --json number --jq '.[0].number' 2>/dev/null || true)
fi

if [[ -z "${PR_NUM:-}" ]]; then
  echo "Unable to determine PR number. Provide it explicitly: scripts/list_code_scanning.sh <PR_NUMBER>" >&2
  exit 1
fi

echo "Checking GitHub auth..."
if ! gh auth status >/dev/null 2>&1; then
  if [[ -n "${GITHUB_TOKEN:-}" || -n "${GH_TOKEN:-}" ]]; then
    echo "Attempting GitHub CLI auth using provided token..."
    TOKEN_TO_USE="${GITHUB_TOKEN:-${GH_TOKEN:-}}"
    gh auth login --hostname github.com --with-token < <(echo "$TOKEN_TO_USE") >/dev/null
  else
    echo "Not authenticated. Export GITHUB_TOKEN (repo+security_events) and retry." >&2
    exit 1
  fi
fi

mkdir -p .reports
OUT_FILE=".reports/code-scanning-PR-${PR_NUM}.json"
echo "Fetching Code Scanning alerts for PR #${PR_NUM}..."

gh api \
  "/repos/${OWNER}/${REPO}/code-scanning/alerts?state=open&pr=${PR_NUM}" \
  --jq '.' > "$OUT_FILE"

COUNT=$(jq 'length' "$OUT_FILE" 2>/dev/null || echo 0)
if [[ "$COUNT" -eq 0 ]]; then
  echo "No open Code Scanning alerts for PR #${PR_NUM}."
else
  echo "Open Code Scanning alerts (${COUNT}):"
  jq -r 'map("- [" + .rule.severity + "] " + .rule.id + " (" + .state + ")")[]' "$OUT_FILE"
fi

echo "Saved full response to $OUT_FILE"

