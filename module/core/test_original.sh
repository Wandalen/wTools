#!/bin/bash

# ================================================================================================
# CROSS-CRATE TESTING SCRIPT
# ================================================================================================
#
# Run tests for test_tools and all its aggregated subcrates to detect cross-crate compatibility 
# issues. Changes in one crate can break others through the test aggregation system.
#
# USAGE:
#   ./test.sh       # Full test suite (~2-3 minutes, ~269+ tests)
#   ./test.sh quick # Compilation check only (~15 seconds)
#
# TESTED CRATES:
#   error_tools      - 18 tests (17 + aggregated runner)
#   collection_tools - 37+ tests (collection types, macros)  
#   mem_tools        - 4+ tests (memory utilities)
#   diagnostics_tools - 17+ tests (assertions)
#   impls_index      - 18+ tests (implementation indexing)
#   test_tools       - 175+ tests (aggregated test suite)
#
# WHY CROSS-CRATE TESTING:
#   - test_tools provides standalone implementations of functionality from other crates
#   - Individual crates use test_tools for testing infrastructure  
#   - the_module alias pattern enables dual-context testing
#   - Changes in standalone.rs can break individual crate tests
#   - Changes in individual crates can break test_tools aggregation
#
# DOCUMENTATION:
#   See CROSS_CRATE_TESTING.md for comprehensive architecture and troubleshooting guide
#
# ================================================================================================

set -e

CORE_DIR="/home/user1/pro/lib/wTools/module/core"
CRATES=(
  "error_tools"
  "collection_tools" 
  "mem_tools"
  "diagnostics_tools"
  "impls_index"
  "test_tools"
)

# Validate core directory exists
if [[ ! -d "$CORE_DIR" ]]; then
  echo "❌ Error: Core directory not found: $CORE_DIR"
  exit 1
fi

cd "$CORE_DIR"

# Track success/failure with detailed error information
FAILED_CRATES=()
SUCCESSFUL_CRATES=()
SKIPPED_CRATES=()

# Error categorization tracking
declare -A CRATE_STATUS
declare -A CRATE_ERRORS
declare -A ERROR_TYPES

# Enhanced test function with error categorization
test_crate_enhanced() {
  local crate="$1"
  local temp_log=$(mktemp)
  
  # Check if directory exists
  if [[ ! -d "$crate" ]]; then
    CRATE_STATUS["$crate"]="SKIPPED"
    CRATE_ERRORS["$crate"]="Directory not found"
    ERROR_TYPES["$crate"]="directory"
    return 1
  fi
  
  # Check if Cargo.toml exists
  if [[ ! -f "$crate/Cargo.toml" ]]; then
    CRATE_STATUS["$crate"]="SKIPPED"
    CRATE_ERRORS["$crate"]="No Cargo.toml found"
    ERROR_TYPES["$crate"]="configuration"
    return 1
  fi
  
  # Try compilation first
  if ! (cd "$crate" && cargo check --all-features &> "$temp_log"); then
    CRATE_STATUS["$crate"]="COMPILATION_FAILED"
    CRATE_ERRORS["$crate"]="$(tail -n 3 "$temp_log" | head -n 1 | cut -c1-100)"
    ERROR_TYPES["$crate"]="compilation"
    rm -f "$temp_log"
    return 1
  fi
  
  # Try running tests
  if ! (cd "$crate" && RUSTFLAGS="-D warnings" cargo nextest run --all-features &> "$temp_log"); then
    CRATE_STATUS["$crate"]="TEST_FAILED"
    CRATE_ERRORS["$crate"]="$(tail -n 3 "$temp_log" | head -n 1 | cut -c1-100)"
    ERROR_TYPES["$crate"]="test"
    rm -f "$temp_log"
    return 1
  fi
  
  CRATE_STATUS["$crate"]="PASSED"
  rm -f "$temp_log"
  return 0
}

if [[ "${1:-}" == "quick" ]]; then
  echo "🚀 Quick compilation check..."
  for crate in "${CRATES[@]}"; do
    if [[ ! -d "$crate" ]]; then
      echo "⚠️  Skipping $crate (directory not found)"
      SKIPPED_CRATES+=("$crate")
      continue
    fi

    echo "🚀 Checking $crate..."
    if (cd "$crate" && cargo check --all-features); then
      echo "✅ $crate: PASSED"
      SUCCESSFUL_CRATES+=("$crate")
    else
      echo "❌ $crate: FAILED"
      FAILED_CRATES+=("$crate")
    fi
    echo ""
  done
else
  echo "🚀 Running all tests with enhanced error analysis..."
  
  # Test all crates and collect detailed results
  for crate in "${CRATES[@]}"; do
    echo "🚀 Testing $crate..."
    
    if test_crate_enhanced "$crate"; then
      echo "✅ $crate: PASSED"
      SUCCESSFUL_CRATES+=("$crate")
    else
      echo "❌ $crate: ${CRATE_STATUS[$crate]}"
      if [[ "${CRATE_STATUS[$crate]}" == "SKIPPED" ]]; then
        SKIPPED_CRATES+=("$crate")
      else
        FAILED_CRATES+=("$crate")
      fi
    fi
    echo ""
  done
fi

# Generate summary report
echo "=== CROSS-CRATE TEST SUMMARY ==="
echo "Total crates: ${#CRATES[@]}"
echo "Successful: ${#SUCCESSFUL_CRATES[@]}"
echo "Failed: ${#FAILED_CRATES[@]}"
echo "Skipped: ${#SKIPPED_CRATES[@]}"
echo ""

if [[ ${#SUCCESSFUL_CRATES[@]} -gt 0 ]]; then
  echo "✅ Successful crates:"
  for crate in "${SUCCESSFUL_CRATES[@]}"; do
    echo "  - $crate"
  done
  echo ""
fi

if [[ ${#FAILED_CRATES[@]} -gt 0 ]]; then
  echo "❌ Failed crates:"
  for crate in "${FAILED_CRATES[@]}"; do
    echo "  - $crate"
  done
  echo ""
fi

if [[ ${#SKIPPED_CRATES[@]} -gt 0 ]]; then
  echo "⚠️  Skipped crates:"
  for crate in "${SKIPPED_CRATES[@]}"; do
    echo "  - $crate"
  done
  echo ""
fi

# Enhanced error analysis with categorization and recovery guidance
if [[ ${#FAILED_CRATES[@]} -gt 0 || ${#SKIPPED_CRATES[@]} -gt 0 ]]; then
  echo "=== DETAILED ERROR ANALYSIS ==="
  
  # Group errors by type
  compilation_errors=()
  test_errors=()
  directory_errors=()
  config_errors=()
  
  for crate in "${FAILED_CRATES[@]}" "${SKIPPED_CRATES[@]}"; do
    case "${ERROR_TYPES[$crate]}" in
      "compilation") compilation_errors+=("$crate") ;;
      "test") test_errors+=("$crate") ;;
      "directory") directory_errors+=("$crate") ;;
      "configuration") config_errors+=("$crate") ;;
    esac
  done
  
  # Report compilation errors
  if [[ ${#compilation_errors[@]} -gt 0 ]]; then
    echo ""
    echo "🔴 COMPILATION ERRORS (${#compilation_errors[@]} crates):"
    for crate in "${compilation_errors[@]}"; do
      echo "  - $crate: ${CRATE_ERRORS[$crate]}"
    done
    echo "  💡 Fix: Address syntax, type, or dependency issues"
    echo "  🔧 Command: cd $crate && cargo check --all-features"
  fi
  
  # Report test failures
  if [[ ${#test_errors[@]} -gt 0 ]]; then
    echo ""
    echo "🔴 TEST FAILURES (${#test_errors[@]} crates):"
    for crate in "${test_errors[@]}"; do
      echo "  - $crate: ${CRATE_ERRORS[$crate]}"
    done
    echo "  💡 Fix: Review failing tests and fix implementation"
    echo "  🔧 Command: cd $crate && RUSTFLAGS=\"-D warnings\" cargo nextest run --all-features"
  fi
  
  # Report directory issues
  if [[ ${#directory_errors[@]} -gt 0 ]]; then
    echo ""
    echo "🔴 DIRECTORY ISSUES (${#directory_errors[@]} crates):"
    for crate in "${directory_errors[@]}"; do
      echo "  - $crate: ${CRATE_ERRORS[$crate]}"
    done
    echo "  💡 Fix: Ensure all crate directories exist in core/"
    echo "  🔧 Command: ls -la core/ # Verify directory structure"
  fi
  
  # Report configuration issues
  if [[ ${#config_errors[@]} -gt 0 ]]; then
    echo ""
    echo "🔴 CONFIGURATION ISSUES (${#config_errors[@]} crates):"
    for crate in "${config_errors[@]}"; do
      echo "  - $crate: ${CRATE_ERRORS[$crate]}"
    done
    echo "  💡 Fix: Ensure Cargo.toml exists and is valid"
    echo "  🔧 Command: ls -la $crate/Cargo.toml"
  fi
  
  echo ""
  echo "🚀 RECOMMENDED NEXT STEPS:"
  echo "1. Fix compilation errors first (they block testing)"
  echo "2. Address test failures in remaining crates"
  echo "3. Re-run this script to verify fixes"
  echo "4. Use './test.sh quick' for fast compilation checks"
  echo ""
fi

# Final status and exit code
if [[ ${#FAILED_CRATES[@]} -eq 0 && ${#SKIPPED_CRATES[@]} -eq 0 ]]; then
  echo "🎉 All ${#SUCCESSFUL_CRATES[@]} crates passed!"
  exit 0
elif [[ ${#FAILED_CRATES[@]} -eq 0 ]]; then
  echo "⚠️  All tests passed but ${#SKIPPED_CRATES[@]} crates were skipped"
  exit 0
else
  echo "💥 ${#FAILED_CRATES[@]} crates failed, ${#SUCCESSFUL_CRATES[@]} passed"
  exit 1
fi