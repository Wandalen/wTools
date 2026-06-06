# BUG-005: Fenced code block heading terminates section extraction prematurely

- **Severity:** Major
- **State:** Fixed
- **Affects:** `include_md_section!` whenever the target section contains a fenced code block with a heading-like line at the same or higher level as the target heading
- **Component:** `src/lib.rs::extract_section`
- **Filed:** 2026-06-06
- **Updated:** 2026-06-06
- **Validated By:** `code_block_heading_not_a_boundary` and `tilde_fence_heading_not_a_boundary` in `tests/section_extraction.rs`
- **Validation Date:** 2026-06-06

## Symptom

```
# Given: tests/fixture/edge_cases.md contains:
#
#   ## H2 With Code Block
#
#   Content before code block.
#
#   ```rust
#   ## this looks like H2 but is code
#   ```
#
#   Content after code block.
#
#   ## Next H2
#
# Before fix — include_md_section!("tests/fixture/edge_cases.md", "## H2 With Code Block")
# Got:  "## H2 With Code Block\n\nContent before code block.\n\n```rust\n"
# Want: "## H2 With Code Block\n\n...(full section including code block and content after)..."
```

## Impact

Any section containing a fenced code block where the code includes a heading-like line
(e.g., `## heading`) at the same or higher depth as the target section heading is silently
truncated. The extracted string ends inside the code fence. No compile error is raised —
the truncation is a silent wrong-value bug.

Entity Scope: None.

## How Discovered

Identified during `/test_manual` session, exhaustive corner case analysis of `extract_section`.
Systematic review of the line scanner revealed no `in_code_block` state variable, confirming
the scanner would misinterpret heading-like lines inside fences.

## Minimum Reproducible Example

```rust
// Reproducer: code block containing same-level heading must not truncate extraction
let section = include_md_section!( "tests/fixture/edge_cases.md", "## H2 With Code Block" );
// Before fix: section ends at the backtick-fence opener, "Content after code block." missing
// After fix:  section includes full code block content and "Content after code block."
assert!( section.contains( "Content after code block." ) );
```

## Hypothesis Table

| ID | Hypothesis | State | Summary | Evidence |
|----|-----------|-------|---------|----------|
| H1 | `extract_section` has no `in_code_block` state, so heading-level check fires on all lines | ✅ Root Cause | Line scanner applies `heading_level()` unconditionally — no fence-tracking variable exists | E1 |
| H2 | `heading_level()` returns None for code fence openers | ✅ Confirmed | `` ` ```rust ` `` does not start with `#` → None; fence opener is NOT the trigger — the heading INSIDE the fence is | E2 |
| H3 | Section boundary fires only for higher-level headings inside fences | ❌ Refuted | Fires for equal-level headings too; any `level <= target_level` inside a fence triggers break | E1 |

## Evidence Table

| # | Location | What it shows | Hypothesis |
|---|----------|---------------|------------|
| E1 | `src/lib.rs::extract_section` (pre-fix, lines 62-73) | No `in_code_block` variable; `heading_level(line)` called on every line unconditionally | H1 ✅ Root Cause |
| E2 | `src/lib.rs::heading_level` | Returns `None` for lines not starting with `#`; fence openers do not start with `#` | H2 ✅ |

## Root Cause

```
extract_section(content, "## H2 With Code Block")
  target_level = 2
  for each line:
    if in_section:
      heading_level(line)          ← called on ALL lines, including inside fences
      if level <= 2: break         ← "## this looks like H2 but is code" → level=2, 2<=2 → BREAK!
```

`extract_section` tracked no `in_code_block` state. The heading detection check fired
unconditionally on every line, including lines inside fenced code blocks. A line that textually
looks like a heading inside a code fence triggers the section boundary logic identically to a real
heading, causing premature truncation.

## Why Not Caught

No fixture file contained an H2 (or same-level) heading inside a fenced code block. All existing
fixtures used plain text sections with no code fences. The existing tests did not exercise this
code path.

## Fix Location

`src/lib.rs::extract_section`:

Added `in_code_block: bool` local variable. On each line inside the section, if the line starts
with ` ``` ` or `~~~` (3+ backticks or tildes), the flag toggles. The `heading_level` boundary
check is skipped entirely while `in_code_block` is true. The fence-opener line itself is always
pushed to the result (before the toggle could affect the boundary check — the toggle happens first,
but boundary check is `if !in_code_block`).

## Prevention

Any section extraction algorithm must track fenced code block state when scanning for headings.
Heading detection must be gated by code block state.

**Pitfall:** Fenced code blocks can contain any markdown-like text including heading-prefixed
lines. Failing to track fence state causes silent wrong-value bugs — the section appears to compile
and expand, but produces a truncated string.

## Generalized Version

**Broken assumption:** A line starting with `#` followed by a space is always a markdown section
heading and always marks a section boundary.

Fails for any invocation of `include_md_section!` where:
1. The target section contains a fenced code block, AND
2. The code block contains a line starting with `## ` at depth ≤ the target section depth

**Detection invariant:**
```
for all sections S with target_level L containing fenced code blocks with heading lines at depth ≤ L:
  extract_section(content, S) contains all content between S and the NEXT REAL heading at depth ≤ L
```

## History

| Date | Event | Notes |
|------|-------|-------|
| 2026-06-06 | filed | Discovered during /test_manual exhaustive corner case analysis |
| 2026-06-06 | fix_applied | Added `in_code_block` toggle in `src/lib.rs::extract_section` |
| 2026-06-06 | verified | `code_block_heading_not_a_boundary` and `tilde_fence_heading_not_a_boundary` pass |
| 2026-06-06 | closed | |

## Refs: src/

- `src/lib.rs` — `extract_section`: `// Fix(BUG-005)` comment; `in_code_block` toggle added

## Refs: tests/

- `tests/section_extraction.rs` — `code_block_heading_not_a_boundary`: primary MRE; `bug_reproducer(BUG-005)`
- `tests/section_extraction.rs` — `tilde_fence_heading_not_a_boundary`: tilde fence variant; `bug_reproducer(BUG-005)`
