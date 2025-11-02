#!/usr/bin/env bash
set -euo pipefail

# Gate merges on GitHub Code Scanning alerts for the current PR.
#
# Requirements:
# - gh (GitHub CLI) authenticated with a token granting `security_events:read`
# - jq
#
# Usage:
#   scripts/codescan-gate.sh [--repo owner/repo] [--pr N]
#
# If not provided, the repo slug and PR number are auto-detected from `origin` and HEAD.

err() { echo "[codescan-gate] $*" >&2; }

need_cmd() {
  command -v "$1" >/dev/null 2>&1 || { err "Missing dependency: $1"; exit 2; }
}

need_cmd gh
need_cmd jq

REPO=""
PR=""
while [[ $# -gt 0 ]]; do
  case "$1" in
    --repo)
      REPO="$2"; shift 2 ;;
    --pr)
      PR="$2"; shift 2 ;;
    *)
      err "Unknown arg: $1"; exit 2 ;;
  esac
done

# Resolve repo slug from git if not provided
if [[ -z "$REPO" ]]; then
  origin_url=$(git remote get-url origin 2>/dev/null || true)
  if [[ -z "$origin_url" ]]; then
    err "Unable to determine git remote 'origin' URL. Use --repo owner/repo."
    exit 2
  fi
  # Support HTTPS and SSH forms
  if [[ "$origin_url" =~ github.com[:/](.+/.+)\.git$ ]]; then
    REPO="${BASH_REMATCH[1]}"
  elif [[ "$origin_url" =~ github.com[:/](.+/.+)$ ]]; then
    REPO="${BASH_REMATCH[1]}"
  else
    err "Unrecognized GitHub remote: $origin_url. Use --repo owner/repo."
    exit 2
  fi
fi

# Resolve PR number if not provided
if [[ -z "$PR" ]]; then
  if ! PR=$(gh pr view --repo "$REPO" --json number -q .number 2>/dev/null); then
    err "Failed to resolve PR number for repo $REPO. Provide with --pr N."
    exit 2
  fi
fi

err "Checking Code Scanning alerts for PR #$PR in $REPO..."

# Fetch all open alerts for this PR (paginate in case of many)
alerts=$(gh api \
  "/repos/$REPO/code-scanning/alerts?state=open&pr=$PR" \
  --paginate \
  --jq '.[] | {rule_id: .rule.id, severity: .rule.severity, message: .most_recent_instance.message.text, url: .html_url}')

if [[ -z "$alerts" ]]; then
  echo "No open Code Scanning alerts for PR #$PR."
  exit 0
fi

echo "Open alerts:\n$alerts" | sed 's/^/- /'

# Fail if any medium/high/critical severities are present
violations=$(gh api \
  "/repos/$REPO/code-scanning/alerts?state=open&pr=$PR" \
  --paginate \
  | jq '[.[] | select((.rule.severity // "unknown") | test("^(?i)(medium|high|critical)$"))] | length')

if [[ "$violations" -gt 0 ]]; then
  err "Found $violations MEDIUM/HIGH/CRITICAL alerts. Failing gate."
  exit 1
fi

echo "No MEDIUM/HIGH/CRITICAL alerts found. Gate passed."
exit 0

