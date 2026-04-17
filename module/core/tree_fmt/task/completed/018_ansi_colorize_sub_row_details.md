# ANSI colorize sub-row detail lines

## Execution State

- **Executor Type:** any
- **Actor:** claude
- **Claimed At:** 2026-04-17
- **Status:** ✅ (Completed)
- **Validated By:** claude (self-validated, independent re-validation recommended)
- **Validation Date:** 2026-04-17

## Goal

Extend the sub-row detail line feature (task 017) to support optional per-line ANSI coloring
via `color_tools::ColorfulText`. Task 017 explicitly deferred this as "separate future task
if needed". The implementation upgrades `row_details` from `Vec<Option<String>>` to
`Vec<Option<color_tools::ColorfulText>>` and applies Algorithm 3 (per-line ANSI wrapping)
in the renderer.

## In Scope

- `src/data.rs` — upgrade `row_details` field type from `Vec<Option<String>>` to
  `Vec<Option<color_tools::ColorfulText>>`; update `with_details()` constructor
- `src/table_tree.rs` — upgrade `RowBuilder::row_details` field and `add_row_with_detail`
  / `add_row_with_detail_mut` signatures to accept `Option<color_tools::ColorfulText>`
- `src/formatters/table.rs` — implement Algorithm 3: per-line ANSI wrapping in the
  detail-line rendering branch
- `Cargo.toml` — add `color_tools` workspace dependency
- `tests/sub_row_test.rs` — add tests T29–T33 covering Algorithm 3 corner cases

## Out of Scope

- Any change to `ExpandedFormatter`, `TreeFormatter`, or other formatters
- Changes to the `kbase` consumer
- Palette / theme configuration

## Description

### Type Upgrade

`row_details` changes from `Vec<Option<String>>` to `Vec<Option<ColorfulText>>`. Plain
callers are unaffected because `ColorfulText` implements `From<String>` and `From<&str>`
transparently — `.add_row_with_detail(row, Some("text".into()))` continues to compile.

### Algorithm 3 — Per-Line ANSI Wrapping

When the formatter encounters a `ColorfulText` detail:

```
if ct.is_empty(): skip (no output)
else if ct.color.is_none(): output indent + ct.text + "\n"
else:
  for line in ct.text.lines():
    output += indent + color + line + ANSI_RESET + "\n"
```

Each line receives its own color prefix and reset. This prevents terminal color bleed
when a multiline block uses a single reset at the end — intermediate lines between the
reset and the next color prefix would render uncolored in many terminals.

Edge cases handled:
- `"\n".lines()` in Rust yields `[""]` (one empty string), not empty iterator — the
  renderer skips empty strings within the loop to avoid a blank colored line for a
  detail containing only `"\n"`.
- A `ColorfulText` with `text = "\n"` is NOT considered empty (`is_empty()` tests
  `text.is_empty()` only) — the renderer emits the sub-row as a blank line `"  \n"`.

## Requirements

- `row_details: Vec<Option<ColorfulText>>` in both `TableView` and `RowBuilder`
- Algorithm 3 applied in `format_internal`
- Existing tests T01–T28 must pass without modification
- New tests T29–T33 must pass
- `RUSTFLAGS="-D warnings" cargo nextest run --all-features` → 0 failures
- `cargo clippy --all-targets --all-features -- -D warnings` → 0 errors
- `RUSTDOCFLAGS="-D warnings" cargo test --doc --all-features` → 0 failures

## Test Matrix — Algorithm 3 (T29–T33)

| # | Scenario | Expected |
|---|----------|----------|
| T29 | Plain `ColorfulText` detail (no color) | Rendered without any ANSI codes |
| T30 | Colored multiline detail (`"alpha\nbeta\ngamma"`) | Each line gets independent color+reset; exactly 3 resets in output |
| T31 | Custom indent with colored detail | Indent appears before color prefix (`indent_pos < ansi_pos`) |
| T32 | Detail with trailing `"\n"` vs without | Both outputs identical (trailing newline stripped naturally) |
| T33 | Detail is only `"\n"` | `is_empty()` = false; renders one blank detail line `"  "` |

## Outcomes

**Source changes (4 files):**
- `src/data.rs` — `row_details` field type upgraded to `Vec<Option<ColorfulText>>`
- `src/table_tree.rs` — `RowBuilder` field and method signatures upgraded
- `src/formatters/table.rs` — Algorithm 3 per-line ANSI wrapping in detail renderer
- `Cargo.toml` — `color_tools` workspace dependency added

**Test additions:**
- `tests/sub_row_test.rs` — tests T29–T33 added (total 33 tests in file)

**Verification:** All 33 sub_row tests pass, all existing tree_fmt tests pass, 0 clippy errors.
