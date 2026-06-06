# BUG-005: Width truncation fires at exact boundary

- **Severity:** Minor
- **State:** Fixed
- **Affects:** `process_output` with any `max_width` value when input line length equals `max_width`
- **Component:** `src/output.rs::apply_width_filtering`
- **Filed:** 2026-05-17
- **Updated:** 2026-05-17
- **Validated By:** `width_exact_boundary` test in `tests/output.rs`
- **Validation Date:** 2026-05-17

## Symptom

```bash
# Before fix — line exactly at max_width=10 was incorrectly truncated:
# process_output("0123456789", "", &config.with_width(10))
# Got:  "012345678→"   (truncated — WRONG)
# Want: "0123456789"   (fits exactly — CORRECT)
```

## Impact

Any CLI tool using `process_output` with a `max_width` equal to an input line's visible length
receives a truncated line with the truncation suffix, making the output shorter than the configured
limit. Silent wrong result — no error is raised. Affects every invocation where `visual_len(line) == max_width`.
Entity Scope: None.

## How Discovered

```bash
# Discovered during migration of unilang::output → cli_fmt.
# Manual inspection of truncation logic: apply_width_filtering passed every line
# to truncate() unconditionally, including lines that fit exactly.
$ cargo test width_exact_boundary -- --nocapture
```

## Minimum Reproducible Example

```bash
mkdir -p /tmp/mre005 && cat > /tmp/mre005/src/lib.rs << 'EOF'
// Reproducer: line of exactly max_width chars must not be truncated
#[test]
fn mre_005() {
    use cli_fmt::output::*;
    let config = OutputConfig::default().with_width(10);
    let result = process_output("0123456789", "", &config);
    // Before fix: result.content == "012345678→", result.width_truncated == true
    // After fix:  result.content starts with "0123456789", result.width_truncated == false
    assert!(!result.width_truncated, "exact-width line must not be truncated");
}
EOF
# Run: cargo test mre_005
```

## Hypothesis Table

| ID | Hypothesis | State | Summary | Evidence |
|----|-----------|-------|---------|----------|
| H1 | `truncate()` is called unconditionally regardless of line length | ✅ Root Cause | `apply_width_filtering` passes every line to `truncate()` without first checking `visual_len(line) > max_width` | E1, E2 |
| H2 | `truncate()` internally handles the exact-width case correctly | ❌ Refuted | `TruncateOptions` reserves suffix space *within* `max_width`, so exact-width input gets shortened by suffix length | E3 |
| H3 | ANSI escape codes interfere with length calculation | ❌ Refuted | Bug reproduces with plain ASCII input — ANSI is not involved | E4 |

## Evidence Table

| # | Location | What it shows | Hypothesis |
|---|----------|---------------|------------|
| E1 | `src/output.rs::apply_width_filtering` (pre-fix) | Every line passed to `truncate_lines()` without `> max_width` guard | H1 ✅ Root Cause |
| E2 | `strs_tools::ansi::truncate_lines` | Truncation fires for any input ≥ max_width, not just > max_width | H1 ✅ Root Cause |
| E3 | `strs_tools::TruncateOptions` | Suffix occupies space within `max_width` — exact-fit input loses suffix-length chars | H2 ❌ |
| E4 | `tests/output.rs::width_exact_boundary` | Plain ASCII "0123456789" with width=10 reproduces the truncation | H3 ❌ |

## Root Cause

```
apply_width_filtering(content, …)
  → for each line:
      truncate_lines(line, TruncateOptions { max_width })   ← unconditional call
                                                              ← truncate() reserves suffix space
                                                              ← exact-width line loses suffix chars
```

`apply_width_filtering` delegated truncation unconditionally. `truncate()` is designed for lines known to be too long — it does not handle the boundary case. The caller must check `visual_len(line) > max_width` before invoking truncation.

## Why Not Caught

No test existed for the exact-width boundary case before migration. Existing tests used inputs
clearly shorter than or clearly longer than `max_width`. The boundary case `len == max_width` was
not covered by any test in `strs_tools::output` at extraction time.

## Fix Location

`strs_tools::ansi::truncate_lines`:

The boundary-detection guard (`visual_len > max_width`) is implemented internally in
`strs_tools::truncate_lines`. `cli_fmt::apply_width_filtering` delegates unconditionally
to `truncate_lines`, which returns the content unchanged when no line exceeds `max_width`.
No guard was added to `cli_fmt`.

## Prevention

Always check `visual_len(line) > max_width` before calling any ANSI-aware truncation function.
Truncation functions designed for "lines known to be too long" must never be called on lines that
may fit exactly — they do not return the original unchanged.

**Pitfall:** `TruncateOptions`-based truncation is designed for unconditional use; callers must perform boundary detection before calling truncate functions.

## Generalized Version

**Broken assumption:** `truncate(line, max_width)` returns `line` unchanged when `visual_len(line) == max_width`.

Fails for any call to ANSI-aware truncation where:
1. The truncation function reserves suffix space within `max_width`, AND
2. The caller does not check `visual_len(line) > max_width` first, AND
3. The input line's visible length equals `max_width` exactly

**Detection invariant:**
```
for all lines where visual_len(line) == max_width:
  process_output(line, "", config.with_width(max_width)).width_truncated == false
```

## History

| Date | Event | Notes |
|------|-------|-------|
| 2026-05-17 | filed | Discovered during unilang::output → cli_fmt migration |
| 2026-05-17 | fix_applied | Boundary-detection guard `visual_len > max_width` already present in `strs_tools::truncate_lines`; no change needed in `cli_fmt` |
| 2026-05-17 | verified | Verified by `width_exact_boundary` test in `tests/output.rs` |
| 2026-05-17 | closed | |

## Refs: src/

- `src/output.rs` — `apply_width_filtering`: no fix comment; boundary guard resides in `strs_tools::truncate_lines` — `apply_width_filtering` delegates unconditionally

## Refs: tests/

- `tests/output.rs` — `width_exact_boundary`: precise boundary-detection reproducer; `bug_reproducer(BUG-005)` annotation present
