#!/usr/bin/env bash
set -euo pipefail

# Usage: scripts/create_pr_task4.sh
# Policy:
# - Do not attempt interactive auth; rely on GH_TOKEN/GITHUB_TOKEN already configured
# - Never push to main; only push current feature branch
# - Parameterize owner/repo via env or gh

BRANCH=$(git rev-parse --abbrev-ref HEAD)
if [[ "$BRANCH" != "feature/task-4-implementation" ]]; then
  echo "Current branch is '$BRANCH'. Please switch to 'feature/task-4-implementation'." >&2
  exit 1
fi

if ! command -v gh >/dev/null 2>&1; then
  echo "GitHub CLI (gh) is required. Install from https://cli.github.com/" >&2
  exit 1
fi

# Resolve repo owner/name without hitting user endpoints
REPO_FULL=${GITHUB_REPOSITORY:-}
if [[ -z "$REPO_FULL" ]]; then
  REPO_FULL=$(gh repo view --json nameWithOwner -q .nameWithOwner)
fi
OWNER=${REPO_FULL%/*}
REPO=${REPO_FULL#*/}

echo "Pushing branch '$BRANCH' to origin..."
git push -u origin "$BRANCH"

# Optional: link an issue with label task-4 if available
ISSUE_NUM=$(gh issue list --label "task-4" --json number --jq '.[0].number' 2>/dev/null || echo "")

TITLE="feat(${REPO}): implement task 4 - product catalog module"
BODY_FILE="task/pr-body-task-4.md"

echo "Creating PR from '$BRANCH' into 'main'..."
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

echo "PR created. Listing Code Scanning alerts (if any)..."
PR_NUM=$(gh pr list --head "$BRANCH" --json number --jq '.[0].number')
if [[ -n "$PR_NUM" ]]; then
  gh api \
    "/repos/${OWNER}/${REPO}/code-scanning/alerts?state=open&pr=$PR_NUM" \
    --jq 'map({rule: .rule.id, severity: .rule.severity, state: .state})'
fi

echo "Done."

