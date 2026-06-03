#!/usr/bin/env bash
set -euo pipefail

VERSIONS=(2.9.4 2.10.1 2.11.4 2.12.2 2.13.1 2.14.0 2.15.2 2.16.1 2.17.0 2.18.0)
OLD_VERSIONS=(2.9.4 2.10.1)

ASDF_INSTALLS="${ASDF_DATA_DIR:-$HOME/.asdf}/installs"
SNFORGE_VERSION="${SNFORGE_VERSION:-0.43.0}"
SNFORGE_BIN="$ASDF_INSTALLS/starknet-foundry/$SNFORGE_VERSION/bin"

result_versions=()
result_statuses=()

for version in "${VERSIONS[@]}"; do
    echo ""
    echo "=========================================="
    echo "Testing with Scarb $version"
    echo "=========================================="

    if [[ ! -d "$ASDF_INSTALLS/scarb/$version" ]]; then
        echo "Installing Scarb $version via asdf..."
        asdf install scarb "$version"
    fi

    scarb_bin="$ASDF_INSTALLS/scarb/$version/bin"
    if [[ ! -x "$scarb_bin/scarb" ]]; then
        echo "ERROR: scarb binary not found at $scarb_bin/scarb" >&2
        result_versions+=("$version")
        result_statuses+=("FAIL")
        continue
    fi

    if printf '%s\n' "${OLD_VERSIONS[@]}" | grep -qxF "$version"; then
        cmd="cargo test --release"
    else
        cmd="cargo test --release --features allows-excluding-macros"
    fi

    echo "snforge: $SNFORGE_VERSION  scarb: $version"
    echo "Running: $cmd"
    if PATH="$scarb_bin:$SNFORGE_BIN:$PATH" $cmd; then
        result_versions+=("$version")
        result_statuses+=("PASS")
    else
        result_versions+=("$version")
        result_statuses+=("FAIL")
    fi
done

echo ""
echo "=========================================="
echo "Results"
echo "=========================================="
all_passed=true
for i in "${!result_versions[@]}"; do
    version="${result_versions[$i]}"
    status="${result_statuses[$i]}"
    echo "  Scarb $version: $status"
    if [[ "$status" != "PASS" ]]; then
        all_passed=false
    fi
done

if $all_passed; then
    echo ""
    echo "All versions passed."
    exit 0
else
    echo ""
    echo "Some versions failed."
    exit 1
fi
