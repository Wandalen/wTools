# Add Word Wrapping Utility

## Goal

`tree_fmt` exposes a `WrapFormatter` / `WrapConfig` API so any downstream crate
can wrap a string to a configured column width with independent first-line and
continuation-line indent strings, a choice of break strategy, and optional line
capping with overflow handling. All behavior is verified by a test suite that
runs under `w3 .test l::3` with zero failures and zero warnings.

SMART breakdown:
- **Specific** — exact public API contract is fixed below (field names, method
  signatures, defaults, behavior contracts).
- **Measurable** — `w3 .test l::3` passes green; every row of the Test Matrix
  has a corresponding passing test; kbase integration test passes.
- **Achievable** — scoped to one new source file plus one new test file; no
  existing code touched except `lib.rs` re-exports.
- **Relevant** — kbase validation output renderer is waiting for this utility
  to replace the current hard-truncation of issue messages.
- **Time-bound** — single deliverable, no external dependencies.

## In Scope

- `src/wrap.rs`: `WrapConfig`, `WrapFormatter`, `BreakStrategy`, `Overflow`
- `tests/word_wrap.rs`: systematic test coverage per Test Matrix
- Re-exports from `lib.rs`: `WrapConfig`, `WrapFormatter`, `BreakStrategy`, `Overflow`
- Doc comments (`///`) on every public item
- All 11 `WrapConfig` fields present (including `strip_ansi`, `unicode_aware`)
  with their default values and builder methods

## Out of Scope

- `strip_ansi=true` runtime behavior — field and builder exist, actual ANSI
  stripping is deferred; `strip_ansi` has no effect on output in this task
- `unicode_aware=true` runtime behavior — field and builder exist, but width
  is measured by char count not display width; deferred to a future task
- Streaming / iterator output (only `Vec<String>` and `String` return forms)
- Performance optimization
- no-std support
- kbase integration — kbase calls this API in a separate kbase task

## Public API Contract

### Enums

```rust
#[ derive( Debug, Clone, Default, PartialEq ) ]
pub enum BreakStrategy { Word, Hard, #[ default ] WordThenHard }

#[ derive( Debug, Clone, PartialEq ) ]
pub enum Overflow { Truncate, Ellipsis( String ) }
```

### `WrapConfig` — fields and defaults

| Field                | Type             | Default        |
|----------------------|------------------|----------------|
| `width`              | `usize`          | `80`           |
| `initial_indent`     | `String`         | `""`           |
| `subsequent_indent`  | `String`         | `""`           |
| `break_strategy`     | `BreakStrategy`  | `WordThenHard` |
| `break_long_words`   | `bool`           | `true`         |
| `preserve_newlines`  | `bool`           | `true`         |
| `max_lines`          | `Option<usize>`  | `None`         |
| `overflow`           | `Overflow`       | `Truncate`     |
| `tab_width`          | `usize`          | `4`            |
| `strip_ansi`         | `bool`           | `false`        |
| `unicode_aware`      | `bool`           | `false`        |

Builder methods (all `#[must_use]`, `mut self -> Self`):
`width`, `initial_indent`, `subsequent_indent`,
`indent` (sets both indent fields to the same value),
`break_strategy`, `break_long_words`, `preserve_newlines`,
`max_lines`, `overflow`, `tab_width`, `strip_ansi`, `unicode_aware`.

### `WrapFormatter`

```rust
pub struct WrapFormatter { config: WrapConfig }

impl WrapFormatter {
  pub fn new() -> Self                               // width=80, all defaults
  pub const fn with_config( config: WrapConfig ) -> Self
  pub fn wrap( &self, text: &str ) -> Vec<String>   // one entry per output line
  pub fn wrap_joined( &self, text: &str ) -> String  // wrap().join("\n")
}
```

### Behavior Contracts

- A line in `wrap()` output never exceeds `width` chars (measured as char count)
  **except** when `break_long_words=false` and a single token is longer than the
  available space.
- `initial_indent` is prepended to line 0; `subsequent_indent` to lines 1+.
  Both count toward `width`.
- `preserve_newlines=true`: `\n` in input is a hard break; wrapping restarts
  with `subsequent_indent`.
- `preserve_newlines=false`: `\n` treated as a single space.
- `tab_width`: each `\t` in input expanded to `tab_width` spaces before processing.
- `max_lines=Some(n)` + `Overflow::Truncate`: output has exactly `n` lines max.
- `max_lines=Some(n)` + `Overflow::Ellipsis(s)`: last kept line has `s` appended,
  truncating content so the total line length ≤ `width`.
- `BreakStrategy::Word`: break at last space before limit; if no space, word
  wraps whole to next line (may overflow if `break_long_words=false`).
- `BreakStrategy::Hard`: split at exactly `width` chars.
- `BreakStrategy::WordThenHard`: word-boundary first; hard-break only when a
  single token exceeds available width.

## Test Matrix

Every cell must have at least one test. Rows = input scenarios.
Columns = configuration dimensions tested against that input.

| # | Input Scenario | Config Under Test | Expected Behavior |
|---|---------------|-------------------|-------------------|
| T01 | Empty string | defaults | `vec![]` or `vec![""]` — contract must be documented |
| T02 | Single short word | defaults | returns as-is, no wrap |
| T03 | Multiple words, fits on one line | `width=80` | single line, unchanged |
| T04 | Multiple words, exceeds width | `width=20` | split at last space before col 20 |
| T05 | Multiple wraps needed | `width=10` | three or more output lines |
| T06 | `initial_indent` set, fits after | `initial_indent=">> "` | first line prefixed, others bare |
| T07 | `subsequent_indent` set | `subsequent_indent="   "` | first line bare, continuations prefixed |
| T08 | Different first/continuation | kbase: `initial_indent="     ② "`, `subsequent_indent="        "`, `width=120` | wraps long message, continuation aligns |
| T09 | `indent()` convenience | `indent("  ")` | both indent fields set to `"  "` |
| T10 | `preserve_newlines=true` | input with `\n` inside | hard break at `\n` position |
| T11 | `preserve_newlines=false` | input with `\n` inside | `\n` treated as space |
| T12 | Tab in input | `tab_width=4` | tab → 4 spaces before wrap |
| T13 | `max_lines=Some(2)` + Truncate | long input | at most 2 lines in output |
| T14 | `max_lines=Some(2)` + `Ellipsis("…")` | long input | second line ends with `"…"` |
| T15 | `BreakStrategy::Word` | no space in token | no mid-word split |
| T16 | `BreakStrategy::Hard` | long word | split at exact char position |
| T17 | `BreakStrategy::WordThenHard` | token longer than available | hard-breaks the overlong token |
| T18 | `break_long_words=true` | word > width | word hard-broken |
| T19 | `break_long_words=false` | word > width | word overflows (line > width) |
| T20 | `wrap_joined()` | any multi-line input | equals `wrap().join("\n")` |

## Work Procedure

Execute in order. Do not skip or reorder steps.

1. **Read rulebooks** — `kbase .rulebooks`; note any constraints that affect
   file layout or code style.
2. **Write `tests/word_wrap.rs`** — implement every test case in the Test Matrix
   (T01–T20). No implementation exists yet; tests must fail to compile.
3. **Confirm Red state** — `cargo nextest run --test word_wrap` must produce
   compile errors (types absent). If any test passes, it is trivially wrong.
4. **Write `src/wrap.rs`** — implement all types and behavior contracts.
   Add `mod wrap;` to `lib.rs` and re-export the four public types.
5. **Green state** — `w3 .test l::3` must pass with zero failures, zero warnings.
6. **Refactor if needed** — ensure no function exceeds 50 lines, no duplication,
   all public items have `///` doc comments. Tests must still pass after.
7. **Walk Validation List** — every answer must be YES before proceeding.
8. **Update task status** — set this task to ✅ in `task/readme.md`, recalculate
   advisability to 0, re-sort index.

## Validation List

Desired answer for every question is YES.

**Goal achieved**
- [ ] Does `use tree_fmt::{ WrapConfig, WrapFormatter, BreakStrategy, Overflow };` compile?
- [ ] Does the kbase integration test (T08) pass with the exact indent strings specified?
- [ ] Does `w3 .test l::3` pass with zero failures?
- [ ] Does `cargo clippy --all-targets --all-features -- -D warnings` report zero warnings?

**Public API completeness**
- [ ] Does `WrapConfig` have exactly 11 fields matching the Field/Default table?
- [ ] Does `WrapConfig::new()` produce a config where `width=80`, `initial_indent=""`,
      `subsequent_indent=""`, `break_long_words=true`, `preserve_newlines=true`,
      `max_lines=None`, `tab_width=4`, `strip_ansi=false`, `unicode_aware=false`?
- [ ] Does `WrapConfig::indent("x")` set both `initial_indent` and `subsequent_indent` to `"x"`?
- [ ] Are all 12 builder methods annotated with `#[must_use]`?
- [ ] Does `WrapFormatter::wrap_joined(t)` equal `WrapFormatter::wrap(t).join("\n")` for
      any input?

**Behavior contracts**
- [ ] Does wrapping `"hello world"` at `width=5` produce `["hello", "world"]`
      (word strategy, no indent)?
- [ ] Does wrapping with `max_lines=Some(2)` + `Overflow::Truncate` produce ≤2 lines?
- [ ] Does wrapping with `max_lines=Some(2)` + `Overflow::Ellipsis("…")` end the
      second line with `"…"` and keep that line ≤width chars?
- [ ] Does `preserve_newlines=true` cause a `\n` in input to produce a hard break?
- [ ] Does `break_long_words=false` allow a single token wider than `width` to overflow?

**Test coverage**
- [ ] Does `tests/word_wrap.rs` contain at least 20 `#[test]` functions?
- [ ] Are all test functions free of `#[ignore]` attributes?
- [ ] Did all tests fail to compile against the unmodified codebase (confirmed at step 3)?

**Code quality**
- [ ] Are all functions in `src/wrap.rs` ≤50 lines?
- [ ] Does every `pub` item in `src/wrap.rs` have a `///` doc comment?
- [ ] Is there zero commented-out code in `src/wrap.rs`?

## Validation Procedure

**VP1 — API reachability**
`grep -n "pub use" tree_fmt/src/lib.rs | grep -E "WrapConfig|WrapFormatter|BreakStrategy|Overflow"`
Expected: four matches. Zero matches means re-export was forgotten.

**VP2 — Default values**
Write a doc-test or unit test asserting each field default. Any mismatch between
the Field/Default table above and `WrapConfig::new()` is a spec deviation.

**VP3 — Line-length invariant**
For each test that does not use `break_long_words=false`, assert that every
element of `wrap(input).iter().map(|l| l.chars().count())` is ≤ `width`.
A violation means a behavior contract is broken.

**VP4 — Kbase integration**
Run test T08 with the exact parameters kbase will use:
`width=120`, `initial_indent="     ② "`, `subsequent_indent="        "`,
input = the full 131-char constructs mismatch message.
Assert: output has exactly 2 lines; first line ≤120 chars; second line starts
with `"        "` and is ≤120 chars.

**VP5 — No trivially-passing tests**
`grep -n "assert!(true)\|assert_eq!(1, 1)" tests/word_wrap.rs` — expect zero matches.

**VP6 — No implementation stubs**
`grep -n "todo!\|unimplemented!" src/wrap.rs` — expect zero matches.
