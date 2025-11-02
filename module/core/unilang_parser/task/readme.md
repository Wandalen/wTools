# Task Management System

## Tasks Index

| Order | ID | Advisability | Value | Easiness | Safety | Priority | Status | Task | Description |
|-------|----|--------------:|------:|---------:|-------:|---------:|--------|------|-------------|
| 1 | 084 | 0 | 9 | 7 | 7 | 0 | ✅ (Completed) | [tokenizer_escaped_quote_handling](./completed/084_tokenizer_escaped_quote_handling.md) | Fix tokenizer handling of backslash-escaped quotes in values with whitespace |
| 2 | 001 | 360 | 10 | 3 | 6 | 2 | 📥 (Backlog) | [zero_copy_tokens](./backlog/001_zero_copy_tokens.md) | Zero-copy token implementation for 8-15x performance improvement |

## Issues Index

| ID | Title | Related Task | Status |
|----|-------|--------------|--------|
| ISSUE-STRS-001 | `strs_tools` Unescaping Bug | - | ✅ (Resolved - Already Implemented) |
| ISSUE-CMD-PATH | [Command Path Parser Bug](./issue/closed/issue_command_path_parser_bug.md) | 084 | 🟢 (Closed - Fixed 2025-11-01) |

## Issues

### ISSUE-STRS-001: `strs_tools` Unescaping Bug

**Status:** ✅ (Resolved - Already Implemented)

**Resolution:** Deep investigation on 2025-11-01 revealed that strs_tools already fully implements escape sequence handling at split.rs:462-498. All MRE tests pass. The parsing failures were actually caused by a separate bug in unilang_parser's command path parser (see ISSUE-CMD-PATH).

**Evidence:**
- Escape handling verified at `/home/user1/pro/lib/wTools/module/core/strs_tools/src/string/split.rs:462-498`
- All tests pass: `/home/user1/pro/lib/wTools/module/core/strs_tools/tests/issue_001_mre.rs`
- strs_tools correctly produces: `["cmd", "::", "value with \"inner\" quotes"]`

**Real Bug:** See ISSUE-CMD-PATH below.

---

### ISSUE-CMD-PATH: Command Path Parser Bug

**Description:** The command path parser (`parse_command_path` at parser_engine.rs:385-404) incorrectly consumed identifiers without checking if they're part of named argument patterns (`name::value`). This caused the parser to misinterpret named arguments as command path segments, leading to "orphaned operator" errors.

**File:** [issue_command_path_parser_bug.md](./issue/closed/issue_command_path_parser_bug.md)
**Completion Report:** [issue_command_path_parser_bug_completion_report.md](./issue/closed/issue_command_path_parser_bug_completion_report.md)

**Location:** `/home/user1/pro/lib/wTools/module/core/unilang_parser/src/parser_engine.rs:385-440`

**Root Cause:** Parser consumed identifier token without lookahead to check for `::` operator, which would indicate a named argument pattern.

**MRE:**
```rust
parse_single_instruction("cmd::value")
// Previously FAILED with: "Named argument operator '::' cannot appear by itself"
// Now WORKS: Parse as named_arguments={"cmd": ["value"]}
```

**Fix Applied:** Added iterator lookahead logic to detect `identifier::` pattern before consuming token (parser_engine.rs:407-428).

**Diagnostic Tests:** `/home/user1/pro/lib/wTools/module/core/unilang_parser/tests/diagnostic_real_bug.rs` (12 comprehensive tests, all passing)

**Status:** 🟢 (Closed - Fixed 2025-11-01)

**Severity:** High (Resolved)

**Related Task:** 084 (Escaped Quotes Handling)

**Results:**
- ✅ All 172 unit tests passing
- ✅ Zero regressions
- ✅ Complete documentation (5-section test docs + 3-field source comments)
- ✅ Module-level pitfall documentation added
