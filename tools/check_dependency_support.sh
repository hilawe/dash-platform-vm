#!/usr/bin/env bash
# Check the execution-engine dependency's support status against the runway
# policy, and check that the repository is telling the truth about it.
#
# Requirements register P4 sets the runway threshold. P7 says an expired line
# does not invalidate architecture-level feasibility evidence, but does make
# every version-bound gate stale and forbids describing the line as an
# integration target.
#
# So this is a CONSISTENCY check rather than a simple expiry check. An expired
# dependency is permitted in research, because that is the honest current state.
# What is NOT permitted is shipping an expired dependency while the documents
# imply it is current. The check therefore fails when the status and the
# disclosure disagree, which is a condition the project can always satisfy by
# telling the truth, and can never satisfy by waiting.
#
# Usage: check_dependency_support.sh <directory-of-docs-to-check>

set -euo pipefail

HERE="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
DOCS="${1:-}"
SUPPORT="$HERE/dependency_support.txt"

red() { printf '\033[31m%s\033[0m\n' "$*"; }
grn() { printf '\033[32m%s\033[0m\n' "$*"; }

[ -f "$SUPPORT" ] || { red "STOPPED: $SUPPORT missing, the check cannot run (an unrun check has not passed)"; exit 1; }
[ -n "$DOCS" ] && [ -d "$DOCS" ] || { red "STOPPED: pass the directory holding the documents to check"; exit 1; }

RUNWAY="$(grep '^RUNWAY_DAYS=' "$SUPPORT" | cut -d= -f2)"
[ -n "$RUNWAY" ] || { red "STOPPED: RUNWAY_DAYS not set in $SUPPORT"; exit 1; }

STATUS=0

while IFS='|' read -r crate version line ends source; do
  case "$crate" in ''|\#*|RUNWAY_DAYS=*) continue ;; esac

  # Day arithmetic in perl, since BSD date and GNU date disagree on everything.
  REMAINING="$(perl -MTime::Local -e '
     my ($y,$m,$d) = split /-/, $ARGV[0];
     my $end = Time::Local::timegm(0,0,12,$d,$m-1,$y);
     my @n = gmtime(time); my $now = Time::Local::timegm(0,0,12,$n[3],$n[4],$n[5]+1900);
     printf "%d", int(($end-$now)/86400);
  ' "$ends")"

  echo "  $crate $version, line $line, support ends $ends, remaining ${REMAINING}d (runway policy ${RUNWAY}d)"

  if [ "$REMAINING" -ge "$RUNWAY" ]; then
    grn "  integration-eligible under P4"
    continue
  fi

  # Below the threshold. The line may still be used for research, but the
  # documents must say so. Each required disclosure is a specific claim the
  # project would have to actively remove in order to mislead.
  if [ "$REMAINING" -lt 0 ]; then
    echo "  EXPIRED. Disclosure is mandatory."
    NEED_EXPIRY=1
  else
    echo "  below the runway threshold, not integration-eligible. Disclosure is mandatory."
    NEED_EXPIRY=0
  fi

  missing=""
  # 1. The support end date must appear somewhere a reader will meet it.
  grep -rqF "$ends" "$DOCS" 2>/dev/null || missing="$missing\n    the support end date $ends is not stated in any shipped document"
  # 2. The gate specification must not claim a version-bound gate passes.
  # Read the gate specification's STATUS TABLE rather than sniffing its prose.
  # An earlier version pattern-matched wording and flagged the sentence "None
  # passes", which asserts the opposite of what it was looking for. A structured
  # field is checkable; a sentence about a field is not.
  if [ -f "$DOCS/docs/GATE_SPECIFICATION.md" ]; then
    claimed_pass="$(awk -F'|' '/^\| G[0-9]/ { gsub(/[ *]/,"",$3); if (tolower($3) == "pass") print $2 }' \
                    "$DOCS/docs/GATE_SPECIFICATION.md" 2>/dev/null || true)"
    if [ -n "$claimed_pass" ]; then
      missing="$missing\n    the gate status table marks these as PASS while the dependency line is not integration-eligible:$(printf '%s' "$claimed_pass" | tr '\n' ' ')"
    fi
    grep -q '^| G[0-9]' "$DOCS/docs/GATE_SPECIFICATION.md" 2>/dev/null \
      || missing="$missing\n    the gate specification has no status table, so gate staleness cannot be checked"
  else
    missing="$missing\n    docs/GATE_SPECIFICATION.md is absent, so gate staleness cannot be checked"
  fi
  # 3. Expiry specifically must be disclosed as such, not merely dated.
  if [ "$NEED_EXPIRY" = "1" ]; then
    grep -rqiE "support (ended|expired)|end of security support|expired (support )?line" "$DOCS" 2>/dev/null \
      || missing="$missing\n    expiry is not disclosed in words anywhere in the shipped documents"
  fi

  if [ -n "$missing" ]; then
    red "  dependency status and disclosure disagree:"
    printf "%b\n" "$missing"
    STATUS=1
  else
    grn "  not integration-eligible, and the documents say so"
  fi
done < "$SUPPORT"

if [ "$STATUS" -ne 0 ]; then
  red "STOPPED: dependency-support gate"
  exit 1
fi
exit 0
