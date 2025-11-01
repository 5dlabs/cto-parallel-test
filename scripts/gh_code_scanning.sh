#!/usr/bin/env bash
set -euo pipefail

# GitHub Code Scanning alert fetcher for the current PR.
# Requires a token with the `security_events` scope.
#
# Usage:
#   REPO=5dlabs/cto-parallel-test PR=123 scripts/gh_code_scanning.sh
#   # or let it auto-detect PR from the current branch:
#   REPO=5dlabs/cto-parallel-test scripts/gh_code_scanning.sh
#
# Exits non-zero if any MEDIUM/HIGH/CRITICAL alerts are found.

color() { printf "\033[%sm%s\033[0m" "$1" "$2"; }
red() { color 31 "$1"; }
yellow() { color 33 "$1"; }
green() { color 32 "$1"; }

# Resolve repository slug (owner/name)
REPO=${REPO:-}
if [[ -z "${REPO}" ]]; then
  if url=$(git config --get remote.origin.url 2>/dev/null); then
    # Support HTTPS and SSH remotes
    if [[ "$url" =~ github.com[:/](.+/.+)\.git$ ]]; then
      REPO="${BASH_REMATCH[1]}"
    fi
  fi
fi

if [[ -z "${REPO:-}" ]]; then
  echo "[error] Could not determine REPO. Set REPO=owner/name." >&2
  exit 2
fi

echo "[info] Repository: ${REPO}"

if ! gh auth status -t >/dev/null 2>&1; then
  echo "[error] gh is not authenticated. Provide a token with security_events scope." >&2
  echo "        Export GITHUB_TOKEN and login:" >&2
  echo "        export GITHUB_TOKEN=\"<token-with-security_events>\"" >&2
  echo "        gh auth login --with-token <<< \"$GITHUB_TOKEN\"" >&2
  exit 2
fi

# Determine PR number
PR=${PR:-}
if [[ -z "${PR}" ]]; then
  current_branch=$(git rev-parse --abbrev-ref HEAD)
  PR=$(gh pr list --repo "$REPO" --head "$current_branch" --json number -q '.[0].number' || true)
fi

if [[ -z "${PR}" ]]; then
  echo "[error] No open PR found for this branch. Provide PR=<number>." >&2
  exit 2
fi

echo "[info] Querying open Code Scanning alerts for PR #${PR} (all pages)..."
# Fetch all pages (up to API limits) and merge arrays
resp=$(gh api -H "Accept: application/vnd.github+json" --paginate \
  "/repos/${REPO}/code-scanning/alerts?state=open&pr=${PR}&per_page=100" | jq -s 'add')

count=$(jq 'length' <<<"$resp")
if [[ "$count" -eq 0 ]]; then
  echo "$(green "No open Code Scanning alerts." )"
  exit 0
fi

echo "severity\trule_id\ttool\tstate\turl"
echo "$resp" | jq -r '.[] | [
  (.rule.security_severity_level // .rule.severity // "unknown"),
  .rule.id,
  .tool.name,
  .state,
  .html_url
] | @tsv'

# Fail on MEDIUM/HIGH/CRITICAL
bad=$(echo "$resp" | jq '[.[] | (.rule.security_severity_level // .rule.severity // "unknown") | ascii_downcase | select(. == "medium" or . == "high" or . == "critical")] | length')
if [[ "$bad" -gt 0 ]]; then
  echo "$(red "Found ${bad} MEDIUM/HIGH/CRITICAL alerts.")" >&2
  exit 1
fi

echo "$(green "Only LOW/INFO alerts present." )"
exit 0
