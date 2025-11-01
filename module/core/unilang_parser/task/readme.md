# Task Management System

## Tasks Index

| Order | ID | Advisability | Value | Easiness | Safety | Priority | Status | Task | Description |
|-------|----|--------------:|------:|---------:|-------:|---------:|--------|------|-------------|
| 1 | 001 | 1620 | 10 | 3 | 6 | 9 | 📥 (Backlog) | [zero_copy_tokens](./001_zero_copy_tokens.md) | Zero-copy token implementation for 8-15x performance improvement |
| 2 | 084 | 0 | 9 | 7 | 7 | 0 | ✅ (Completed) | [tokenizer_escaped_quote_handling](./completed/084_tokenizer_escaped_quote_handling.md) | Fix tokenizer handling of backslash-escaped quotes in values with whitespace |

## Issues Index

| ID | Title | Related Task | Status |
|----|-------|--------------|--------|
| ISSUE-STRS-001 | `strs_tools` Unescaping Bug | - | 🔴 (Open) |

## Issues

### ISSUE-STRS-001: `strs_tools` Unescaping Bug

**Description:** The `strs_tools::string::split` function, when `quoting(true)` is enabled, does not correctly unescape quoted strings containing escaped quotes (`\"`) or escaped backslashes (`\\`). The `SplitFastIterator`'s logic for finding the end of a quoted segment is flawed, leading to incorrect input for the `unescape_str` function.

**Location:** `module/core/strs_tools/src/string/split.rs`

**Issue Rationale:** This bug prevents `unilang_instruction_parser` from correctly parsing command arguments that contain escaped characters within quoted strings, leading to functional errors. A fix is required in `strs_tools` to unblock `unilang_instruction_parser` development.

**Related Proposal:** `module/core/strs_tools/task.md`

**Status:** 🔴 (Open)

**Severity:** High
