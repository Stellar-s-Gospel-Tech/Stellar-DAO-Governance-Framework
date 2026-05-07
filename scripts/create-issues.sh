#!/bin/bash
# Usage: ./scripts/create-issues.sh [layer]
#
# Examples:
#   ./scripts/create-issues.sh 2        # create Phase 2 issues
#   ./scripts/create-issues.sh all      # create all layers
#
# Prerequisites:
#   gh auth login   (run once to authenticate with your GitHub account)
#   Set REPO below to your GitHub repo in the format owner/repo-name.

set -e

REPO="YOUR_GITHUB_USERNAME/Stellar_DAO_Governance_Framework"
DIR="$(dirname "$0")/issues"
LAYER="${1:-2}"

create_issue() {
  local file="$1"
  local title="$2"
  local labels="$3"

  echo "Creating: $title"
  gh issue create \
    --repo "$REPO" \
    --title "$title" \
    --label "$labels" \
    --body-file "$file"
  sleep 2  # avoid GitHub API rate limiting
}

create_labels() {
  echo "Ensuring labels exist..."
  gh label create "phase-2"        --color "0075ca" --description "Phase 2 — core logic"              --repo "$REPO" 2>/dev/null || true
  gh label create "phase-3"        --color "e4e669" --description "Phase 3 — hardening"               --repo "$REPO" 2>/dev/null || true
  gh label create "good first issue" --color "7057ff" --description "Good for newcomers"              --repo "$REPO" 2>/dev/null || true
  gh label create "help wanted"    --color "008672" --description "Extra attention needed"             --repo "$REPO" 2>/dev/null || true
  gh label create "governance"     --color "d93f0b" --description "Governance contract"               --repo "$REPO" 2>/dev/null || true
  gh label create "treasury"       --color "b60205" --description "Treasury contract"                 --repo "$REPO" 2>/dev/null || true
  gh label create "token-weight"   --color "1d76db" --description "Token-weight contract"             --repo "$REPO" 2>/dev/null || true
  gh label create "testing"        --color "cccccc" --description "Test coverage"                     --repo "$REPO" 2>/dev/null || true
}

layer2() {
  create_issue "$DIR/issue-01.md" "GOV-001: Wire token-weight cross-contract call in cast_vote"          "phase-2,governance,token-weight,good first issue"
  create_issue "$DIR/issue-02.md" "GOV-002: Implement snapshot-based voting weight"                      "phase-2,governance,token-weight,help wanted"
  create_issue "$DIR/issue-03.md" "GOV-003: Implement on-chain action dispatch in execute()"             "phase-2,governance,help wanted"
  create_issue "$DIR/issue-04.md" "GOV-004: Upgrade quorum check to use total token supply"              "phase-2,governance,good first issue"
  create_issue "$DIR/issue-05.md" "GOV-005: Enforce minimum token balance for proposal creation"         "phase-2,governance,good first issue"
  create_issue "$DIR/issue-06.md" "GOV-006: Implement vote delegation"                                   "phase-2,governance,help wanted"
  create_issue "$DIR/issue-07.md" "GOV-007: Emit events for proposal creation, voting, finalize, spend"  "phase-2,governance,treasury,good first issue"
}

# Add layer3() here when Phase 3 issues are written.
# Follow the same pattern: write issue body files in scripts/issues/,
# add a layer3() function, and add 3) layer3() ;; to the case block below.

create_labels

case "$LAYER" in
  2)   layer2 ;;
  all) layer2 ;;
  *)   echo "Unknown layer: $LAYER. Use 2 or all."; exit 1 ;;
esac

echo ""
echo "Done. View issues at: https://github.com/$REPO/issues"
