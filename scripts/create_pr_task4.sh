#!/usr/bin/env bash
set -euo pipefail

# Usage: scripts/create_pr_task4.sh
# Requires: gh CLI authenticated (GITHUB_TOKEN with repo+security_events)

BRANCH=$(git rev-parse --abbrev-ref HEAD)
if [[ "$BRANCH" != "feature/task-4-implementation" ]]; then
  echo "Current branch is '$BRANCH'. Please switch to 'feature/task-4-implementation'." >&2
  exit 1
fi

if ! command -v gh >/dev/null 2>&1; then
  echo "GitHub CLI (gh) is required. Install from https://cli.github.com/" >&2
  exit 1
fi

echo "Checking GitHub auth..."
if ! gh auth status >/dev/null 2>&1; then
  echo "Not authenticated. Export GITHUB_TOKEN then run: gh auth login --with-token < <(echo '<token>')" >&2
  exit 1
fi

echo "Pushing branch..."
git push -u origin "$BRANCH"

# Try to fetch the task issue number (optional)
ISSUE_NUM=$(gh issue list --label "task-4" --json number --jq '.[0].number' 2>/dev/null || echo "")

TITLE="feat(cto-parallel-test): implement task 4 - product catalog module"
BODY_FILE="task/pr-body-task-4.md"

echo "Creating PR..."
if [[ -n "$ISSUE_NUM" ]]; then
  gh pr create \
    --title "$TITLE" \
    --label "task-4" \
    --label "service-cto-parallel-test" \
    --label "run-play-task-4-nkndw" \
    --body "$(cat "$BODY_FILE")

## Links
Closes #$ISSUE_NUM
" \
    --base main \
    --head "$BRANCH"
else
  gh pr create \
    --title "$TITLE" \
    --label "task-4" \
    --label "service-cto-parallel-test" \
    --label "run-play-task-4-nkndw" \
    --body-file "$BODY_FILE" \
    --base main \
    --head "$BRANCH"
fi

echo "PR created. Listing code scanning alerts (if any)..."
PR_NUM=$(gh pr list --head "$BRANCH" --json number --jq '.[0].number')
if [[ -n "$PR_NUM" ]]; then
  gh api \
    "/repos/5dlabs/cto-parallel-test/code-scanning/alerts?state=open&pr=$PR_NUM" \
    --jq 'map({rule: .rule.id, severity: .rule.severity, state: .state})'
fi

echo "Done."
