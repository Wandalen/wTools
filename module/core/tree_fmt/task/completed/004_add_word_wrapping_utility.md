# Add Word Wrapping Utility

## Status: ✅ Completed

## Validation List

**Goal achieved**
- [x] Does `use tree_fmt::{ WrapConfig, WrapFormatter, BreakStrategy, Overflow };` compile?
- [x] Does the kbase integration test (T08) pass with the exact indent strings specified?
- [x] Does `w3 .test l::3` pass with zero failures?
- [x] Does `cargo clippy --all-targets --all-features -- -D warnings` report zero warnings?

**Public API completeness**
- [x] Does `WrapConfig` have exactly 11 fields matching the Field/Default table?
- [x] Does `WrapConfig::new()` produce a config where `width=80`, `initial_indent=""`,
      `subsequent_indent=""`, `break_long_words=true`, `preserve_newlines=true`,
      `max_lines=None`, `tab_width=4`, `strip_ansi=false`, `unicode_aware=false`?
- [x] Does `WrapConfig::indent("x")` set both `initial_indent` and `subsequent_indent` to `"x"`?
- [x] Are all 12 builder methods annotated with `#[must_use]`?
- [x] Does `WrapFormatter::wrap_joined(t)` equal `WrapFormatter::wrap(t).join("\n")` for
      any input?

**Behavior contracts**
- [x] Does wrapping `"hello world"` at `width=5` produce `["hello", "world"]`
      (word strategy, no indent)?
- [x] Does wrapping with `max_lines=Some(2)` + `Overflow::Truncate` produce ≤2 lines?
- [x] Does wrapping with `max_lines=Some(2)` + `Overflow::Ellipsis("…")` end the
      second line with `"…"` and keep that line ≤width chars?
- [x] Does `preserve_newlines=true` cause a `\n` in input to produce a hard break?
- [x] Does `break_long_words=false` allow a single token wider than `width` to overflow?

**Test coverage**
- [x] Does `tests/word_wrap.rs` contain at least 20 `#[test]` functions?
- [x] Are all test functions free of `#[ignore]` attributes?
- [x] Did all tests fail to compile against the unmodified codebase (confirmed at step 3)?

**Code quality**
- [x] Are all functions in `src/wrap.rs` ≤50 lines?
- [x] Does every `pub` item in `src/wrap.rs` have a `///` doc comment?
- [x] Is there zero commented-out code in `src/wrap.rs`?

**Documentation**
- [x] Does `src/readme.md` have a `wrap.rs` row in its Responsibility Table?
- [x] Does `tests/readme.md` have a `word_wrap.rs` row in its Responsibility Table?
- [x] Does the test count prose in `tests/readme.md` match actual `cargo nextest` output?

## Deliverables

- `src/wrap.rs` — `WrapConfig` (11 fields, 12 builder methods), `WrapFormatter` (wrap + wrap_joined), `BreakStrategy`, `Overflow`; all private helpers (hard_chunks, hard_break_str, wrap_words, push_overlong_word, flush_pending, apply_overflow, expand_tabs, char_count, indent_for, available_for)
- `tests/word_wrap.rs` — 22 tests: T01–T20 + `defaults_match_spec` + `indent_counts_toward_width`
- `src/lib.rs` — added `mod wrap;` + `pub use wrap::{ WrapConfig, WrapFormatter, BreakStrategy, Overflow };`
- `src/readme.md` — added `wrap.rs` row
- `tests/readme.md` — added `word_wrap.rs` row, updated test count to 331 nextest + 73 doc

## Final Test Results

- nextest: 331 passed, 0 failed
- doc tests: 73 passed, 0 failed
- clippy: 0 warnings
