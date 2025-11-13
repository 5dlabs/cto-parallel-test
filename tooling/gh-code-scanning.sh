#!/usr/bin/env bash
set -euo pipefail

# Fetch open GitHub Code Scanning alerts for a PR and print MEDIUM/HIGH/CRITICAL.
# Usage:
#   ./tooling/gh-code-scanning.sh [repo] [pr_number]
# Defaults:
#   repo: 5dlabs/cto-parallel-test
#   pr_number: inferred from current branch via gh (if authenticated)

REPO="${1:-5dlabs/cto-parallel-test}"
PR_NUM="${2:-}"

have_cmd() { command -v "$1" >/dev/null 2>&1; }

err() { echo "[error] $*" >&2; }
info() { echo "[info] $*" >&2; }

if [[ -z "$PR_NUM" ]]; then
  if have_cmd gh; then
    if gh auth status >/dev/null 2>&1; then
      BR=$(git rev-parse --abbrev-ref HEAD)
      PR_NUM=$(gh pr list --repo "$REPO" --head "$BR" --json number -q '.[0].number' || true)
      if [[ -z "$PR_NUM" ]]; then
        err "Could not infer PR number for branch '$BR'. Provide it explicitly."
        exit 2
      fi
    else
      err "gh is not authenticated; provide PR number explicitly."
      exit 2
    fi
  else
    err "gh not installed; provide PR number explicitly."
    exit 2
  fi
fi

fetch_with_gh() {
  gh api \
    "/repos/${REPO}/code-scanning/alerts?state=open&per_page=100&pr=${PR_NUM}" 2>/dev/null || return 1
}

fetch_with_curl() {
  if [[ -z "${GH_TOKEN:-${GITHUB_TOKEN:-}}" ]]; then
    return 1
  fi
  local token="${GH_TOKEN:-${GITHUB_TOKEN:-}}"
  curl -fsSL -H "Authorization: token ${token}" \
    "https://api.github.com/repos/${REPO}/code-scanning/alerts?state=open&per_page=100&pr=${PR_NUM}"
}

JSON=""
if have_cmd gh && gh auth status >/dev/null 2>&1; then
  info "Fetching alerts with gh api for ${REPO} PR #${PR_NUM}"
  set +e
  JSON=$(fetch_with_gh)
  rc=$?
  set -e
  if [[ $rc -ne 0 || -z "$JSON" ]]; then
    err "gh api failed; trying curl with GH_TOKEN/GITHUB_TOKEN"
  fi
fi

if [[ -z "$JSON" ]]; then
  set +e
  JSON=$(fetch_with_curl)
  rc=$?
  set -e
  if [[ $rc -ne 0 || -z "$JSON" ]]; then
    err "Failed to fetch alerts. Ensure gh is authenticated or GH_TOKEN/GITHUB_TOKEN is set."
    exit 1
  fi
fi

if [[ -z "$JSON" || "$JSON" == "[]" ]]; then
  echo "No open Code Scanning alerts for PR #${PR_NUM}."
  exit 0
fi

# Print only MEDIUM/HIGH/CRITICAL alerts in a concise table.
if have_cmd jq; then
  echo "$JSON" | jq -r '
    .[]
    | select((.rule.security_severity_level // "") as $lvl
             | ($lvl|ascii_downcase)=="critical" or ($lvl|ascii_downcase)=="high" or ($lvl|ascii_downcase)=="medium")
    | [
        (.rule.security_severity_level // "unknown"),
        (.rule.id // .rule.name // "rule"),
        (.most_recent_instance.location.path // "path"),
        ((.most_recent_instance.location.start_line // 1) | tostring)
      ]
    | @tsv'
else
  # Fallback: print raw JSON
  echo "$JSON"
fi

