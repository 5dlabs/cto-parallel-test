#!/usr/bin/env bash
set -euo pipefail

# List GitHub Code Scanning alerts for the current PR or a provided PR number.
# Usage:
#   scripts/list_code_scanning.sh              # infers PR from current branch
#   scripts/list_code_scanning.sh <PR_NUMBER>  # explicit PR number
#
# Requires: gh CLI authenticated with a token having repo + security_events

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

PR_NUM="${1:-}"
if [[ -z "$PR_NUM" ]]; then
  BRANCH=$(git rev-parse --abbrev-ref HEAD)
  PR_NUM=$(gh pr list --head "$BRANCH" --json number --jq '.[0].number' 2>/dev/null || true)
  # Fallback via REST API that may work unauthenticated for public repos
  if [[ -z "$PR_NUM" ]]; then
    PR_NUM=$(gh api -X GET "/repos/${OWNER}/${REPO}/pulls?state=open&head=${OWNER}:${BRANCH}" --jq '.[0].number' 2>/dev/null || true)
  fi
fi

if [[ -z "${PR_NUM:-}" ]]; then
  echo "Unable to determine PR number. Provide it explicitly: scripts/list_code_scanning.sh <PR_NUMBER>" >&2
  exit 1
fi

mkdir -p .reports
OUT_FILE=".reports/code-scanning-PR-${PR_NUM}.json"
echo "Fetching Code Scanning alerts for ${OWNER}/${REPO} PR #${PR_NUM}..."

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
