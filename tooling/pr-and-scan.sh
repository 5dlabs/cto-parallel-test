#!/usr/bin/env bash
set -euo pipefail

# Create or reuse a PR for the current branch, then fetch GH Code Scanning alerts for it.
# Usage:
#   tooling/pr-and-scan.sh [owner/repo]
# Env:
#   PR_LABELS      Comma-separated labels to apply to the PR
#                  default: "task-6,service-cto-parallel-test,run-play-task-6-qdv4v"
#   PR_TITLE       Optional PR title (falls back to --fill if unset)
#   PR_BODY_FILE   Path to body file (default: docs/PR_BODY_TASK_6.md)
# Defaults:
#   owner/repo inferred from git remote 'origin'

have_cmd() { command -v "$1" >/dev/null 2>&1; }
err() { echo "[error] $*" >&2; }
info() { echo "[info] $*" >&2; }

repo_from_remote() {
  local url
  url=$(git remote get-url origin 2>/dev/null || true)
  if [[ -z "$url" ]]; then
    return 1
  fi
  # Support https and ssh formats
  if [[ "$url" =~ ^https?://github.com/([^/]+)/([^/.]+)(\.git)?$ ]]; then
    echo "${BASH_REMATCH[1]}/${BASH_REMATCH[2]}"
  elif [[ "$url" =~ ^git@github.com:([^/]+)/([^/.]+)(\.git)?$ ]]; then
    echo "${BASH_REMATCH[1]}/${BASH_REMATCH[2]}"
  else
    return 1
  fi
}

if ! have_cmd git; then
  err "git is required"
  exit 2
fi

REPO="${1:-}"
if [[ -z "$REPO" ]]; then
  REPO=$(repo_from_remote || true)
fi
if [[ -z "$REPO" ]]; then
  err "Could not infer repo. Provide as owner/repo."
  exit 2
fi

BRANCH=$(git rev-parse --abbrev-ref HEAD)
if [[ "$BRANCH" == "HEAD" || -z "$BRANCH" ]]; then
  err "Not on a branch. Checkout a branch first."
  exit 2
fi

info "Repository: $REPO"
info "Branch: $BRANCH"

# Ensure we can talk to GitHub via gh or tokens
if ! have_cmd gh; then
  err "GitHub CLI (gh) is required. Install gh and authenticate."
  exit 2
fi

if ! gh auth status >/dev/null 2>&1; then
  if [[ -n "${GH_TOKEN:-${GITHUB_TOKEN:-}}" ]]; then
    info "Attempting non-interactive gh auth using GH_TOKEN/GITHUB_TOKEN"
    # shellcheck disable=SC2312
    printf '%s' "${GH_TOKEN:-${GITHUB_TOKEN:-}}" | gh auth login --with-token >/dev/null 2>&1 || true
  fi
fi

if ! gh auth status >/dev/null 2>&1; then
  err "gh is not authenticated. Export GH_TOKEN or run 'gh auth login'."
  exit 2
fi

info "Pushing branch to origin..."
git push -u origin "$BRANCH"

PR_LABELS_DEFAULT="task-6,service-cto-parallel-test,run-play-task-6-qdv4v"
PR_LABELS_CSV="${PR_LABELS:-$PR_LABELS_DEFAULT}"
PR_BODY_FILE="${PR_BODY_FILE:-docs/PR_BODY_TASK_6.md}"
PR_TITLE="${PR_TITLE:-}"

# Try to create a PR, but tolerate if it already exists
PR_URL=""
set +e
if [[ -n "$PR_TITLE" ]]; then
  PR_URL=$(gh pr create --repo "$REPO" \
    --title "$PR_TITLE" \
    --body-file "$PR_BODY_FILE" \
    --label "$PR_LABELS_CSV" 2>/dev/null)
else
  PR_URL=$(gh pr create --repo "$REPO" \
    --fill \
    --body-file "$PR_BODY_FILE" \
    --label "$PR_LABELS_CSV" 2>/dev/null)
fi
rc=$?
set -e
if [[ $rc -ne 0 ]]; then
  info "PR may already exist or creation failed; attempting to resolve existing PR"
fi

PR_NUMBER=$(gh pr view --repo "$REPO" --json number -q .number)
info "PR #$PR_NUMBER ready: ${PR_URL:-"$(gh pr view --repo "$REPO" --json url -q .url)"}"

info "Querying Code Scanning alerts for PR #$PR_NUMBER ..."
ALERTS=$("$(dirname "$0")/gh-code-scanning.sh" "$REPO" "$PR_NUMBER" || true)

if [[ -z "$ALERTS" || "$ALERTS" == "No open Code Scanning alerts for PR #${PR_NUMBER}." ]]; then
  echo "No open MEDIUM/HIGH/CRITICAL alerts."
  exit 0
fi

echo "Open MEDIUM/HIGH/CRITICAL alerts:" >&2
echo "$ALERTS" | sed 's/^/  /' >&2

# Fail so CI can catch unresolved security issues
exit 1
