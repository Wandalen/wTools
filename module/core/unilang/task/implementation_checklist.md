# Implementation Checklist: Multi-Word Parameter Fix

Print this checklist and check off items as you complete them.

---

## Pre-Implementation ⏱️ 15 min

### Baseline Testing
- [ ] Run `cargo test --all-features > /tmp/baseline_tests.txt 2>&1`
- [ ] Run `cargo test --test cli_multiword_params_test > /tmp/baseline_cli.txt 2>&1`
- [ ] Record results: CLI tests = ___/10 pass (expect: 4/10)
- [ ] Manual test: `cargo run --bin unilang_cli -- .video.search query::"llm rust"`
- [ ] Result: ❌ FAILS (confirm bug exists)

### Git Setup
- [ ] `git status` (verify clean)
- [ ] `git checkout -b fix/multiword-parameter-parsing`
- [ ] `git status` (verify on new branch)

---

## Implementation ⏱️ 30 min

### Edit unilang_cli.rs
- [ ] Open `/home/user1/pro/lib/wTools/module/core/unilang/src/bin/unilang_cli.rs`
- [ ] **DELETE** lines 26-73 (`rejoin_broken_quoted_args` function + doc)
- [ ] Navigate to parsing section (~line 550 after deletion)
- [ ] **REPLACE** this code:
  ```rust
  let command_input_str = rejoin_broken_quoted_args( &processed_args );
  if verbosity > 1 {
    eprintln!( "DEBUG: Processed arguments: {processed_args:?}" );
    eprintln!( "DEBUG: Rejoined command string: {command_input_str:?}" );
  }
  let instruction = parser.parse_single_instruction( &command_input_str )?;
  ```
- [ ] **WITH** this code:
  ```rust
  if verbosity > 1 {
    eprintln!( "DEBUG: Processing argv: {processed_args:?}" );
  }
  let instruction = parser.parse_from_argv( &processed_args )?;
  ```
- [ ] Save file
- [ ] Run `cargo check --bin unilang_cli`
- [ ] Verify: ✅ No errors

### Verify Changes
- [ ] Run `git diff src/bin/unilang_cli.rs | wc -l`
- [ ] Expect: ~55-60 lines changed
- [ ] Run `git diff src/bin/unilang_cli.rs | grep "parse_from_argv"`
- [ ] Expect: See new call to parse_from_argv
- [ ] Run `grep -n "rejoin_broken_quoted_args" src/bin/unilang_cli.rs`
- [ ] Expect: No matches (function deleted)

---

## Testing ⏱️ 45 min

### Unit Tests
- [ ] `cd /home/user1/pro/lib/wTools/module/core/unilang_parser`
- [ ] `cargo test --all-features`
- [ ] Result: All pass ✅ (expect: same as baseline)
- [ ] `cargo test --test argv_multiword_bug_test`
- [ ] Result: 9/10 pass ✅ (expect: same as baseline)

### CLI Integration Tests
- [ ] `cd /home/user1/pro/lib/wTools/module/core/unilang`
- [ ] `cargo test --test cli_multiword_params_test`
- [ ] Result: ___/10 pass (expect: 10/10) ✅
- [ ] If < 10/10, STOP and investigate failures

### Full Test Suite
- [ ] `cargo test --all-features`
- [ ] Result: All pass ✅
- [ ] Count total tests: _____ (expect: same or more than baseline)
- [ ] If any failures, STOP and investigate

---

## Manual Validation ⏱️ 20 min

### Critical Tests
- [ ] `cargo run --bin unilang_cli -- .video.search query::"llm rust"`
  - Expected: `Query: llm rust` ✅
  - Actual: _________________

- [ ] `cargo run --bin unilang_cli -- .video.search query::"cargo build --release"`
  - Expected: `Query: cargo build --release` ✅
  - Actual: _________________

- [ ] `cargo run --bin unilang_cli -- .video.search query::"/My Documents/file.txt"`
  - Expected: `Query: /My Documents/file.txt` ✅
  - Actual: _________________

### Regression Tests
- [ ] `cargo run --bin unilang_cli -- .video.search query::rust`
  - Expected: `Query: rust` ✅
  - Actual: _________________

- [ ] `cargo run --bin unilang_cli -- .video.search 'query::"llm rust"'`
  - Expected: `Query: llm rust` ✅ (backward compat)
  - Actual: _________________

### Multi-Parameter Test
- [ ] `cargo run --bin unilang_cli -- .video.search query::"llm rust" title::"Tutorial"`
  - Expected: Both params correct ✅
  - Actual: _________________

---

## Code Review ⏱️ 15 min

### Self Review
- [ ] `git diff` (review all changes)
- [ ] No commented-out code
- [ ] No debug prints left in
- [ ] 2-space indentation maintained
- [ ] Code follows rulebook style
- [ ] Changes are minimal and focused

### Quality Checks
- [ ] `git diff | grep -E "unwrap\(\)|panic!|todo!|xxx:"` → No matches
- [ ] `cargo clippy --all-targets --all-features` → No warnings
- [ ] No unsafe code added
- [ ] Error handling proper

---

## Documentation ⏱️ 15 min

### Code Comments
- [ ] Added comment explaining why `parse_from_argv` is used
- [ ] Removed references to deleted function
- [ ] Debug logging is clear

### Version Updates
- [ ] Update `unilang/Cargo.toml` version: `0.31.0` → `0.32.0`
- [ ] Update workspace `Cargo.toml` if needed
- [ ] Run `cargo update -p unilang`

### Changelog (if exists)
- [ ] Add entry for bug fix
- [ ] Document impact and migration
- [ ] Reference issue number (if applicable)

---

## Commit ⏱️ 10 min

### Stage Files
- [ ] `git add src/bin/unilang_cli.rs`
- [ ] `git add tests/cli_multiword_params_test.rs`
- [ ] `git add Cargo.toml` (if version bumped)
- [ ] `git add task/*.md` (documentation)
- [ ] `git status` (review staged files)

### Commit Message
- [ ] Use template from `implementation_plan.md` Phase 9.2
- [ ] Message includes: problem, root cause, solution, testing, impact
- [ ] Message includes Co-authored-by line
- [ ] No AI attribution (per rulebook)

### Create Commit
- [ ] `git commit -m "..."`  OR use heredoc from plan
- [ ] `git log -1` (review commit)
- [ ] `git show --stat` (review changes)

---

## Final Validation ⏱️ 15 min

### Clean Build Test
- [ ] `cargo clean`
- [ ] `cargo build --all-features`
- [ ] Result: Success ✅

### Full Test Suite
- [ ] `cargo test --all-features > /tmp/final_tests.txt 2>&1`
- [ ] `cat /tmp/final_tests.txt | grep "test result:"`
- [ ] All test suites: `ok` ✅

### Compare with Baseline
- [ ] `diff /tmp/baseline_tests.txt /tmp/final_tests.txt`
- [ ] CLI tests improved: 4/10 → 10/10 ✅
- [ ] No regressions in other tests ✅

### Examples Check
- [ ] `cargo run --example 00_minimal`
- [ ] `cargo run --example 11_pipeline_api`
- [ ] Both run successfully ✅

---

## Success Criteria

All items must be checked before considering implementation complete:

**Testing:**
- [x] All unit tests pass
- [x] CLI integration tests: 10/10 pass
- [x] Manual tests all work
- [x] No regressions

**Code Quality:**
- [x] Minimal changes (~50 lines deleted)
- [x] No clippy warnings
- [x] Follows rulebook style
- [x] Clear commit message

**Functionality:**
- [x] Multi-word params work: `param::"word1 word2"`
- [x] Single-word params still work: `param::word`
- [x] Backward compat maintained: `'param::"word1 word2"'`
- [x] Error messages clear

**Documentation:**
- [x] Code comments updated
- [x] Version bumped
- [x] Changelog updated (if applicable)
- [x] Tests documented

---

## If Something Fails

### Test Failures
1. [ ] Read error message carefully
2. [ ] Check if test expectation is correct
3. [ ] Run single test with `--nocapture` for details
4. [ ] Fix implementation or test
5. [ ] Re-run full suite

### Build Errors
1. [ ] Read compiler error
2. [ ] Check syntax at modified lines
3. [ ] Verify correct function name used
4. [ ] Run `cargo check` for detailed errors

### Manual Test Failures
1. [ ] Run with `UNILANG_VERBOSITY=2` for debug output
2. [ ] Check argv received by CLI
3. [ ] Verify parser is called correctly
4. [ ] Test with simpler input first

### Need to Rollback
1. [ ] `git diff` (see what you changed)
2. [ ] `git checkout src/bin/unilang_cli.rs` (discard changes)
3. [ ] `git status` (verify clean)
4. [ ] Review plan and try again

---

## Time Tracking

**Planned:** 2-3 hours
**Actual:**

- Pre-implementation: _____ min (planned: 15)
- Implementation: _____ min (planned: 30)
- Testing: _____ min (planned: 45)
- Manual validation: _____ min (planned: 20)
- Code review: _____ min (planned: 15)
- Documentation: _____ min (planned: 15)
- Commit: _____ min (planned: 10)
- Final validation: _____ min (planned: 15)

**Total: _____ min (planned: 165 min / 2.75 hours)**

---

## Notes / Issues Encountered

_(Use this space to document any issues, surprises, or learnings)_

```




```

---

## Completion Sign-Off

- [ ] All checklist items completed
- [ ] All tests passing
- [ ] Changes committed
- [ ] Ready for review/merge

**Completed by:** _________________
**Date:** _________________
**Time taken:** _________________

---

**Next Step:** Merge to master or create PR
