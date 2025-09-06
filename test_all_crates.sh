#!/bin/bash

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