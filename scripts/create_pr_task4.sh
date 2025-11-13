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

# Resolve repo owner/name with robust fallbacks
# 1) $GITHUB_REPOSITORY if present
# 2) Parse from `git remote get-url origin`
# 3) `gh repo view` as a last resort (may require auth)
REPO_FULL=${GITHUB_REPOSITORY:-}
if [[ -z "$REPO_FULL" ]]; then
  if URL=$(git remote get-url origin 2>/dev/null); then
    case "$URL" in
      git@github.com:*/*.git)
        REPO_FULL=${URL#git@github.com:}
        REPO_FULL=${REPO_FULL%.git}
        ;;
      https://github.com/*/*.git)
        REPO_FULL=${URL#https://github.com/}
        REPO_FULL=${REPO_FULL%.git}
        ;;
      https://github.com/*/*)
        REPO_FULL=${URL#https://github.com/}
        ;;
      *)
        : # leave empty; try gh below
        ;;
    esac
  fi
fi
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
PR_NUM=$(gh pr list --head "$BRANCH" --json number --jq '.[0].number' 2>/dev/null || true)
# Fallback via REST API that may work unauthenticated for public repos
if [[ -z "$PR_NUM" ]]; then
  PR_NUM=$(gh api -X GET "/repos/${OWNER}/${REPO}/pulls?state=open&head=${OWNER}:${BRANCH}" --jq '.[0].number' 2>/dev/null || true)
fi
if [[ -n "$PR_NUM" ]]; then
  mkdir -p .reports
  OUT_FILE=".reports/code-scanning-PR-${PR_NUM}.json"
  # Query full alert payload and save to reports; also echo a summary if present
  if gh api \
    "/repos/${OWNER}/${REPO}/code-scanning/alerts?state=open&pr=$PR_NUM" \
    --jq '.' > "$OUT_FILE" 2>/dev/null; then
    COUNT=$(jq 'length' "$OUT_FILE" 2>/dev/null || echo 0)
    if [[ "$COUNT" -eq 0 ]]; then
      echo "No open Code Scanning alerts for PR #${PR_NUM}."
    else
      echo "Open Code Scanning alerts (${COUNT}):"
      jq -r 'map("- [" + .rule.severity + "] " + .rule.id + " (" + .state + ")")[]' "$OUT_FILE"
    fi
    echo "Saved full response to $OUT_FILE"
  else
    echo "Failed to fetch Code Scanning alerts (authentication likely required)." >&2
  fi
fi

echo "Done."
