#!/usr/bin/env bash
set -euo pipefail

# Summarize GitHub Code Scanning alerts from a saved JSON file.
#
# Usage:
#   scripts/summarize_code_scanning.sh <PR_NUMBER>
#   scripts/summarize_code_scanning.sh --file .reports/code-scanning-PR-123.json
#
# Notes:
# - Exits non-zero if any MEDIUM/HIGH/CRITICAL alerts are present.
# - Designed to consume files produced by scripts/list_code_scanning.sh

usage() {
  cat >&2 <<'USAGE'
Summarize Code Scanning alerts JSON.

Usage:
  scripts/summarize_code_scanning.sh <PR_NUMBER>
  scripts/summarize_code_scanning.sh --file <path-to-json>

The script prints severity totals and a concise list of alerts.
Returns exit code 2 if any MEDIUM/HIGH/CRITICAL alerts are present.
USAGE
}

JSON_FILE=""
if [[ ${1:-} == "--file" ]]; then
  JSON_FILE=${2:-}
  shift 2 || true
elif [[ $# -ge 1 ]]; then
  PR_NUM=$1
  JSON_FILE=".reports/code-scanning-PR-${PR_NUM}.json"
  shift || true
else
  usage
  exit 1
fi

if [[ -z "${JSON_FILE}" || ! -f "${JSON_FILE}" ]]; then
  echo "JSON file not found: ${JSON_FILE:-<empty>}" >&2
  exit 1
fi

if ! command -v jq >/dev/null 2>&1; then
  echo "jq is required to parse JSON. Please install jq." >&2
  exit 1
fi

# Detect error payloads (e.g., API rate limit). Top-level object with a 'message' field.
TYPE=$(jq -r 'type' "$JSON_FILE" 2>/dev/null || echo "unknown")
if [[ "$TYPE" != "array" ]]; then
  MSG=$(jq -r 'try .message // empty' "$JSON_FILE" 2>/dev/null || true)
  if [[ -n "$MSG" ]]; then
    echo "Code Scanning API response indicates an error:" >&2
    echo "  $MSG" >&2
    DOC=$(jq -r 'try .documentation_url // empty' "$JSON_FILE" 2>/dev/null || true)
    [[ -n "$DOC" ]] && echo "  See: $DOC" >&2
    exit 1
  fi
  echo "Unexpected JSON format in $JSON_FILE (type=$TYPE)." >&2
  exit 1
fi

COUNT=$(jq 'length' "$JSON_FILE")
if [[ "$COUNT" -eq 0 ]]; then
  echo "No open Code Scanning alerts."
  exit 0
fi

# Normalize severity with reasonable fallbacks, to lowercase. Then map
# tool-specific severities (error/warning/note) into security severities
# so gating remains strict even when security_severity_level is absent.
RAW_SEV_JQ='(.rule.severity // .severity // .rule.security_severity_level // .security_severity_level // "unknown") | ascii_downcase'

echo "Open Code Scanning alerts: $COUNT"
echo

# Totals by severity
declare -A totals
for s in critical high medium low note unknown; do totals[$s]=0; done

map_to_security_bucket() {
  case "$1" in
    critical) echo critical ;;
    high)     echo high ;;
    medium)   echo medium ;;
    low)      echo low ;;
    error)    echo high ;;
    warning)  echo low ;;
    note)     echo note ;;
    *)        echo unknown ;;
  esac
}

while IFS= read -r raw; do
  bucket=$(map_to_security_bucket "$raw")
  totals[$bucket]=$(( totals[$bucket] + 1 ))
done < <(jq -r ".[] | ${RAW_SEV_JQ}" "$JSON_FILE")

echo "Severity totals:"
printf "  critical: %d\n" "${totals[critical]}"
printf "  high:     %d\n" "${totals[high]}"
printf "  medium:   %d\n" "${totals[medium]}"
printf "  low:      %d\n" "${totals[low]}"
printf "  note:     %d\n" "${totals[note]}"
printf "  unknown:  %d\n" "${totals[unknown]}"

echo
echo "Alerts:"
# Show the original (raw) severity alongside the mapped bucket
jq -r \
  ".[] | (
    ${RAW_SEV_JQ} as $raw | 
    (
      if ($raw == \"critical\" or $raw == \"high\" or $raw == \"medium\" or $raw == \"low\") then $raw
      elif $raw == \"error\" then \"high\"
      elif $raw == \"warning\" then \"low\"
      elif $raw == \"note\" then \"note\"
      else \"unknown\" end
    ) as $bucket |
    \"- [\" + $bucket + \"] \" + (.rule.id // .rule.name // \"<no-id>\") + \" (state: \" + .state + \", raw: \" + $raw + \")\"
  )" \
  "$JSON_FILE"

# Exit non-zero if any medium/high/critical alerts present
if (( totals[critical] > 0 || totals[high] > 0 || totals[medium] > 0 )); then
  exit 2
fi

exit 0
