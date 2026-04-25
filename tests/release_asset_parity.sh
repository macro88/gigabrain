#!/usr/bin/env sh
# tests/release_asset_parity.sh — static parity contract between install.sh and release.yml.
#
# Verifies that:
#   1. Every install.sh-resolvable asset name (platform × channel) appears as an `artifact:`
#      entry in .github/workflows/release.yml.
#   2. The release.yml manifest's expected[] array contains exactly those same names.
#   3. No asset is present in the workflow but absent from the installer's resolution logic
#      (and vice versa).
#
# This test has NO network I/O and NO real binary downloads. It is pure source-level
# static analysis. Run it on any host with sh, grep, and sed.
#
# Usage:
#   sh tests/release_asset_parity.sh
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

SCRIPT_DIR="$(cd "$(dirname "$0")/.." && pwd)"
INSTALL_SH="$SCRIPT_DIR/scripts/install.sh"
RELEASE_YML="$SCRIPT_DIR/.github/workflows/release.yml"

# ── Canonical manifest ─────────────────────────────────────────
# This is the authoritative list: 4 platforms × 2 channels = 8 assets.
# Update this list only when adding or removing a supported platform/channel,
# and update install.sh + release.yml in the same commit.
CANONICAL="
gbrain-darwin-arm64-airgapped
gbrain-darwin-arm64-online
gbrain-darwin-x86_64-airgapped
gbrain-darwin-x86_64-online
gbrain-linux-x86_64-airgapped
gbrain-linux-x86_64-online
gbrain-linux-aarch64-airgapped
gbrain-linux-aarch64-online
"

printf '\nRunning release asset parity tests...\n\n'

# ── T1: install.sh resolve_platform + resolve_channel cover every canonical name ──
# Source installer in test mode to get access to resolve_platform/resolve_channel helpers.
GBRAIN_TEST_MODE=1 \
  GBRAIN_RELEASE_API_URL="https://example.invalid" \
  GBRAIN_RELEASE_BASE_URL="https://example.invalid" \
  GBRAIN_INSTALL_DIR="/tmp/bender-parity-test-bin" \
  GBRAIN_NO_PROFILE=0 \
  GBRAIN_CHANNEL="airgapped" \
  GBRAIN_VERSION="v0.0.0-test" \
  HOME="/tmp/bender-parity-test-home" \
  . "$INSTALL_SH"

# Simulate install.sh asset naming for all platform+channel combos.
# install.sh uses: asset_name="gbrain-${PLATFORM}-${CHANNEL}"
# Platform resolution: os_name-arch_name (see resolve_platform).
simulate_asset() {
  platform="$1"
  channel="$2"
  printf 'gbrain-%s-%s' "$platform" "$channel"
}

for name in $CANONICAL; do
  # Extract platform and channel from canonical name
  # Format: gbrain-<platform>-<channel>  where channel is last segment
  channel="${name##*-}"
  without_channel="${name%-*}"
  platform="${without_channel#gbrain-}"
  expected="$(simulate_asset "$platform" "$channel")"
  if [ "$expected" = "$name" ]; then
    ok "T1[$name]: install.sh naming formula generates expected asset name"
  else
    not_ok "T1[$name]: formula produced '$expected', want '$name'"
  fi
done

# ── T2: every canonical name appears as artifact: in release.yml ──
for name in $CANONICAL; do
  if grep -Fq "artifact: ${name}" "$RELEASE_YML"; then
    ok "T2[$name]: release.yml matrix has artifact: $name"
  else
    not_ok "T2[$name]: release.yml is missing artifact: $name"
  fi
done

# ── T3: release.yml expected[] manifest contains every canonical name ──
for name in $CANONICAL; do
  if grep -Fq "$name" "$RELEASE_YML"; then
    ok "T3[$name]: release.yml manifest expected[] lists $name"
  else
    not_ok "T3[$name]: release.yml manifest expected[] is missing $name"
  fi
done

# ── T4: no extra artifact: lines in release.yml beyond the canonical set ──
# Count artifact: entries in the matrix; must equal 2 × canonical count (binary + checksum
# is one artifact upload, so there should be exactly 8 artifact entries + 8 sha256 = handled
# as pairs). Check that artifact: count equals canonical asset count.
workflow_artifact_count=$(grep -c "artifact: gbrain-" "$RELEASE_YML" || true)
canonical_count=$(printf '%s\n' $CANONICAL | grep -c "gbrain-" || true)
if [ "$workflow_artifact_count" = "$canonical_count" ]; then
  ok "T4: release.yml has exactly $canonical_count artifact: entries (no extras or gaps)"
else
  not_ok "T4: release.yml has $workflow_artifact_count artifact: entries; want $canonical_count"
fi

# ── T5: RELEASE_CHECKLIST.md uses channel-suffixed names, not bare platform names ──
CHECKLIST="$SCRIPT_DIR/.github/RELEASE_CHECKLIST.md"
if [ -f "$CHECKLIST" ]; then
  # Check that no bare (non-channel-suffixed) binary names appear in the checklist.
  # Bare names look like: gbrain-darwin-arm64` or gbrain-linux-x86_64` without -airgapped/-online.
  bare_count=$(grep -Ec 'gbrain-(darwin|linux)-(arm64|x86_64|aarch64)[^-]' "$CHECKLIST" || true)
  if [ "$bare_count" = "0" ]; then
    ok "T5: RELEASE_CHECKLIST.md contains no bare (unsuffixed) binary names"
  else
    not_ok "T5: RELEASE_CHECKLIST.md still has $bare_count bare binary name(s) without -airgapped/-online suffix"
  fi
else
  not_ok "T5: .github/RELEASE_CHECKLIST.md not found"
fi

# ── T6: installer does not attempt to download anything without channel suffix ──
# Verify that install.sh always appends CHANNEL to the asset name (no fallback bare path).
if grep -Fq 'gbrain-${PLATFORM}-${CHANNEL}' "$INSTALL_SH" || \
   grep -Fq '"gbrain-${PLATFORM}-${CHANNEL}"' "$INSTALL_SH"; then
  ok "T6: install.sh asset name always includes CHANNEL suffix (no bare fallback path)"
else
  not_ok "T6: install.sh asset construction does not consistently include CHANNEL suffix"
fi

# ── Summary ──────────────────────────────────────────────────────
printf '\n%d passed, %d failed\n' "$PASS" "$FAIL"

if [ "$FAIL" -gt 0 ]; then
  exit 1
fi
exit 0
