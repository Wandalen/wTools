# Feature: DecoratedText

### Scope

- **Purpose**: Provide typed text with optional ANSI color prefix for per-instance terminal coloring without global configuration.
- **Responsibility**: Documents the `DecoratedText` struct — its construction API, rendering contract, emptiness semantics, and integration points.
- **In Scope**: Builder pattern (`with_color`), ANSI color parameter syntax, rendering behavior, emptiness semantics, serde support, and data_fmt integration.
- **Out of Scope**: ANSI validation (caller responsibility); terminal capability detection (caller responsibility); per-line color wrapping (formatter responsibility).

### Abstract

Typed wrapper for text that may optionally carry an ANSI color prefix, enabling per-instance terminal coloring without global configuration. Designed as a transparent drop-in for `String` at call sites that need optional color.

### Design

| Scenario | Behavior |
|----------|----------|
| `DecoratedText::from("text")` | `color: None`; `render()` returns raw text clone |
| `DecoratedText::from("text").with_color("\x1b[33m")` | `color: Some(...)`, `render()` returns `"\x1b[33mtext\x1b[0m"` |
| `DecoratedText::from("text").with_color_named(Color::Yellow)` | same `render()` as `.with_color("\x1b[33m")`; avoids hand-crafting raw ANSI strings |
| `DecoratedText::from("")` | `is_empty()` returns `true`; `render()` returns `""` |
| `DecoratedText::from("").with_color("\x1b[33m")` | `is_empty()` returns `true` (text is empty regardless of color) |

#### Color Parameter Syntax

The `color` field stores a raw **ANSI SGR** (Select Graphic Rendition) escape sequence. Callers supply the complete opening sequence — ESC byte, CSI bracket, parameters, and `m` terminator:

```
\x1b  [  params  m
```

| Format | Example | Meaning |
|--------|---------|---------|
| 4-bit foreground | `"\x1b[31m"` | Red |
| 4-bit foreground | `"\x1b[33m"` | Yellow |
| Combined SGR | `"\x1b[1;33m"` | Bold + yellow |
| 256-color | `"\x1b[38;5;208m"` | 256-color orange |
| 24-bit true color | `"\x1b[38;2;255;165;0m"` | RGB orange |

The opening sequence is stored as-is; `render()` appends the reset `"\x1b[0m"` automatically. No closing sequence should be supplied — it would appear in the rendered text before the automatic reset.

**Integration with data_fmt:** `data_fmt`'s `row_details: Vec<Option<DecoratedText>>` uses this type so per-row detail lines can carry independent ANSI color without affecting the table's `TableConfig`.

**Serde support:** When the `serde_support` feature is enabled, `DecoratedText` derives `Serialize` and `Deserialize`. Both `text` and `color` fields are serialized as-is.

```toml
color_tools = { workspace = true, features = [ "enabled", "serde_support" ] }
```

### Constraints

- No ANSI validation — callers are responsible for passing valid escape sequences to `.with_color()`
- No terminal capability detection — stripping colors for non-TTY output is the caller's responsibility
- Whole-block rendering — `.render()` wraps the entire text block with one color prefix and one reset; per-line ANSI wrapping is the formatter's responsibility
- Terminal-only rendering — `render()` and `Display` produce ANSI SGR sequences for terminal emulators only; translation to HTML/CSS or other display targets requires external parsing of the raw `color` field by the caller

### Cross-References

| Entity | File | Relationship |
|--------|------|-------------|
| invariant/001 | [Transparent Conversion](../invariant/001_transparent_conversion.md) | `From<T>` zero-overhead guarantee |
| invariant/002 | [Render Reset Contract](../invariant/002_render_reset_contract.md) | Reset-only-when-colored guarantee |
| invariant/003 | [Emptiness Semantics](../invariant/003_emptiness_semantics.md) | `is_empty()` tests text, not render |
| invariant/004 | [Render Is Canonical](../invariant/004_render_is_canonical.md) | Single rendering path guarantee |
| feature/002 | [serde Support](002_serde_support.md) | Optional serde extension |
| api/001 | [DecoratedText Type](../api/001_decorated_text_type.md) | Public API reference |
