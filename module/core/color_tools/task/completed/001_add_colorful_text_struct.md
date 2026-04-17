# Add `ColorfulText` struct to `color_tools`

## Execution State

- **Executor Type:** any
- **Actor:** claude
- **Claimed At:** 2026-04-17
- **Status:** ✅ (Completed)
- **Validated By:** claude (self-validated, independent re-validation recommended)
- **Validation Date:** 2026-04-17

## Goal

Add a `ColorfulText` struct to `color_tools` — a typed wrapper around a plain `String` that
carries an optional ANSI escape color prefix. Required by `tree_fmt` task 017 (sub-row detail
lines) and its ANSI extension (task 018), both of which need to pass colored-or-plain text
through a data pipeline without losing color intent.

## In Scope

- `src/colorful_text.rs` — new file: `ColorfulText` struct + all impls
- `src/lib.rs` — re-export `ColorfulText` from the crate root
- `src/readme.md` — register the new file
- `tests/colorful_text_test.rs` — 17 tests covering the full API surface
- `tests/readme.md` — register the new test file
- Optional serde support behind `serde_support` feature flag
- `readme.md` — document the struct in the crate overview

## Out of Scope

- Palette / theme management (separate future concern)
- Terminal capability detection (no-tty stripping — caller responsibility)
- Any changes to dependent crates

## Description

### Problem

`tree_fmt` needed to accept optional per-row detail lines that could be either plain text or
ANSI-colored text. Using a raw `String` forces the formatter to interpret escape codes itself
or strip them; using a newtype preserves intent and keeps the coloring concern in the type.

### Design

```rust
pub struct ColorfulText
{
  pub text  : String,
  pub color : Option< String >,
}
```

- `color` stores the raw ANSI escape prefix (e.g. `"\x1b[33m"`). No validation — callers
  are responsible for passing valid escape sequences.
- `.render()` returns `color + text + "\x1b[0m"` when colored, plain `text` when not.
- `is_empty()` tests `text.is_empty()` only — color presence does not affect emptiness.
- `From<String>` / `From<&str>` — transparent, no color, no allocation overhead.
- `From<ColorfulText> for String` — delegates to `.render()`.
- `Display` — delegates to `.render()`.

### Per-line ANSI wrapping (Algorithm 3)

When a `ColorfulText` detail spans multiple lines (`\n` in `text`), `tree_fmt`'s renderer
applies the color independently per line:

```
for line in ct.text.lines():
  output += color + line + ANSI_RESET + "\n"
```

This prevents terminal color bleed when a single RESET at end of a multiline block would
only reset after the last line while intermediate lines stay colored in some terminals.

## Requirements

- `ColorfulText` must be accessible as `color_tools::ColorfulText`
- `.render()` must produce `color + text + "\x1b[0m"` when colored, plain `text` when not
- `is_empty()` must test `text.is_empty()` only (not `render().is_empty()`)
- `From<String>`, `From<&str>`, `From<ColorfulText> for String`, `Display` must all be implemented
- `serde_support` feature must enable `Serialize` / `Deserialize`
- All 17 tests must pass under `RUSTFLAGS="-D warnings" cargo nextest run --all-features`
- Clippy must report zero warnings

## Test Matrix

| # | Scenario | API Under Test | Expected |
|---|----------|----------------|----------|
| T01 | Plain construction from `&str` | `From<&str>` | `color = None`, `text` matches |
| T02 | Plain `.render()` | `.render()` | Returns plain text, no escapes |
| T03 | Colored construction | `.with_color("\x1b[33m")` | `color = Some(...)` |
| T04 | Colored `.render()` | `.render()` | `color + text + "\x1b[0m"` |
| T05 | `is_colored()` plain | `.is_colored()` | `false` |
| T06 | `is_colored()` colored | `.is_colored()` | `true` |
| T07 | `is_empty()` non-empty | `.is_empty()` | `false` |
| T08 | `is_empty()` empty text | `.is_empty()` | `true` even when colored |
| T09 | `From<String>` | `From<String>` | Same as `From<&str>` |
| T10 | `From<ColorfulText> for String` plain | `String::from(ct)` | Plain text |
| T11 | `From<ColorfulText> for String` colored | `String::from(ct)` | Rendered with ANSI |
| T12 | `Display` plain | `format!("{ct}")` | Plain text |
| T13 | `Display` colored | `format!("{ct}")` | Rendered with ANSI |
| T14 | `with_color` override | Second `.with_color()` call | New color replaces old |
| T15 | Empty colored `.render()` | `ColorfulText::from("").with_color(...)` | Empty string (no ANSI codes) |
| T16 | Multiline uncolored | `From<"line1\nline2">` | `render()` = `"line1\nline2"` |
| T17 | Multiline colored single reset | colored multiline | `render()` has one reset at end |

## Outcomes

**Source changes (2 files):**
- `src/colorful_text.rs` — new: `ColorfulText` struct, 5 methods, 4 trait impls
- `src/lib.rs` — re-export `ColorfulText`

**Test file (1 new):**
- `tests/colorful_text_test.rs` — 17 tests covering T01–T17

**Documentation:**
- `src/readme.md` — registered `colorful_text.rs`
- `readme.md` — documented struct in crate overview
- `tests/readme.md` — registered test file

**Verification:** All 17 tests pass, 0 clippy errors, doc tests pass.
