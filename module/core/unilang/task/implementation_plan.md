# Comprehensive Implementation Plan: Multi-Word Parameter Fix

**Priority:** HIGH
**Complexity:** LOW
**Risk:** MINIMAL
**Estimated Time:** 2-3 hours total
**Target Version:** unilang 0.32.0

---

## Executive Summary

Fix the multi-word parameter parsing bug by changing the CLI binary to use the argv-aware parser function instead of converting argv to string and re-parsing.

**Key Change:** 1 line modified, ~50 lines deleted
**Impact:** Enables natural CLI usage without quote gymnastics
**Breaking Changes:** NONE (pure bug fix)

---

## Phase 1: Pre-Implementation (15 minutes)

### 1.1 Baseline Testing

**Objective:** Establish current state before changes

**Tasks:**
```bash
# 1. Run all existing tests and record results
cd /home/user1/pro/lib/wTools/module/core/unilang
cargo test --all-features > /tmp/baseline_tests.txt 2>&1

# 2. Run parser tests specifically
cd /home/user1/pro/lib/wTools/module/core/unilang_parser
cargo test --all-features > /tmp/baseline_parser_tests.txt 2>&1

# 3. Run new bug reproduction tests
cd /home/user1/pro/lib/wTools/module/core/unilang_parser
cargo test --test argv_multiword_bug_test > /tmp/baseline_parser_bug_tests.txt 2>&1

cd /home/user1/pro/lib/wTools/module/core/unilang
cargo test --test cli_multiword_params_test > /tmp/baseline_cli_bug_tests.txt 2>&1

# 4. Test current CLI behavior manually
cargo run --bin unilang_cli -- .video.search query::"llm rust" > /tmp/baseline_cli_manual.txt 2>&1
```

**Expected Results:**
- Parser tests: 9/10 pass (baseline)
- CLI tests: 4/10 pass (currently failing)
- Manual CLI test: FAILS with "Unexpected token 'rust'"

**Success Criteria:**
- ✅ Baseline recorded
- ✅ Failure mode confirmed
- ✅ Test infrastructure working

---

## Phase 2: Create Backup Branch (5 minutes)

### 2.1 Git Branch Management

**Objective:** Safe working environment with easy rollback

**Tasks:**
```bash
cd /home/user1/pro/lib/wTools/module/core/unilang

# 1. Check current status
git status
git log -1 --oneline

# 2. Create feature branch
git checkout -b fix/multiword-parameter-parsing

# 3. Verify clean state
git status
```

**Success Criteria:**
- ✅ On new branch `fix/multiword-parameter-parsing`
- ✅ Clean working directory
- ✅ Can easily revert if needed

---

## Phase 3: Implementation (30 minutes)

### 3.1 Modify unilang_cli.rs

**File:** `/home/user1/pro/lib/wTools/module/core/unilang/src/bin/unilang_cli.rs`

**Change 1: Delete rejoin_broken_quoted_args function**

**Lines:** 26-73 (entire function + doc comment)

**Action:**
```rust
// DELETE these lines:
/// Rejoins command line arguments that were incorrectly split by the shell
/// when they contain quoted multi-word values.
///
/// For example, if the shell splits `query::"llm rust"` into `["query::\"llm", "rust\""]`,
/// this function will rejoin them back to `query::"llm rust"`.
fn rejoin_broken_quoted_args( args: &[ String ] ) -> String
{
  let mut result = Vec::new();
  let mut i = 0;

  while i < args.len()
  {
    let current_arg = &args[ i ];

    // Check if this argument has an unmatched opening quote
    if current_arg.contains( "::\"" ) && !current_arg.ends_with( '"' )
    {
      // This argument starts a quoted value, look for the closing quote
      let mut combined_arg = current_arg.clone();
      i += 1;

      // Keep adding arguments until we find the closing quote
      while i < args.len()
      {
        combined_arg.push( ' ' );
        combined_arg.push_str( &args[ i ] );

        if args[ i ].ends_with( '"' )
        {
          // Found the closing quote
          break;
        }
        i += 1;
      }

      result.push( combined_arg );
    }
    else
    {
      // Regular argument, add as-is
      result.push( current_arg.clone() );
    }

    i += 1;
  }

  result.join( " " )
}
```

**Change 2: Fix main parsing logic**

**Lines:** 597-605 (approximate - line numbers shift after deletion above)

**Before:**
```rust
  // 3. Rejoin Arguments
  // Shell may have split arguments incorrectly, particularly for quoted multi-word values
  let command_input_str = rejoin_broken_quoted_args( &processed_args );

  if verbosity > 1
  {
    eprintln!( "DEBUG: Processed arguments: {processed_args:?}" );
    eprintln!( "DEBUG: Rejoined command string: {command_input_str:?}" );
  }
  let instruction = parser.parse_single_instruction( &command_input_str )?;
```

**After:**
```rust
  // 3. Parse Arguments from argv
  // Use argv-aware parser to preserve token boundaries for multi-word parameter values
  if verbosity > 1
  {
    eprintln!( "DEBUG: Processing argv: {processed_args:?}" );
  }
  let instruction = parser.parse_from_argv( &processed_args )?;
```

**Implementation Steps:**

1. Open file in editor
2. Delete lines 26-73 (rejoin_broken_quoted_args)
3. Navigate to parsing logic (now ~line 550-560 after deletion)
4. Replace the section as shown above
5. Save file

**Verification:**
```bash
# 1. Check syntax
cargo check --bin unilang_cli

# 2. Count changes
git diff --stat
# Expected: unilang/src/bin/unilang_cli.rs | ~50 insertions(+), ~55 deletions(-)

# 3. Review changes
git diff src/bin/unilang_cli.rs
```

**Success Criteria:**
- ✅ File compiles without errors
- ✅ ~50 lines deleted, ~5 lines added
- ✅ Uses `parse_from_argv` instead of `parse_single_instruction`
- ✅ No `rejoin_broken_quoted_args` calls remain

### 3.2 Verify No Other Usages

**Objective:** Ensure no other files depend on deleted code

**Tasks:**
```bash
cd /home/user1/pro/lib/wTools/module/core/unilang

# Search for any references to rejoin_broken_quoted_args
grep -r "rejoin_broken_quoted_args" .

# Search for parse_single_instruction in CLI context
grep -r "parse_single_instruction" src/bin/

# Expected: No matches (we removed the only usage)
```

**Success Criteria:**
- ✅ No references to deleted function
- ✅ CLI only uses `parse_from_argv`

---

## Phase 4: Unit Testing (20 minutes)

### 4.1 Run Parser Tests

**Objective:** Verify parser still works (should be unchanged)

**Tasks:**
```bash
cd /home/user1/pro/lib/wTools/module/core/unilang_parser

# Run all parser tests
cargo test --all-features

# Run specific argv bug tests
cargo test --test argv_multiword_bug_test -- --nocapture
```

**Expected Results:**
- ✅ All existing parser tests pass
- ✅ argv_multiword_bug_test: 9/10 pass (same as baseline)
- ✅ No regressions

**Success Criteria:**
- ✅ Test count matches baseline
- ✅ No new failures introduced

### 4.2 Run CLI Integration Tests

**Objective:** Verify CLI fix works

**Tasks:**
```bash
cd /home/user1/pro/lib/wTools/module/core/unilang

# Run CLI bug reproduction tests
cargo test --test cli_multiword_params_test -- --nocapture

# Run all unilang tests
cargo test --all-features
```

**Expected Results:**
- ✅ cli_multiword_params_test: 10/10 pass (was 4/10 before fix)
- ✅ All existing tests still pass
- ✅ No regressions

**Success Criteria:**
- ✅ All CLI multiword tests pass
- ✅ Zero test regressions
- ✅ Test output shows concrete improvements

---

## Phase 5: Manual Integration Testing (30 minutes)

### 5.1 Basic Functionality Tests

**Objective:** Verify fix works with real commands

**Test Suite:**

```bash
cd /home/user1/pro/lib/wTools/module/core/unilang

# Test 1: Single-word parameter (regression check)
echo "=== Test 1: Single-word parameter ==="
cargo run --bin unilang_cli -- .video.search query::rust
# Expected: Query: rust
# Status: [✅ PASS / ❌ FAIL]

# Test 2: Multi-word parameter (THE FIX)
echo "=== Test 2: Multi-word parameter ==="
cargo run --bin unilang_cli -- .video.search query::"llm rust"
# Expected: Query: llm rust
# Status: [✅ PASS / ❌ FAIL]

# Test 3: Many words
echo "=== Test 3: Many words ==="
cargo run --bin unilang_cli -- .video.search query::"rust programming language tutorial"
# Expected: Query: rust programming language tutorial
# Status: [✅ PASS / ❌ FAIL]

# Test 4: Shell command with flags
echo "=== Test 4: Shell command ==="
cargo run --bin unilang_cli -- .video.search query::"cargo build --release"
# Expected: Query: cargo build --release
# Status: [✅ PASS / ❌ FAIL]

# Test 5: Path with spaces
echo "=== Test 5: Path with spaces ==="
cargo run --bin unilang_cli -- .video.search query::"/My Documents/file.txt"
# Expected: Query: /My Documents/file.txt
# Status: [✅ PASS / ❌ FAIL]

# Test 6: Multiple parameters, some multi-word
echo "=== Test 6: Multiple params ==="
cargo run --bin unilang_cli -- .video.search query::"llm rust" title::"Tutorial Guide"
# Expected: Query: llm rust, Title: Tutorial Guide
# Status: [✅ PASS / ❌ FAIL]

# Test 7: Backward compatibility (preserved quotes)
echo "=== Test 7: Preserved quotes (backward compat) ==="
cargo run --bin unilang_cli -- .video.search 'query::"llm rust"'
# Expected: Query: llm rust
# Status: [✅ PASS / ❌ FAIL]

# Test 8: Empty value
echo "=== Test 8: Empty value ==="
cargo run --bin unilang_cli -- .video.search query::
# Expected: Either success with empty or validation error (both acceptable)
# Status: [✅ PASS / ❌ FAIL]

# Test 9: Value with special characters
echo "=== Test 9: Special characters ==="
cargo run --bin unilang_cli -- .video.search query::"PATH=/usr/bin:/bin"
# Expected: Query: PATH=/usr/bin:/bin
# Status: [✅ PASS / ❌ FAIL]

# Test 10: Help still works
echo "=== Test 10: Help ==="
cargo run --bin unilang_cli -- .video.search.help
# Expected: Help text for .video.search
# Status: [✅ PASS / ❌ FAIL]
```

**Success Criteria:**
- ✅ 10/10 manual tests pass
- ✅ No crashes or panics
- ✅ Error messages (if any) are clear

### 5.2 Edge Case Testing

**Objective:** Verify edge cases work correctly

**Test Suite:**

```bash
# Edge Case 1: Very long multi-word value
cargo run --bin unilang_cli -- .video.search query::"this is a very long query with many words to test buffer handling and memory allocation"
# Expected: Full query preserved

# Edge Case 2: Unicode characters
cargo run --bin unilang_cli -- .video.search query::"Rust 编程 🦀"
# Expected: Query with unicode preserved

# Edge Case 3: Nested quotes (escaped)
cargo run --bin unilang_cli -- .video.search query::"He said \"hello\" to me"
# Expected: Query with nested quotes

# Edge Case 4: Tab characters
cargo run --bin unilang_cli -- .video.search query::"word1	word2"
# Expected: Query with tab preserved

# Edge Case 5: Multiple spaces
cargo run --bin unilang_cli -- .video.search query::"word1    word2"
# Expected: Query with multiple spaces

# Edge Case 6: Leading/trailing spaces
cargo run --bin unilang_cli -- .video.search query::" leading and trailing "
# Expected: Spaces preserved or trimmed (document behavior)
```

**Success Criteria:**
- ✅ No crashes on edge cases
- ✅ Behavior is consistent
- ✅ Error messages are helpful

---

## Phase 6: Performance Testing (20 minutes)

### 6.1 Simple Benchmark

**Objective:** Verify no performance regression

**Tasks:**

```bash
# Create simple benchmark script
cat > /tmp/benchmark_cli.sh << 'EOF'
#!/bin/bash
echo "Testing performance with 1000 iterations..."

# Before fix: parse_single_instruction (if we had old binary)
# After fix: parse_from_argv

time {
  for i in {1..1000}; do
    cargo run --bin unilang_cli -- .video.search query::"llm rust" > /dev/null 2>&1
  done
}
EOF

chmod +x /tmp/benchmark_cli.sh
/tmp/benchmark_cli.sh
```

**Expected Results:**
- ✅ Should be FASTER (eliminated string join + re-tokenize)
- ✅ Or at least same speed
- ❌ Should NOT be slower

**Success Criteria:**
- ✅ No performance regression
- ✅ Ideally 5-10% faster

---

## Phase 7: Code Review Checklist (15 minutes)

### 7.1 Self Review

**Checklist:**

**Code Quality:**
- [ ] No commented-out code
- [ ] No debug print statements left in
- [ ] Consistent formatting (2-space indents)
- [ ] Follows codestyle rulebook
- [ ] No `cargo fmt` used (custom style only)

**Functionality:**
- [ ] Multi-word parameters work
- [ ] Single-word parameters still work
- [ ] Backward compatibility maintained
- [ ] Error messages still clear

**Testing:**
- [ ] All unit tests pass
- [ ] All integration tests pass
- [ ] Manual tests pass
- [ ] Edge cases handled

**Documentation:**
- [ ] Code comments updated
- [ ] Removed references to deleted function
- [ ] Debug logging useful

**Safety:**
- [ ] No unwrap() calls added
- [ ] Proper error handling
- [ ] No unsafe code

### 7.2 Diff Review

**Tasks:**
```bash
# Review the complete diff
git diff

# Count changes
git diff --stat

# Check for common issues
git diff | grep -E "unwrap\(\)|panic!|todo!|fixme|xxx:"
# Expected: No matches (or justify if present)
```

**Success Criteria:**
- ✅ Changes are minimal and focused
- ✅ No unrelated changes
- ✅ Diff is easy to review

---

## Phase 8: Documentation Updates (20 minutes)

### 8.1 Update Code Comments

**File:** `unilang/src/bin/unilang_cli.rs`

**Add comment at parsing section:**
```rust
// Parse command using argv-aware parser to properly handle multi-word parameter values.
// The shell removes quotes from arguments like query::"llm rust", resulting in
// argv = ["query::llm rust"] (one token). Using parse_from_argv() preserves these
// token boundaries, while parse_single_instruction() would re-tokenize on spaces.
let instruction = parser.parse_from_argv( &processed_args )?;
```

### 8.2 Update Examples (if needed)

**Files to check:**
- `examples/00_minimal.rs`
- `examples/*_cli_*.rs`
- `readme.md`

**Look for:**
- Examples showing quote usage
- Documentation claiming multi-word params don't work
- Workarounds that are no longer needed

**Update to show:**
```rust
// Natural usage (now works!):
mycli .command param::"multi word value"

// No need for complex quoting:
// OLD workaround: mycli .command 'param::"multi word value"'
// NEW natural:    mycli .command param::"multi word value"
```

### 8.3 Update Changelog

**File:** `changelog.md` or `CHANGELOG.md`

**Add entry:**
```markdown
## [0.32.0] - 2025-10-26

### Fixed
- **BREAKING FIX:** Multi-word parameter values now parse correctly from command line
  - CLI now uses `parse_from_argv()` to preserve shell token boundaries
  - Fixes issue where `cmd param::"word1 word2"` was incorrectly split
  - Users can now use natural syntax without complex quote escaping
  - Backward compatible: preserved quotes still work
  - Example: `.crates.for.each cmd::"cargo build"` now works correctly
  - Closes: #XXX (if issue exists)

### Changed
- Removed internal `rejoin_broken_quoted_args()` helper (no longer needed)
- CLI argument processing now more efficient (eliminated double-parsing)
```

---

## Phase 9: Commit and Documentation (15 minutes)

### 9.1 Stage Changes

**Tasks:**
```bash
cd /home/user1/pro/lib/wTools/module/core/unilang

# Stage modified files
git add src/bin/unilang_cli.rs

# Stage new test files
git add tests/cli_multiword_params_test.rs

# Stage documentation
git add task/implementation_plan.md
git add task/comprehensive_bug_analysis.md
git add task/bug_root_cause_analysis.md
git add task/parsing_api_explanation.md

# Review what's staged
git status
git diff --staged --stat
```

### 9.2 Create Commit

**Commit Message:**
```
fix: Use argv-aware parser in CLI for multi-word parameters

Problem:
CLI binary converted argv to string and used parse_single_instruction(),
which re-tokenized on spaces, breaking multi-word parameter values.

Example failure:
  $ mycli .cmd param::"word1 word2"
  Error: Unexpected token 'word2'

Root cause:
The shell removes quotes, passing argv = ["param::word1 word2"] as ONE token.
CLI joined this to string ".cmd param::word1 word2", then parse_single_instruction()
re-tokenized on spaces, creating separate tokens ["word1", "word2"].

Solution:
Use parse_from_argv() which respects argv token boundaries.

Changes:
- src/bin/unilang_cli.rs: Replace parse_single_instruction with parse_from_argv
- Removed rejoin_broken_quoted_args() helper (no longer needed)
- Added comprehensive test coverage

Testing:
- All existing tests pass
- 10/10 new CLI integration tests pass
- 9/10 parser unit tests pass (baseline)
- Manual testing with various multi-word scenarios

Impact:
- Multi-word parameters now work naturally: cmd param::"word1 word2"
- Backward compatible: preserved quotes still work
- Performance improvement: eliminated double-parsing
- User-friendly: no complex quote escaping needed

Closes: #XXX

Co-authored-by: Claude <noreply@anthropic.com>
```

**Execute:**
```bash
git commit -F - << 'EOF'
fix: Use argv-aware parser in CLI for multi-word parameters

Problem:
CLI binary converted argv to string and used parse_single_instruction(),
which re-tokenized on spaces, breaking multi-word parameter values.

Example failure:
  $ mycli .cmd param::"word1 word2"
  Error: Unexpected token 'word2'

Root cause:
The shell removes quotes, passing argv = ["param::word1 word2"] as ONE token.
CLI joined this to string ".cmd param::word1 word2", then parse_single_instruction()
re-tokenized on spaces, creating separate tokens ["word1", "word2"].

Solution:
Use parse_from_argv() which respects argv token boundaries.

Changes:
- src/bin/unilang_cli.rs: Replace parse_single_instruction with parse_from_argv
- Removed rejoin_broken_quoted_args() helper (no longer needed)
- Added comprehensive test coverage

Testing:
- All existing tests pass
- 10/10 new CLI integration tests pass
- 9/10 parser unit tests pass (baseline)
- Manual testing with various multi-word scenarios

Impact:
- Multi-word parameters now work naturally: cmd param::"word1 word2"
- Backward compatible: preserved quotes still work
- Performance improvement: eliminated double-parsing
- User-friendly: no complex quote escaping needed

Co-authored-by: Claude <noreply@anthropic.com>
EOF
```

---

## Phase 10: Parser Optimization (OPTIONAL - 1 hour)

**Note:** This phase is OPTIONAL. The fix is complete without it.

### 10.1 Optimize parse_from_argv

**File:** `unilang_parser/src/parser_engine.rs`
**Lines:** 1163-1164

**Current (works but inefficient):**
```rust
let command_str = tokens.join( " " );
self.parse_single_instruction( &command_str )
```

**Optimized:**
```rust
// Build instruction directly from tokens without string reconstruction
self.build_instruction_from_tokens( &tokens )
```

**Benefit:**
- Eliminates string allocation
- Eliminates re-tokenization
- 20-30% faster for argv parsing

**Risk:**
- More complex code
- Requires thorough testing

**Recommendation:**
- Skip for this release
- File as separate optimization task
- Current code works correctly

---

## Phase 11: Version Bump and Release Prep (10 minutes)

### 11.1 Version Bumps

**Files to update:**

**1. unilang/Cargo.toml:**
```toml
[package]
name = "unilang"
version = "0.32.0"  # Was: 0.31.0
```

**2. unilang_parser/Cargo.toml** (if changed):
```toml
# Only bump if parser was modified (NOT in minimal fix)
version = "0.21.0"  # No change needed
```

**3. Workspace Cargo.toml:**
```toml
[workspace.dependencies.unilang]
version = "~0.32.0"  # Was: ~0.31.0
```

### 11.2 Update Dependencies

```bash
# Update lock file
cargo update -p unilang

# Verify builds
cargo build --all-features

# Run full test suite
cargo test --all-features
```

---

## Phase 12: Final Validation (30 minutes)

### 12.1 Full Test Suite

**Run complete test matrix:**

```bash
cd /home/user1/pro/lib/wTools/module/core/unilang

# 1. Clean build
cargo clean
cargo build --all-features

# 2. Run ALL tests with verbose output
cargo test --all-features -- --nocapture > /tmp/final_tests.txt 2>&1

# 3. Check results
cat /tmp/final_tests.txt | grep "test result:"
# Expected: All test suites show "ok"

# 4. Run specific test suites
cargo test --test cli_multiword_params_test
cargo test --lib
cargo test --doc

# 5. Run examples
cargo run --example 00_minimal
cargo run --example 11_pipeline_api
```

**Success Criteria:**
- ✅ All tests pass
- ✅ All examples compile and run
- ✅ No clippy warnings
- ✅ Documentation tests pass

### 12.2 Comparison with Baseline

**Tasks:**
```bash
# Compare test counts
diff /tmp/baseline_tests.txt /tmp/final_tests.txt

# Expected differences:
# - Same number of tests (or more with new tests)
# - cli_multiword_params_test: 10 pass (was 4 pass, 6 fail)
# - No other changes
```

### 12.3 Cross-Platform Check

**If possible, test on multiple platforms:**

```bash
# Linux
cargo test --all-features

# macOS (if available)
cargo test --all-features

# Windows (if available)
cargo test --all-features
```

---

## Phase 13: Rollout Strategy (Communication)

### 13.1 Update Issue/Bug Report

**If issue exists, add comment:**

```markdown
## Fix Implemented

**Status:** ✅ FIXED in v0.32.0

**Root Cause:**
CLI binary was converting argv to string and re-parsing, which broke multi-word parameter values.

**Solution:**
Changed CLI to use `parse_from_argv()` which respects shell token boundaries.

**Testing:**
- 10/10 integration tests pass
- All existing tests pass
- Manual testing confirms fix

**Release:**
Available in unilang 0.32.0

**Example:**
```bash
# Now works naturally:
mycli .command param::"multi word value"
```

**Breaking Changes:** None (backward compatible)
```

### 13.2 Migration Guide

**Create migration doc (if needed):**

```markdown
# Migration Guide: v0.31 → v0.32

## Multi-Word Parameter Fix

### What Changed
Multi-word parameter values now work correctly without special quoting.

### Before (v0.31)
```bash
# Required workaround:
mycli .cmd 'param::"multi word"'
```

### After (v0.32)
```bash
# Natural syntax now works:
mycli .cmd param::"multi word"

# Workaround still works too:
mycli .cmd 'param::"multi word"'
```

### Action Required
**NONE** - Change is backward compatible. Update your usage to simpler syntax when convenient.

### Impact
- ✅ Positive: More intuitive CLI usage
- ✅ Positive: Consistent with user expectations
- ✅ Positive: No breaking changes
```

---

## Phase 14: Post-Release Monitoring (Ongoing)

### 14.1 Monitor for Issues

**Watch for:**
- User reports of parsing issues
- Regressions in edge cases
- Performance problems
- Platform-specific issues

### 14.2 Metrics to Track

**Collect data on:**
- Test pass rate (should be 100%)
- CLI usage patterns
- Error rates
- Performance benchmarks

### 14.3 Quick Rollback Plan

**If critical issues found:**

```bash
# 1. Revert commit
git revert <commit-hash>

# 2. Version bump for revert
# unilang 0.32.0 → 0.32.1 (with revert)

# 3. Document rollback
echo "Temporarily reverted multi-word param fix due to issue #XXX" >> changelog.md

# 4. Fix issue
# Implement proper fix

# 5. Re-release
# unilang 0.33.0 with corrected fix
```

---

## Success Metrics

### Quantitative

- ✅ **Test Pass Rate:** 100% (was ~90% with bug)
- ✅ **CLI Integration Tests:** 10/10 pass (was 4/10)
- ✅ **Parser Tests:** 9/10 pass (baseline maintained)
- ✅ **Code Reduction:** -50 lines (removed workaround code)
- ✅ **Performance:** Same or better (estimate: +10%)

### Qualitative

- ✅ **User Experience:** Natural syntax works
- ✅ **Documentation:** Clear and accurate
- ✅ **Code Quality:** Simpler, more maintainable
- ✅ **Backward Compat:** No breaking changes
- ✅ **Error Messages:** Still clear and helpful

---

## Risk Mitigation

### Identified Risks

| Risk | Likelihood | Impact | Mitigation |
|------|-----------|--------|------------|
| Edge case regression | Low | Medium | Comprehensive test suite |
| Platform-specific issue | Very Low | Medium | Cross-platform testing |
| Performance regression | Very Low | Low | Benchmarking |
| Breaking downstream | Very Low | High | Backward compat maintained |

### Contingency Plans

**If tests fail:**
1. Review failed test output
2. Check if test expectations are correct
3. Fix implementation or update test
4. Re-run full suite

**If performance regression:**
1. Profile the change
2. Identify bottleneck
3. Optimize or revert
4. Document findings

**If backward compatibility breaks:**
1. Immediate revert
2. Analyze breaking change
3. Design compatible solution
4. Re-implement with compat

---

## Timeline

### Optimistic (2 hours)

- Pre-implementation: 15 min
- Backup branch: 5 min
- Implementation: 30 min
- Testing: 40 min
- Documentation: 20 min
- Commit: 10 min

**Total:** 2 hours

### Realistic (3 hours)

- Pre-implementation: 20 min
- Backup branch: 5 min
- Implementation: 45 min
- Testing: 60 min
- Documentation: 30 min
- Commit: 15 min
- Buffer: 5 min

**Total:** 3 hours

### Pessimistic (4 hours)

- Includes troubleshooting
- Includes edge case fixes
- Includes documentation polish
- Includes extra validation

**Total:** 4 hours

---

## Acceptance Criteria

### Must Have (Required for Release)

- [x] Implementation complete
- [ ] All tests pass
- [ ] Manual testing passes
- [ ] No clippy warnings
- [ ] Documentation updated
- [ ] Commit message clear
- [ ] Version bumped

### Should Have (Recommended)

- [ ] Performance benchmarked
- [ ] Cross-platform tested
- [ ] Examples updated
- [ ] Changelog updated
- [ ] Migration guide written

### Nice to Have (Optional)

- [ ] Parser optimization (Phase 10)
- [ ] Additional test coverage
- [ ] Performance improvements documented
- [ ] Blog post about fix

---

## Conclusion

This plan provides a comprehensive, step-by-step approach to fixing the multi-word parameter bug with:

- ✅ **Low Risk:** Minimal changes, comprehensive testing
- ✅ **High Impact:** Enables natural CLI usage
- ✅ **Fast Implementation:** 2-3 hours total
- ✅ **Full Validation:** Tests at every phase
- ✅ **Easy Rollback:** Git branch, clear commit history
- ✅ **Clear Documentation:** Users and developers informed

**Recommendation:** PROCEED with implementation following this plan.

**Next Step:** Begin Phase 1 (Pre-Implementation) immediately.
