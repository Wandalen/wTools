#!/bin/bash
#
# Pre-commit hook for test organization validation
#
# This script validates that any modified test files follow the systematic
# organization principles before allowing commits. It prevents regression
# to problematic task-based naming patterns.
#
# Usage:
#   1. Copy to .git/hooks/pre-commit
#   2. Make executable: chmod +x .git/hooks/pre-commit
#   3. Test files will be validated on every commit

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${BLUE}🔍 Validating test organization...${NC}"

# Get the root directory of the git repository
REPO_ROOT=$(git rev-parse --show-toplevel)

# Check if we're in a unilang project (look for specific test structure)
if [ ! -d "$REPO_ROOT/tests/unit" ] && [ ! -d "$REPO_ROOT/module/move/unilang/tests/unit" ]; then
  echo -e "${YELLOW}⚠️  Test organization validation skipped (not a unilang project)${NC}"
  exit 0
fi

# Determine tests directory
TESTS_DIR=""
if [ -d "$REPO_ROOT/tests/unit" ]; then
  TESTS_DIR="$REPO_ROOT/tests"
elif [ -d "$REPO_ROOT/module/move/unilang/tests/unit" ]; then
  TESTS_DIR="$REPO_ROOT/module/move/unilang/tests"
else
  echo -e "${YELLOW}⚠️  Could not locate tests directory${NC}"
  exit 0
fi

# Get list of staged test files
STAGED_TEST_FILES=$(git diff --cached --name-only --diff-filter=ACM | grep -E "tests/.*\.rs$" || true)

if [ -z "$STAGED_TEST_FILES" ]; then
  echo -e "${GREEN}✅ No test files modified${NC}"
  exit 0
fi

echo -e "${BLUE}📋 Checking staged test files:${NC}"
echo "$STAGED_TEST_FILES" | sed 's/^/  - /'

# Validation rules (embedded for simplicity)
check_prohibited_patterns() {
  local file="$1"
  local filename=$(basename "$file" .rs)

  # Check for prohibited task-based patterns
  if [[ "$filename" =~ ^(task_|issue_|fix_|bug_|feature_|enhancement_) ]]; then
    echo -e "${RED}❌ Prohibited naming pattern in: $file${NC}"
    echo -e "   File uses task-based naming which violates organization standards"
    echo -e "   Please rename to use feature-based naming (see tests/readme.md)"
    return 1
  fi

  return 0
}

check_directory_structure() {
  local file="$1"
  local relative_path="${file#$TESTS_DIR/}"

  # Extract first directory component
  local first_dir=$(echo "$relative_path" | cut -d'/' -f1)

  # Check if it's in an allowed top-level directory
  case "$first_dir" in
    unit|integration|acceptance|regression|inc|tools)
      return 0
      ;;
    *)
      echo -e "${RED}❌ Invalid directory structure: $file${NC}"
      echo -e "   File is not in an allowed category directory"
      echo -e "   Allowed: unit/, integration/, acceptance/, regression/, inc/, tools/"
      return 1
      ;;
  esac
}

check_depth_limit() {
  local file="$1"
  local relative_path="${file#$TESTS_DIR/}"
  local depth=$(echo "$relative_path" | tr '/' '\n' | wc -l)

  if [ "$depth" -gt 4 ]; then
    echo -e "${RED}❌ Excessive nesting depth: $file${NC}"
    echo -e "   Directory nesting is too deep (max: 4 levels)"
    return 1
  fi

  return 0
}

check_category_specific() {
  local file="$1"
  local relative_path="${file#$TESTS_DIR/}"
  local first_dir=$(echo "$relative_path" | cut -d'/' -f1)
  local filename=$(basename "$file" .rs)

  case "$first_dir" in
    unit)
      # Unit tests should not suggest integration
      if [[ "$filename" =~ (integration|end_to_end) ]]; then
        echo -e "${YELLOW}⚠️  Unit test suggests integration: $file${NC}"
        echo -e "   Consider moving to integration/ or renaming"
        return 1
      fi
      ;;
    acceptance)
      # Acceptance tests should suggest user interaction
      if [[ ! "$filename" =~ (cli|user|scenario) ]]; then
        echo -e "${YELLOW}⚠️  Acceptance test should indicate user scenario: $file${NC}"
        echo -e "   Consider including 'cli', 'user', or 'scenario' in filename"
        return 1
      fi
      ;;
    regression)
      # Regression tests should indicate bug prevention
      if [[ ! "$filename" =~ (regression|fix) ]]; then
        echo -e "${YELLOW}⚠️  Regression test should indicate bug prevention: $file${NC}"
        echo -e "   Consider including 'regression' or 'fix' in filename"
        return 1
      fi
      ;;
  esac

  return 0
}

# Validate each staged test file
VALIDATION_FAILED=false

for file in $STAGED_TEST_FILES; do
  # Convert to absolute path
  abs_file="$REPO_ROOT/$file"

  # Skip if file doesn't exist (might be deleted)
  if [ ! -f "$abs_file" ]; then
    continue
  fi

  # Run all validation checks
  if ! check_prohibited_patterns "$abs_file"; then
    VALIDATION_FAILED=true
  fi

  if ! check_directory_structure "$abs_file"; then
    VALIDATION_FAILED=true
  fi

  if ! check_depth_limit "$abs_file"; then
    VALIDATION_FAILED=true
  fi

  if ! check_category_specific "$abs_file"; then
    VALIDATION_FAILED=true
  fi
done

# Final result
if [ "$VALIDATION_FAILED" = true ]; then
  echo ""
  echo -e "${RED}❌ Test organization validation failed!${NC}"
  echo -e "${BLUE}📖 Please review tests/readme.md for organization standards${NC}"
  echo -e "${BLUE}🔧 Use feature-based naming instead of task-based patterns${NC}"
  echo ""
  echo -e "${YELLOW}To bypass this check (not recommended):${NC}"
  echo -e "${YELLOW}  git commit --no-verify${NC}"
  exit 1
else
  echo -e "${GREEN}✅ All test files comply with organization standards${NC}"
  exit 0
fi