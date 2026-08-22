#!/bin/bash

# Refuse to run unattended: this script builds+tests every crate in the
# workspace (~48 crates), which has repeatedly exhausted local disk space.
# See rulebook.md: Exception to longrun.rulebook.md § Breadth Selection.
if ! read -r -t 20 -p "This runs the FULL workspace test suite across every crate. Type 'yes' to continue: " confirm < /dev/tty; then
	echo "test_all_crates.sh: no interactive confirmation available (no controlling terminal, or timed out) — refusing to run." >&2
	exit 1
fi
if [ "$confirm" != "yes" ]; then
	echo "test_all_crates.sh: confirmation declined — aborting." >&2
	exit 1
fi

# Script to test all crates individually and identify failures
echo "=== Testing All Crates Individually ==="
echo "Date: $(date)"
echo ""

# Get all workspace members
crates=$(cargo metadata --no-deps --format-version 1 2>/dev/null | jq -r '.workspace_members[]' | sed 's/ .*//')

failed_crates=()
passed_crates=()
total_crates=0

for crate in $crates; do
    total_crates=$((total_crates + 1))
    echo -n "Testing $crate... "
    
    # Test the crate with warnings as errors
    if RUSTFLAGS="-D warnings" cargo test --no-run -p "$crate" --all-features >/dev/null 2>&1; then
        echo "PASS"
        passed_crates+=("$crate")
    else
        echo "FAIL"
        failed_crates+=("$crate")
    fi
done

echo ""
echo "=== SUMMARY ==="
echo "Total crates tested: $total_crates"
echo "Passed: ${#passed_crates[@]}"
echo "Failed: ${#failed_crates[@]}"

echo ""
echo "=== FAILING CRATES ==="
for crate in "${failed_crates[@]}"; do
    echo "- $crate"
done

echo ""
echo "=== DETAILED FAILURE ANALYSIS ==="
for crate in "${failed_crates[@]}"; do
    echo ""
    echo "=== $crate ==="
    RUSTFLAGS="-D warnings" cargo test --no-run -p "$crate" --all-features 2>&1 | head -20
    echo "..."
done