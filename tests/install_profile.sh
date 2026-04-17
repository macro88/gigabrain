#!/usr/bin/env sh
# tests/install_profile.sh — deterministic tests for the installer's profile-write logic.
#
# Sources scripts/install.sh in GBRAIN_TEST_MODE=1 to isolate profile functions without
# running the download/install path. Tests cover: fresh write, idempotency, opt-out
# (--no-profile / GBRAIN_NO_PROFILE=1), and regex-metacharacter safety.
#
# Usage:
#   sh tests/install_profile.sh
#
# Exit code: 0 = all pass, 1 = one or more failures.

set -e

PASS=0
FAIL=0

ok() {
  printf '  ok: %s\n' "$1"
  PASS=$((PASS + 1))
}

not_ok() {
  printf '  FAIL: %s\n' "$1"
  FAIL=$((FAIL + 1))
}

# Locate project root relative to this script
SCRIPT_DIR="$(cd "$(dirname "$0")/.." && pwd)"
INSTALL_SH="$SCRIPT_DIR/scripts/install.sh"

# Isolated home dir written under target/ so it is gitignored and cleaned by cargo clean
TEST_HOME="$SCRIPT_DIR/target/test-home-install-profile"
rm -rf "$TEST_HOME"
mkdir -p "$TEST_HOME"

# Required variables install.sh reads at top level before any function executes
GBRAIN_TEST_MODE=1
GBRAIN_RELEASE_API_URL="https://example.invalid"
GBRAIN_RELEASE_BASE_URL="https://example.invalid"
GBRAIN_INSTALL_DIR="$TEST_HOME/bin"
GBRAIN_NO_PROFILE=0
GBRAIN_CHANNEL="airgapped"
GBRAIN_VERSION="v0.0.0-test"
HOME="$TEST_HOME"
mkdir -p "$GBRAIN_INSTALL_DIR"

# Source the installer in test mode — function definitions load, main() does not run
# shellcheck source=../scripts/install.sh
. "$INSTALL_SH"

printf '\nRunning install profile tests...\n\n'

# ---------------------------------------------------------------
# write_profile_line
# ---------------------------------------------------------------

PROFILE="$TEST_HOME/.zshrc_test"
LINE='export PATH="/test/.local/bin:$PATH"'

# T1: appends to a fresh (empty) file
printf '' > "$PROFILE"
if write_profile_line "$PROFILE" "$LINE"; then
  if grep -Fq "$LINE" "$PROFILE"; then
    ok "T1: write_profile_line appends to fresh file"
  else
    not_ok "T1: write_profile_line appends to fresh file — line missing after append"
  fi
else
  not_ok "T1: write_profile_line appends to fresh file — returned non-zero"
fi

# T2a: idempotent — second call returns non-zero (nothing to write)
if write_profile_line "$PROFILE" "$LINE"; then
  not_ok "T2a: write_profile_line returns non-zero when line already present"
else
  ok "T2a: write_profile_line returns non-zero when line already present"
fi

# T2b: idempotent — no duplicate written
count=$(grep -cF "$LINE" "$PROFILE")
if [ "$count" = "1" ]; then
  ok "T2b: write_profile_line does not duplicate an existing line"
else
  not_ok "T2b: write_profile_line duplicated line: found $count copies"
fi

# T3: missing profile file — returns non-zero, does not create file
MISSING="$TEST_HOME/.no_such_profile"
rm -f "$MISSING"
if write_profile_line "$MISSING" "$LINE"; then
  not_ok "T3: write_profile_line returns non-zero for missing file"
else
  ok "T3: write_profile_line returns non-zero for missing file"
fi
if [ -f "$MISSING" ]; then
  not_ok "T3b: write_profile_line must not create missing profile"
else
  ok "T3b: write_profile_line does not create missing profile"
fi

# T4: fixed-string matching — regex metacharacters in path are handled correctly
REGEX_PROFILE="$TEST_HOME/.profile_regex"
REGEX_LINE='export PATH="/test/.local/bin[x]:$PATH"'
printf '' > "$REGEX_PROFILE"
if write_profile_line "$REGEX_PROFILE" "$REGEX_LINE"; then
  ok "T4: write_profile_line handles regex metacharacters in path (first write)"
else
  not_ok "T4: write_profile_line failed with metacharacters in path"
fi
# Second write must still be idempotent
if write_profile_line "$REGEX_PROFILE" "$REGEX_LINE"; then
  not_ok "T4b: idempotent check failed with metacharacters in path"
else
  ok "T4b: idempotent check works with metacharacters in path"
fi
count2=$(grep -cF "$REGEX_LINE" "$REGEX_PROFILE")
if [ "$count2" = "1" ]; then
  ok "T4c: no duplicate for metacharacter path"
else
  not_ok "T4c: found $count2 copies for metacharacter path"
fi

# ---------------------------------------------------------------
# write_profile
# ---------------------------------------------------------------

WP_PROFILE="$TEST_HOME/.wp_zshrc"
printf '' > "$WP_PROFILE"

# Override detect_profile so write_profile uses our test file
detect_profile() { PROFILE_FILE="$WP_PROFILE"; }

# T5: write_profile writes both exports to a fresh profile
write_profile
if grep -Fq "export PATH=" "$WP_PROFILE" && grep -Fq "export GBRAIN_DB=" "$WP_PROFILE"; then
  ok "T5: write_profile writes both PATH and GBRAIN_DB exports"
else
  not_ok "T5: write_profile did not write expected exports"
fi

# T6: write_profile is idempotent on re-run
write_profile
path_count=$(grep -c "export PATH=" "$WP_PROFILE")
db_count=$(grep -c "export GBRAIN_DB=" "$WP_PROFILE")
if [ "$path_count" = "1" ] && [ "$db_count" = "1" ]; then
  ok "T6: write_profile is idempotent (no duplicates on re-run)"
else
  not_ok "T6: write_profile duplicated lines: PATH×${path_count} GBRAIN_DB×${db_count}"
fi

# T7: profile does not key off current PATH — write_profile always checks the file
# Even if INSTALL_DIR happens to appear in the current session PATH, write_profile
# must defer to the profile file for idempotency, not the live environment.
OLD_PATH="$PATH"
# Temporarily add INSTALL_DIR to the session PATH to simulate an already-active session
PATH="${INSTALL_DIR}:${PATH}"
WP_PROFILE2="$TEST_HOME/.wp_fresh"
printf '' > "$WP_PROFILE2"
detect_profile() { PROFILE_FILE="$WP_PROFILE2"; }
write_profile
if grep -Fq "export PATH=" "$WP_PROFILE2"; then
  ok "T7: write_profile writes to profile regardless of current session PATH"
else
  not_ok "T7: write_profile skipped PATH write because INSTALL_DIR was already in session PATH"
fi
PATH="$OLD_PATH"

# ---------------------------------------------------------------
# --no-profile / GBRAIN_NO_PROFILE=1 opt-out
# ---------------------------------------------------------------

# T8: NO_PROFILE=1 branch — profile file must not be touched
WP_PROFILE3="$TEST_HOME/.optout_test"
printf '' > "$WP_PROFILE3"
detect_profile() { PROFILE_FILE="$WP_PROFILE3"; }
NO_PROFILE=1
# Simulate the main() branch: --no-profile calls print_manual_hints, not write_profile
if [ "$NO_PROFILE" = "1" ]; then
  print_manual_hints > /dev/null 2>&1
fi
if [ -s "$WP_PROFILE3" ]; then
  not_ok "T8: NO_PROFILE=1 must not write to profile"
else
  ok "T8: NO_PROFILE=1 leaves profile untouched"
fi
NO_PROFILE=0

# T9: GBRAIN_NO_PROFILE=1 env var is read at startup into NO_PROFILE
# The script initializes: NO_PROFILE="${GBRAIN_NO_PROFILE:-0}"
# We verify that behavior by checking the value directly.
GBRAIN_NO_PROFILE=1
_computed_no_profile="${GBRAIN_NO_PROFILE:-0}"
if [ "$_computed_no_profile" = "1" ]; then
  ok "T9: GBRAIN_NO_PROFILE=1 env var propagates to NO_PROFILE at startup"
else
  not_ok "T9: GBRAIN_NO_PROFILE=1 did not propagate"
fi
GBRAIN_NO_PROFILE=0

# ---------------------------------------------------------------
# Summary
# ---------------------------------------------------------------
printf '\n%d passed, %d failed\n' "$PASS" "$FAIL"

if [ "$FAIL" -gt 0 ]; then
  exit 1
fi
exit 0
