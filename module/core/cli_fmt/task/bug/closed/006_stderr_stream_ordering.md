# BUG-006: stderr appears after stdout in merged output

- **Severity:** Medium
- **State:** Fixed
- **Affects:** `merge_streams` with `StreamFilter::Both` — all invocations of `process_output` with both streams non-empty
- **Component:** `src/output.rs::merge_streams`
- **Filed:** 2025-11-29
- **Updated:** 2025-11-29
- **Validated By:** `merge_streams_ordering` test in `tests/output.rs`
- **Validation Date:** 2025-11-29

## Symptom

```bash
# Before fix — stdout appeared before stderr:
# merge_streams("stdout", "stderr", &StreamFilter::Both)
# Got:  "stdout\nstderr"   (WRONG — stdout first)
# Want: "stderr\nstdout"   (CORRECT — errors first)
```

## Impact

Any CLI tool using `process_output` or `merge_streams` with both streams non-empty received
stderr after stdout. Error messages appeared below normal output, making them easy to miss when
output is long. Silent wrong result — no error raised. Affects every invocation where both
`stdout` and `stderr` are non-empty strings.
Entity Scope: None.

## How Discovered

```bash
# Discovered during extraction of output module from strs_tools.
# Code review of merge_streams identified alphabetical variable ordering
# (stdout before stderr) that violated CLI convention.
$ cargo test merge_streams_ordering -- --nocapture
```

## Minimum Reproducible Example

```bash
mkdir -p /tmp/mre006 && cat > /tmp/mre006/test.rs << 'EOF'
// Reproducer: stderr must precede stdout in merged output
#[test]
fn mre_006() {
    use cli_fmt::output::{ merge_streams, StreamFilter };
    let result = merge_streams("stdout", "stderr", &StreamFilter::Both);
    // Before fix: result == "stdout\nstderr"
    // After fix:  result == "stderr\nstdout"
    assert!(result.starts_with("stderr"), "stderr must appear before stdout");
}
EOF
# Run: cargo test mre_006
```

## Hypothesis Table

| ID | Hypothesis | State | Summary | Evidence |
|----|-----------|-------|---------|----------|
| H1 | `merge_streams` concatenates stdout before stderr | ✅ Root Cause | Original implementation appended `stdout` first, then `stderr`, due to alphabetical argument ordering matching concatenation order | E1, E2 |
| H2 | Stream filter variant controls ordering | ❌ Refuted | `StreamFilter::Both` has no ordering semantics — ordering was implicit in the implementation body | E3 |
| H3 | The bug only appears when stderr contains a newline | ❌ Refuted | Bug reproduces with single-word inputs ("stdout", "stderr") with no newlines | E4 |

## Evidence Table

| # | Location | What it shows | Hypothesis |
|---|----------|---------------|------------|
| E1 | `src/output.rs::merge_streams` (pre-fix) | `merged.push_str(stdout)` executed before `merged.push_str(stderr)` | H1 ✅ Root Cause |
| E2 | Terminal output | `merge_streams("stdout", "stderr", &Both)` returned `"stdout\nstderr"` | H1 ✅ Root Cause |
| E3 | `src/output.rs::StreamFilter` enum | `Both` variant carries no ordering data — purely a selection flag | H2 ❌ |
| E4 | `tests/output.rs::merge_streams_ordering` | Plain strings without newlines still trigger wrong ordering | H3 ❌ |

## Root Cause

```
merge_streams(stdout, stderr, Both)
  → merged.push_str(stdout)   ← stdout appended first (alphabetical ordering)
  → merged.push_str(stderr)   ← stderr appended second
  → "stdout\nstderr"          ← WRONG: errors buried after normal output
```

The original implementation followed parameter declaration order (stdout, stderr) when concatenating.
CLI convention requires errors to appear before normal output so they are visible without scrolling.

## Why Not Caught

No test existed asserting the relative order of stderr vs. stdout in merged output. Existing tests
checked that both streams appeared in the output but did not assert which appeared first. The
ordering was not specified in any invariant or feature document at the time.

## Fix Location

`src/output.rs::merge_streams`, `StreamFilter::Both` arm:

```rust
// Before — stdout appended first:
merged.push_str( stdout );
if !stdout.is_empty() && !stderr.is_empty() { merged.push( '\n' ); }
merged.push_str( stderr );

// After — stderr appended first:
merged.push_str( stderr );
if !stderr.ends_with( '\n' ) && !stdout.is_empty() { merged.push( '\n' ); }
merged.push_str( stdout );
```

## Prevention

Any function merging stdout and stderr streams must place stderr before stdout. Add an ordering
assertion to any test that calls `merge_streams` or `process_output` with both streams non-empty:
`assert!(result.starts_with(stderr_content))`.

**Pitfall:** Stream ordering is easy to overlook in tests — always assert not just content presence but correct stream order.

## Generalized Version

**Broken assumption:** The alphabetical parameter order (stdout, stderr) is a safe guide for concatenation order.

Fails whenever:
1. Both stdout and stderr are non-empty, AND
2. The merge function appends in parameter-declaration order, AND
3. The caller expects CLI convention (errors first)

**Detection invariant:**
```
for all (stdout, stderr) where both non-empty:
  merge_streams(stdout, stderr, Both).starts_with(stderr)
```

## History

| Date | Event | Notes |
|------|-------|-------|
| 2025-11-29 | filed | Discovered during output module extraction from strs_tools |
| 2025-11-29 | fix_applied | stderr appended first in `merge_streams` `Both` arm |
| 2025-11-29 | verified | Verified by `merge_streams_ordering` test in `tests/output.rs` |
| 2025-11-29 | closed | |

## Refs: src/

- `src/output.rs` — `merge_streams`: fix applied; `Fix(BUG-006)` backreference present

## Refs: tests/

- `tests/output.rs` — `merge_streams_ordering`: ordering assertion verifying stderr-before-stdout invariant
