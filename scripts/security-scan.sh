#!/usr/bin/env bash
set -euo pipefail

# Security scanning helper
# - Runs gitleaks and cargo-audit locally
# - If GH_TOKEN and a PR exist for the current branch, queries GitHub Code Scanning alerts

echo "[security] Starting local scans (gitleaks, cargo-audit)"

if command -v gitleaks >/dev/null 2>&1; then
  gitleaks detect --no-banner --no-git --source .
else
  echo "[security] gitleaks not found; skipping. Install via: curl -sSL https://github.com/gitleaks/gitleaks/releases | tar ..."
fi

if command -v cargo-audit >/dev/null 2>&1; then
  cargo audit
else
  echo "[security] cargo-audit not found; skipping. Install via: cargo install cargo-audit --locked"
fi

OWNER_REPO=$(git config --get remote.origin.url | sed -E 's#(git@|https://)github.com[:/ ]##; s/\.git$//')
BRANCH=$(git rev-parse --abbrev-ref HEAD)

# Attempt to discover PR number for current branch
PR_NUMBER=""
if command -v gh >/dev/null 2>&1; then
  set +e
  PR_NUMBER=$(gh pr list --head "$BRANCH" --json number -q '.[0].number' 2>/dev/null)
  set -e
fi

if [[ -n "${PR_NUMBER}" && -n "${GH_TOKEN:-}" ]]; then
  echo "[security] Checking GitHub code scanning alerts for PR #${PR_NUMBER} on ${OWNER_REPO}"
  # Prefer gh api with explicit Authorization header to avoid local auth state
  set +e
  RESP=$(gh api -H "Authorization: Bearer ${GH_TOKEN}" \
      "/repos/${OWNER_REPO}/code-scanning/alerts?state=open&pr=${PR_NUMBER}" 2>/dev/null)
  RC=$?
  set -e
  if [[ $RC -ne 0 || -z "$RESP" ]]; then
    echo "[security] gh api failed; trying curl fallback"
    RESP=$(curl -sfL -H "Authorization: Bearer ${GH_TOKEN}" -H "Accept: application/vnd.github+json" \
      "https://api.github.com/repos/${OWNER_REPO}/code-scanning/alerts?state=open&pr=${PR_NUMBER}") || true
  fi

  if [[ -n "$RESP" ]]; then
    OPEN_TOTAL=$(printf '%s' "$RESP" | jq 'length')
    OPEN_MHC=$(printf '%s' "$RESP" | jq '[.[] | ( ( .rule.severity // .rule.security_severity_level // .severity // empty ) | ascii_downcase ) | select(. == "medium" or . == "high" or . == "critical")] | length')
    echo "[security] Open alerts: $OPEN_TOTAL (MEDIUM/HIGH/CRITICAL: $OPEN_MHC)"
    if [[ $OPEN_MHC -gt 0 ]]; then
      echo "[security] MEDIUM/HIGH/CRITICAL alerts present; please address before merge." >&2
      exit 1
    fi
  else
    echo "[security] Unable to retrieve code scanning alerts (no response)"
  fi
else
  echo "[security] Skipping GitHub code scanning API check (no PR discovered or GH_TOKEN missing)"
fi

echo "[security] Security scans complete"

