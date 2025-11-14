#!/usr/bin/env bash
set -euo pipefail

# Fetch open GitHub Code Scanning alerts for the current PR
# Outputs:
#  - .reports/code_scanning_alerts.json
#  - .reports/code_scanning_alerts_summary.json (severity counts)

if ! command -v gh >/dev/null 2>&1; then
  echo "gh CLI not found. Install from https://github.com/cli/cli/releases" >&2
  exit 1
fi

if ! gh auth status -h github.com >/dev/null 2>&1; then
  echo "GitHub auth required. Authenticate with one of:" >&2
  echo '  export GH_TOKEN=... && echo "$GH_TOKEN" | gh auth login --hostname github.com --with-token' >&2
  echo '  gh auth login --hostname github.com --web' >&2
  exit 2
fi

mkdir -p .reports

# Derive owner/repo from git remote
OWNER_REPO=$(git remote get-url origin | sed -E 's#.*github.com[:/](.+/[^/.]+)(\\.git)?$#\1#')

PR_NUMBER="${PR_NUMBER:-}"
if [ -z "$PR_NUMBER" ]; then
  # Try to infer PR number from current context
  if PR_NUMBER=$(gh pr view --json number -q .number 2>/dev/null); then
    :
  else
    BRANCH=$(git branch --show-current)
    PR_NUMBER=$(gh pr list --state open --json number,headRefName \
      -q "map(select(.headRefName==\"$BRANCH\")) | .[0].number" 2>/dev/null || echo "")
  fi
fi

if [ -z "$PR_NUMBER" ]; then
  echo "No open PR detected for this branch. Provide PR number as PR_NUMBER=123" >&2
  exit 3
fi

echo "Fetching Code Scanning alerts for $OWNER_REPO PR #$PR_NUMBER ..."
gh api \
  "/repos/$OWNER_REPO/code-scanning/alerts?state=open&per_page=100&pr=$PR_NUMBER" \
  -H "Accept: application/json" \
  > .reports/code_scanning_alerts.json

TOTAL=$(jq 'length' .reports/code_scanning_alerts.json)
echo "Total open alerts: $TOTAL"

# Summarize by severity if available
jq -r '[.[] | (.rule.security_severity_level // .rule.severity // "unknown")] \
       | group_by(.) \
       | map({severity: .[0], count: length})' \
  .reports/code_scanning_alerts.json > .reports/code_scanning_alerts_summary.json || true

echo "Wrote: .reports/code_scanning_alerts.json and .reports/code_scanning_alerts_summary.json"

