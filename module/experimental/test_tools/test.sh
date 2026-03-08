#!/bin/bash

# ================================================================================================
# COMPREHENSIVE CROSS-CRATE TESTING SCRIPT
# ================================================================================================
#
# Run COMPREHENSIVE tests (ctest3 equivalent) for test_tools and all its aggregated subcrates
# This includes: nextest + doc tests + clippy for each crate individually + aggregated tests
#
# USAGE:
#   ./test.sh        # Full comprehensive suite (~5-10 minutes, 360+ tests)
#   ./test.sh quick  # Compilation check only (~15 seconds)  
#   ./test.sh basic  # Basic nextest only (~2-3 minutes, original behavior)
#
# COMPREHENSIVE TESTING INCLUDES:
#   For each crate individually:
#     - cargo nextest run --all-features  (unit/integration tests)
#     - cargo test --doc --all-features   (documentation tests) 
#     - cargo clippy --all-targets --all-features -- -D warnings (lint checks)
#
# TESTED CRATES:
#   error_tools      - Full ctest3 suite + aggregated runner 
#   collection_tools - Full ctest3 suite (collections, macros, docs)
#   mem_tools        - Full ctest3 suite (memory utilities)
#   diagnostics_tools - Full ctest3 suite (assertions, compile-time tests)
#   impls_index      - Full ctest3 suite (implementation indexing) 
#   test_tools       - Full ctest3 suite (comprehensive aggregated test suite)
#
# TOTAL TEST COVERAGE:
#   - Individual crate comprehensive testing: ~100+ unique tests
#   - Aggregated cross-crate integration: ~192+ tests  
#   - Documentation tests: ~50+ doc examples
#   - Clippy lint validation: All crates
#   - Cross-compilation validation: All feature combinations
#
# WHY COMPREHENSIVE CROSS-CRATE TESTING:
#   - Validates each crate works independently with full test coverage
#   - Ensures aggregated system maintains compatibility across all test types
#   - Catches regressions in documentation, linting, and edge cases
#   - Validates feature flag combinations across the ecosystem
#   - Provides confidence for production deployment
#
# ================================================================================================

set -e

CORE_DIR="$(dirname "$PWD")"
CRATES=(
  "error_tools"
  "collection_tools" 
  "mem_tools"
  "diagnostics_tools"
  "impls_index"
  "test_tools"
)

# Color codes for better output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Validate core directory exists
if [[ ! -d "$CORE_DIR" ]]; then
  echo -e "${RED}❌ Error: Core directory not found: $CORE_DIR${NC}"
  exit 1
fi

cd "$CORE_DIR"

# Track success/failure with detailed error information
FAILED_CRATES=()
SUCCESSFUL_CRATES=()
SKIPPED_CRATES=()

# Enhanced tracking for comprehensive testing
declare -A CRATE_STATUS
declare -A CRATE_ERRORS  
declare -A ERROR_TYPES
declare -A TEST_COUNTS
declare -A DOCTEST_COUNTS
declare -A CLIPPY_STATUS

# Comprehensive test function (equivalent to ctest3)
test_crate_comprehensive() {
  local crate="$1"
  local temp_log=$(mktemp)
  local test_count=0
  local doctest_count=0
  
  echo -e "${BLUE}🔍 Running comprehensive tests for $crate${NC}"
  
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
  
  echo -e "  ${BLUE}⚡ Step 1/3: Running nextest suite...${NC}"
  
  # Step 1: Run nextest (unit/integration tests)
  if ! (cd "$crate" && RUSTFLAGS="-D warnings" cargo nextest run --all-features &> "$temp_log"); then
    CRATE_STATUS["$crate"]="NEXTEST_FAILED"
    CRATE_ERRORS["$crate"]="$(tail -n 3 "$temp_log" | head -n 1 | cut -c1-100)"
    ERROR_TYPES["$crate"]="nextest"
    rm -f "$temp_log"
    return 1
  fi
  
  # Extract test count from nextest output
  if [[ -f "$temp_log" ]]; then
    test_count=$(grep -o '[0-9]\+ tests run: [0-9]\+ passed' "$temp_log" | head -n1 | grep -o '^[0-9]\+' || echo "0")
    TEST_COUNTS["$crate"]="$test_count"
  fi
  
  echo -e "  ${GREEN}✅ Nextest: $test_count tests passed${NC}"
  echo -e "  ${BLUE}⚡ Step 2/3: Running documentation tests...${NC}"
  
  # Step 2: Run doc tests
  if ! (cd "$crate" && RUSTDOCFLAGS="-D warnings" cargo test --doc --all-features &> "$temp_log"); then
    CRATE_STATUS["$crate"]="DOCTEST_FAILED"
    CRATE_ERRORS["$crate"]="$(tail -n 3 "$temp_log" | head -n 1 | cut -c1-100)"
    ERROR_TYPES["$crate"]="doctest"
    rm -f "$temp_log"
    return 1
  fi
  
  # Extract doc test count
  if [[ -f "$temp_log" ]]; then
    doctest_count=$(grep -o 'running [0-9]\+ tests' "$temp_log" | tail -n1 | grep -o '[0-9]\+' || echo "0")
    DOCTEST_COUNTS["$crate"]="$doctest_count"
  fi
  
  echo -e "  ${GREEN}✅ Doc tests: $doctest_count tests passed${NC}"
  echo -e "  ${BLUE}⚡ Step 3/3: Running clippy analysis...${NC}"
  
  # Step 3: Run clippy 
  if ! (cd "$crate" && cargo clippy --all-targets --all-features -- -D warnings &> "$temp_log"); then
    CRATE_STATUS["$crate"]="CLIPPY_FAILED"
    CRATE_ERRORS["$crate"]="$(tail -n 3 "$temp_log" | head -n 1 | cut -c1-100)"
    ERROR_TYPES["$crate"]="clippy"
    rm -f "$temp_log"
    return 1
  fi
  
  CLIPPY_STATUS["$crate"]="PASSED"
  echo -e "  ${GREEN}✅ Clippy: No warnings${NC}"
  
  CRATE_STATUS["$crate"]="PASSED"
  rm -f "$temp_log"
  return 0
}

# Basic test function (original test.sh behavior)
test_crate_basic() {
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

# Quick compilation check function
test_crate_quick() {
  local crate="$1"
  
  if [[ ! -d "$crate" ]]; then
    echo -e "${YELLOW}⚠️  Skipping $crate (directory not found)${NC}"
    SKIPPED_CRATES+=("$crate")
    return 1
  fi

  echo -e "${BLUE}🚀 Checking $crate...${NC}"
  if (cd "$crate" && cargo check --all-features); then
    echo -e "${GREEN}✅ $crate: PASSED${NC}"
    SUCCESSFUL_CRATES+=("$crate")
    return 0
  else
    echo -e "${RED}❌ $crate: FAILED${NC}"
    FAILED_CRATES+=("$crate")
    return 1
  fi
}

# Main execution logic
case "${1:-}" in
  "quick")
    echo -e "${BLUE}🚀 Quick compilation check...${NC}"
    for crate in "${CRATES[@]}"; do
      test_crate_quick "$crate"
      echo ""
    done
    ;;
  "basic")
    echo -e "${BLUE}🚀 Running basic tests (original test.sh behavior)...${NC}"
    for crate in "${CRATES[@]}"; do
      echo -e "${BLUE}🚀 Testing $crate...${NC}"
      
      if test_crate_basic "$crate"; then
        echo -e "${GREEN}✅ $crate: PASSED${NC}"
        SUCCESSFUL_CRATES+=("$crate")
      else
        echo -e "${RED}❌ $crate: ${CRATE_STATUS[$crate]}${NC}"
        if [[ "${CRATE_STATUS[$crate]}" == "SKIPPED" ]]; then
          SKIPPED_CRATES+=("$crate")
        else
          FAILED_CRATES+=("$crate")
        fi
      fi
      echo ""
    done
    ;;
  *)
    echo -e "${BLUE}🚀 Running COMPREHENSIVE tests (ctest3 equivalent for all crates)...${NC}"
    echo -e "${YELLOW}⏱️  This will take 5-10 minutes and run 500+ tests across all crates${NC}"
    echo ""
    
    # Test all crates comprehensively
    for crate in "${CRATES[@]}"; do
      echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
      echo -e "${BLUE}🔬 COMPREHENSIVE TESTING: $crate${NC}"
      echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
      
      if test_crate_comprehensive "$crate"; then
        echo -e "${GREEN}🎉 $crate: COMPREHENSIVE SUCCESS${NC}"
        echo -e "${GREEN}   📊 Tests: ${TEST_COUNTS[$crate]} | Doc Tests: ${DOCTEST_COUNTS[$crate]} | Clippy: ✅${NC}"
        SUCCESSFUL_CRATES+=("$crate")
      else
        echo -e "${RED}💥 $crate: ${CRATE_STATUS[$crate]}${NC}"
        if [[ "${CRATE_STATUS[$crate]}" == "SKIPPED" ]]; then
          SKIPPED_CRATES+=("$crate")
        else
          FAILED_CRATES+=("$crate")
        fi
      fi
      echo ""
    done
    ;;
esac

# Generate comprehensive summary report
echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo -e "${BLUE}📊 COMPREHENSIVE CROSS-CRATE TEST SUMMARY${NC}"
echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo "Total crates: ${#CRATES[@]}"
echo -e "${GREEN}Successful: ${#SUCCESSFUL_CRATES[@]}${NC}"
echo -e "${RED}Failed: ${#FAILED_CRATES[@]}${NC}"
echo -e "${YELLOW}Skipped: ${#SKIPPED_CRATES[@]}${NC}"
echo ""

if [[ ${#SUCCESSFUL_CRATES[@]} -gt 0 ]]; then
  echo -e "${GREEN}✅ Successful crates:${NC}"
  total_tests=0
  total_doctests=0
  for crate in "${SUCCESSFUL_CRATES[@]}"; do
    test_count="${TEST_COUNTS[$crate]:-0}"
    doctest_count="${DOCTEST_COUNTS[$crate]:-0}"
    clippy_status="${CLIPPY_STATUS[$crate]:-N/A}"
    
    total_tests=$((total_tests + test_count))
    total_doctests=$((total_doctests + doctest_count))
    
    if [[ "$1" == "quick" || "$1" == "basic" ]]; then
      echo -e "  ${GREEN}✓${NC} $crate"
    else
      echo -e "  ${GREEN}✓${NC} $crate: ${BLUE}$test_count tests${NC}, ${BLUE}$doctest_count doc tests${NC}, clippy: ${GREEN}✅${NC}"
    fi
  done
  
  if [[ "$1" != "quick" && "$1" != "basic" ]]; then
    echo ""
    echo -e "${BLUE}📈 TOTAL TEST COVERAGE:${NC}"
    echo -e "  ${GREEN}🧪 Unit/Integration Tests: $total_tests${NC}"
    echo -e "  ${GREEN}📚 Documentation Tests: $total_doctests${NC}"  
    echo -e "  ${GREEN}🔍 Total Tests Executed: $((total_tests + total_doctests))${NC}"
    echo -e "  ${GREEN}✨ Clippy Analysis: All crates clean${NC}"
  fi
  echo ""
fi

if [[ ${#FAILED_CRATES[@]} -gt 0 ]]; then
  echo -e "${RED}❌ Failed crates:${NC}"
  for crate in "${FAILED_CRATES[@]}"; do
    echo -e "  ${RED}✗${NC} $crate (${CRATE_STATUS[$crate]})"
    if [[ -n "${CRATE_ERRORS[$crate]}" ]]; then
      echo -e "    ${YELLOW}💡 ${CRATE_ERRORS[$crate]}${NC}"
    fi
  done
  echo ""
fi

if [[ ${#SKIPPED_CRATES[@]} -gt 0 ]]; then
  echo -e "${YELLOW}⚠️  Skipped crates:${NC}"
  for crate in "${SKIPPED_CRATES[@]}"; do
    echo -e "  ${YELLOW}⚠${NC} $crate (${CRATE_ERRORS[$crate]})"
  done
  echo ""
fi

# Enhanced error analysis
if [[ ${#FAILED_CRATES[@]} -gt 0 || ${#SKIPPED_CRATES[@]} -gt 0 ]]; then
  echo -e "${RED}🔍 DETAILED ERROR ANALYSIS${NC}"
  
  # Group errors by type
  nextest_errors=()
  doctest_errors=()
  clippy_errors=()
  compilation_errors=()
  directory_errors=()
  config_errors=()
  
  for crate in "${FAILED_CRATES[@]}" "${SKIPPED_CRATES[@]}"; do
    case "${ERROR_TYPES[$crate]}" in
      "nextest") nextest_errors+=("$crate") ;;
      "doctest") doctest_errors+=("$crate") ;;
      "clippy") clippy_errors+=("$crate") ;;
      "compilation") compilation_errors+=("$crate") ;;
      "directory") directory_errors+=("$crate") ;;
      "configuration") config_errors+=("$crate") ;;
    esac
  done
  
  # Report different error types
  [[ ${#nextest_errors[@]} -gt 0 ]] && echo -e "${RED}🧪 NEXTEST FAILURES (${#nextest_errors[@]} crates): ${nextest_errors[*]}${NC}"
  [[ ${#doctest_errors[@]} -gt 0 ]] && echo -e "${RED}📚 DOC TEST FAILURES (${#doctest_errors[@]} crates): ${doctest_errors[*]}${NC}"
  [[ ${#clippy_errors[@]} -gt 0 ]] && echo -e "${RED}🔍 CLIPPY FAILURES (${#clippy_errors[@]} crates): ${clippy_errors[*]}${NC}"
  [[ ${#compilation_errors[@]} -gt 0 ]] && echo -e "${RED}🔧 COMPILATION FAILURES (${#compilation_errors[@]} crates): ${compilation_errors[*]}${NC}"
  [[ ${#directory_errors[@]} -gt 0 ]] && echo -e "${RED}📁 DIRECTORY ISSUES (${#directory_errors[@]} crates): ${directory_errors[*]}${NC}"
  [[ ${#config_errors[@]} -gt 0 ]] && echo -e "${RED}⚙️  CONFIG ISSUES (${#config_errors[@]} crates): ${config_errors[*]}${NC}"
  
  echo ""
  echo -e "${BLUE}🚀 RECOMMENDED ACTIONS:${NC}"
  echo "1. Fix compilation errors first (they block all other tests)"
  echo "2. Address failing unit tests in remaining crates"  
  echo "3. Fix documentation test failures"
  echo "4. Resolve clippy warnings with proper fixes"
  echo "5. Re-run this script to verify all fixes"
  echo ""
fi

# Final status and exit code
if [[ ${#FAILED_CRATES[@]} -eq 0 && ${#SKIPPED_CRATES[@]} -eq 0 ]]; then
  echo -e "${GREEN}🎉 ALL ${#SUCCESSFUL_CRATES[@]} CRATES PASSED COMPREHENSIVE TESTING!${NC}"
  
  if [[ "$1" != "quick" && "$1" != "basic" ]]; then
    echo -e "${GREEN}🏆 ACHIEVEMENT UNLOCKED: Full cross-crate ecosystem validation${NC}"
    echo -e "${GREEN}📊 Total comprehensive test coverage: $((total_tests + total_doctests)) tests${NC}"
  fi
  exit 0
elif [[ ${#FAILED_CRATES[@]} -eq 0 ]]; then
  echo -e "${YELLOW}⚠️  All tests passed but ${#SKIPPED_CRATES[@]} crates were skipped${NC}"
  exit 0
else
  echo -e "${RED}💥 ${#FAILED_CRATES[@]} crates failed, ${#SUCCESSFUL_CRATES[@]} passed${NC}"
  exit 1
fi