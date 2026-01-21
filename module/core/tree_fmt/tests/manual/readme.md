# Manual Testing Plan for tree_fmt v0.5.0

This document describes manual testing procedures for verifying column truncation and multiline cell features in tree_fmt.

## Features Under Test

### 1. Column Truncation (`max_column_width`)
- ANSI-aware truncation (visual length calculation excluding color codes)
- Truncation marker appending
- Edge cases (marker longer than limit, exact fit, empty cells)
- Backward compatibility (disabled by default)

### 2. Multiline Cells (automatic `\n` detection)
- Automatic detection and rendering
- Row height calculation (max lines across cells)
- Column alignment preservation
- Padding shorter cells to match row height
- ANSI code support in multiline contexts

### 3. Combined Features (Multiline + Truncation)
- Per-line truncation after splitting
- ANSI code preservation through both transformations
- Visual correctness of combined output

## Test Cases

### TC-001: Basic Column Truncation

**Objective**: Verify basic truncation with default marker

**Steps**:
1. Create table with long cell content (>20 chars)
2. Set `max_column_width` to 20
3. Render and inspect output

**Expected**:
- Content truncated to 17 chars + "..." = 20 total
- No ANSI codes counted toward limit
- Output visually readable

**Status**: [ ] Pass [ ] Fail

---

### TC-002: ANSI Code Preservation in Truncation

**Objective**: Verify ANSI color codes preserved after truncation

**Steps**:
1. Create cell with ANSI colored text exceeding limit
2. Apply truncation with `max_column_width`
3. Verify output contains ANSI codes before truncated text

**Expected**:
- Color codes present in output
- Visual length (excluding codes) equals limit
- Color rendering works in terminal

**Status**: [ ] Pass [ ] Fail

---

### TC-003: Custom Truncation Marker

**Objective**: Verify custom markers work correctly

**Steps**:
1. Set `truncation_marker` to "…" (single unicode char)
2. Truncate long content
3. Verify marker appears

**Expected**:
- Custom marker used instead of "..."
- Visual length calculation accounts for unicode
- Marker visually distinct

**Status**: [ ] Pass [ ] Fail

---

### TC-004: Marker Longer Than Limit

**Objective**: Edge case - marker itself exceeds `max_column_width`

**Steps**:
1. Set `max_column_width` to 5
2. Set `truncation_marker` to "........." (9 chars)
3. Attempt truncation

**Expected**:
- Graceful handling via `saturating_sub`
- Returns marker only (or empty + marker)
- No panic or error

**Status**: [ ] Pass [ ] Fail

---

### TC-005: Exact Fit (No Truncation)

**Objective**: Content exactly at limit should NOT truncate

**Steps**:
1. Create cell with exactly 20 characters
2. Set `max_column_width` to 20
3. Render

**Expected**:
- Full content displayed
- No truncation marker added
- Visual length = 20

**Status**: [ ] Pass [ ] Fail

---

### TC-006: Basic Multiline Cell

**Objective**: Single cell with multiple lines renders correctly

**Steps**:
1. Create cell with "Line1\nLine2\nLine3"
2. Render with plain style
3. Inspect output structure

**Expected**:
- Three visual lines in table
- Column separators on each line
- Proper alignment maintained

**Status**: [ ] Pass [ ] Fail

---

### TC-007: Multiline Row with Mixed Heights

**Objective**: Row with cells of different line counts

**Steps**:
1. Create row: ["A", "B\nC\nD", "E"]
2. Render table
3. Check alignment

**Expected**:
- Row height = 3 (max lines in any cell)
- "A" and "E" padded to 3 lines
- Vertical alignment of column separators

**Status**: [ ] Pass [ ] Fail

---

### TC-008: Multiline with ANSI Colors

**Objective**: ANSI codes in multiline cells

**Steps**:
1. Create cell: "\x1b[31mRed\x1b[0m\n\x1b[32mGreen\x1b[0m"
2. Render
3. Verify colors display

**Expected**:
- Red text on line 1
- Green text on line 2
- Alignment unaffected by ANSI codes

**Status**: [ ] Pass [ ] Fail

---

### TC-009: Empty Lines in Multiline Cell

**Objective**: Cells with empty lines between content

**Steps**:
1. Create cell: "Line1\n\nLine3" (empty line 2)
2. Render
3. Check spacing

**Expected**:
- Three lines rendered
- Middle line empty but space preserved
- No collapsing of empty lines

**Status**: [ ] Pass [ ] Fail

---

### TC-010: Multiline + Truncation Combined

**Objective**: Both features active simultaneously

**Steps**:
1. Create multiline cell with long lines: "Very long first line\nShort"
2. Set `max_column_width` to 20
3. Render

**Expected**:
- First line truncated to 17 chars + "..."
- Second line displayed fully (not truncated)
- Each line independently evaluated

**Status**: [ ] Pass [ ] Fail

---

### TC-011: Multiline + Truncation + ANSI

**Objective**: All three features combined

**Steps**:
1. Create cell: "\x1b[31mVery long red text here\x1b[0m\n\x1b[32mShort green\x1b[0m"
2. Set `max_column_width` to 20
3. Render with terminal color support

**Expected**:
- Line 1: Red color preserved, truncated with marker
- Line 2: Green color preserved, no truncation
- Visual length correct on both lines

**Status**: [ ] Pass [ ] Fail

---

### TC-012: Multiline in CSV Format

**Objective**: Verify multiline disabled in CSV (newlines literal)

**Steps**:
1. Create cell with "A\nB"
2. Format with `CsvFormatter`
3. Check output

**Expected**:
- Output contains literal `\n` characters
- No actual line break in CSV
- CSV remains single-line per record

**Status**: [ ] Pass [ ] Fail

---

### TC-013: Truncation Backward Compatibility

**Objective**: Default behavior unchanged when feature not configured

**Steps**:
1. Create table with long content
2. Render WITHOUT setting `max_column_width`
3. Compare to v0.4.0 behavior

**Expected**:
- Full content displayed
- No truncation applied
- Identical to previous version output

**Status**: [ ] Pass [ ] Fail

---

### TC-014: Multiple Columns with Truncation

**Objective**: Truncation applied independently per column

**Steps**:
1. Create table with 3 columns
2. Set `max_column_width` to 15
3. Add rows with varying content lengths

**Expected**:
- Each column truncated independently
- Truncation applied to cells exceeding limit
- Cells under limit displayed fully

**Status**: [ ] Pass [ ] Fail

---

### TC-015: Headers with Truncation

**Objective**: Headers also respect `max_column_width`

**Steps**:
1. Create table with long header names
2. Set `max_column_width` to 20
3. Render

**Expected**:
- Long headers truncated
- Short headers displayed fully
- Consistent truncation rules for headers and data

**Status**: [ ] Pass [ ] Fail

---

## Visual Inspection Tests

These tests require human visual verification in a terminal:

### VI-001: Terminal Color Display
- Run test with ANSI colored output
- Verify colors render correctly
- Check truncation doesn't break color sequences

### VI-002: Unicode Truncation Marker
- Use unicode markers (…, →, ⋯)
- Verify correct display in terminal
- Check width calculation accurate

### VI-003: Table Border Alignment
- Render multiline cells with bordered style
- Verify borders align correctly across all lines
- Check corner characters position correctly

### VI-004: Markdown Table Format
- Render multiline cells with markdown style
- Verify markdown syntax valid
- Check alignment characters (`|`, `-`) position correctly

## Test Execution

### Prerequisites
- Terminal with ANSI color support (for color tests)
- Unicode-capable terminal (for unicode marker tests)
- tree_fmt v0.5.0 installed

### Running Tests

Execute manual test programs in this directory:

```bash
# Run all manual tests
cargo run --example manual_test_001_truncation
cargo run --example manual_test_002_multiline
cargo run --example manual_test_003_combined

# Or use test script
./run_manual_tests.sh
```

### Recording Results

Update each test case status in this document:
- [x] Pass - Feature works as expected
- [ ] Fail - Issue found (document in Issues section below)

## Issues Found

Document any issues discovered during manual testing:

### Issue Template
```
**ID**: ISSUE-XXX
**Test Case**: TC-XXX
**Severity**: Critical | High | Medium | Low
**Description**: [What went wrong]
**Steps to Reproduce**: [Exact steps]
**Expected**: [What should happen]
**Actual**: [What actually happened]
**Fix Applied**: [Description of fix] or [Not yet fixed]
```

---

## Completion Criteria

All tests marked Pass, zero issues with severity Critical or High remaining.

**Test Results Summary**:
- Total Tests: 15 functional + 4 visual = 19
- Passed: ___ / 19
- Failed: ___ / 19
- Issues Found: ___
- Issues Fixed: ___
- Issues Remaining: ___

**Sign-off**: Manual testing complete when all criteria met and documented.
