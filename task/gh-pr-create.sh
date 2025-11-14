#!/usr/bin/env bash
set -euo pipefail

# Create the Task 6 PR from the current repository/branch using gh CLI.
# Usage:
#   bash task/gh-pr-create.sh [head_branch] [base_branch]
# Defaults: head=feature/task-6-implementation, base=main

REPO_SLUG="5dlabs/cto-parallel-test"
HEAD_BRANCH="${1:-feature/task-6-implementation}"
BASE_BRANCH="${2:-main}"

if ! command -v gh >/dev/null 2>&1; then
  echo "ERROR: gh CLI is not installed." >&2
  exit 1
fi

if ! gh auth status >/dev/null 2>&1; then
  echo "ERROR: gh CLI not authenticated for github.com" >&2
  echo "Run: gh auth login -h github.com" >&2
  exit 2
fi

ISSUE_NUM=$(gh issue list --repo "$REPO_SLUG" --label "task-6" --json number --jq '.[0].number' 2>/dev/null || echo "")

TITLE="feat(cto-parallel-test): implement task 6 - secure frontend + scans"
BODY_FILE="docs/pr-body-task-6.md"

ARGS=(
  --repo "$REPO_SLUG"
  --base "$BASE_BRANCH"
  --head "$HEAD_BRANCH"
  --title "$TITLE"
  --label "task-6"
  --label "service-cto-parallel-test"
  --label "run-play-task-6-wcw5b"
)

if [[ -f "$BODY_FILE" ]]; then
  ARGS+=(--body-file "$BODY_FILE")
else
  ARGS+=(--body "Task 6 PR for $HEAD_BRANCH")
fi

# Append issue link automatically if available
if [[ -n "${ISSUE_NUM:-}" ]]; then
  echo "Appending issue link: #$ISSUE_NUM" >&2
  BODY_CONTENT=$(cat "$BODY_FILE")
  printf "%s\n\n## Links\nCloses #%s\n" "$BODY_CONTENT" "$ISSUE_NUM" > "$BODY_FILE"
fi

echo "Creating PR in $REPO_SLUG from $HEAD_BRANCH -> $BASE_BRANCH" >&2
gh pr create "${ARGS[@]}"

