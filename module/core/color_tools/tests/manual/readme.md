# Manual Testing Plan — color_tools

## Overview

Manual verification of `ColorfulText` ANSI rendering in a real terminal. Automated tests cover all invariants; these scenarios exist to confirm visual output looks correct to a human and to catch terminal-specific rendering issues (e.g., color bleed, missing resets) that assertions cannot observe.

Run with the `enabled` and `serde_support` features active:

```bash
cd module/core/color_tools
cargo run --example manual_color --features enabled
```

Or use a quick inline `cargo-script` / `fn main` test binary.

## Test Matrix

### 1. Basic Color Output

| Test Case | Scenario | Input | Expected | Status |
|-----------|----------|-------|----------|--------|
| `plain_no_color` | Plain text renders without escape codes | `ColorfulText::from("status: ok")` | Terminal prints `status: ok` in default color | ⏳ |
| `yellow_warn` | Yellow color prefix applied | `.with_color("\x1b[33m")` on `"status: warn"` | Terminal prints `status: warn` in yellow | ⏳ |
| `red_error` | Red color prefix applied | `.with_color("\x1b[31m")` on `"error"` | Terminal prints `error` in red | ⏳ |
| `reset_restores_default` | ANSI reset ends color correctly | Colored text followed by plain text | Plain text after colored text renders in default terminal color | ⏳ |

### 2. Empty Text Edge Cases

| Test Case | Scenario | Input | Expected | Status |
|-----------|----------|-------|----------|--------|
| `empty_plain_render` | Empty uncolored text renders nothing | `ColorfulText::from("")` | No visible output, no escape codes | ⏳ |
| `empty_colored_render` | Empty colored text emits only escape sequences | `from("").with_color("\x1b[33m")` | No visible text, but color+reset bytes present (no terminal artifacts) | ⏳ |

### 3. Multiline Handling

| Test Case | Scenario | Input | Expected | Status |
|-----------|----------|-------|----------|--------|
| `multiline_uncolored` | Newlines preserved verbatim | `from("line1\nline2\nline3")` | Three lines printed, no ANSI artifacts | ⏳ |
| `multiline_colored_single_reset` | One reset at end of entire block | `from("line1\nline2").with_color("\x1b[33m")` | Both lines yellow, reset only after last line — intermediate lines do NOT have independent resets | ⏳ |

### 4. Display / String Conversion

| Test Case | Scenario | Input | Expected | Status |
|-----------|----------|-------|----------|--------|
| `display_equals_render` | `println!("{ct}")` matches `.render()` | Colored `ColorfulText` | Terminal output matches `.render()` output visually | ⏳ |
| `string_from_ct` | `String::from(ct)` produces rendered string | Plain and colored variants | String contains correct escape sequences (inspect with `{:?}`) | ⏳ |

### 5. Serde Feature (requires `serde_support`)

| Test Case | Scenario | Input | Expected | Status |
|-----------|----------|-------|----------|--------|
| `serde_json_plain` | Plain text serializes to JSON | `from("hello")` | JSON: `{"text":"hello","color":null}` | ⏳ |
| `serde_json_colored` | Colored text serializes with escape string | `.with_color("\x1b[33m")` | JSON: `{"text":"...","color":"\u001b[33m"}` | ⏳ |
| `serde_roundtrip_visual` | Deserialized colored text renders correctly | serialize → deserialize → `.render()` | Terminal output identical to original | ⏳ |

## Status Legend

| Symbol | Meaning |
|--------|---------|
| ✅ | Verified on latest run |
| ❌ | Failed — open issue |
| ⏳ | Not yet run |
