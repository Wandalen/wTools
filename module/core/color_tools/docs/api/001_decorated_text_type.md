# API: DecoratedText

### Scope

- **Purpose**: Document the public interface of `DecoratedText` — all methods, trait implementations, and compatibility guarantees.
- **Responsibility**: Provides the canonical API reference for callers using `DecoratedText` in their own code.
- **In Scope**: Method signatures, conversion trait implementations, rendering targets and translation scope, error handling policy, and semver stability guarantees.
- **Out of Scope**: Internal implementation details (→ `src/decorated_text.rs`); behavioral contracts (→ `invariant/`).

### Abstract

Public interface of `DecoratedText` — a typed text wrapper with optional ANSI color. Provides builder-style color attachment, rendering to terminal-ready strings, and transparent conversions to and from plain strings.

### Operations

| Operation | Purpose | Parameters | Returns |
|-----------|---------|------------|---------|
| `with_color` | Attach a raw ANSI color prefix | ANSI escape string (e.g. `"\x1b[33m"`) | `Self` for builder chaining |
| `with_color_named` | Attach a semantic color | `Color` variant (e.g. `Color::Yellow`) | `Self`; stores named_color for HTML output |
| `render` | Produce terminal-ready string | — | Colored: `prefix + text + reset`; uncolored: plain text |
| `render_html` | Produce HTML output (feature `html_support`) | — | Named-color: `<span style="color: css">text</span>`; plain/raw: escaped text |
| `is_colored` | Query whether a color is attached | — | Boolean |
| `is_empty` | Query whether the text content is empty | — | Boolean (tests text field, not render output) |

**Conversions:**

| Conversion | Direction | Behavior |
|------------|-----------|----------|
| `From<String>` | String → DecoratedText | Transparent — `color: None` |
| `From<&str>` | &str → DecoratedText | Transparent — `color: None`, text owned |
| `From<DecoratedText> for String` | DecoratedText → String | Delegates to `render()` |
| `Display` | DecoratedText → formatted output | Delegates to `render()` |
| `Default` | — → DecoratedText | Empty text, no color |

**Derives:** `Debug`, `Clone`, `PartialEq`, `Eq`, `Default`.
Optional: `Serialize`, `Deserialize` (feature `serde_support`).

**Conditional field:** `named_color: Option<Color>` (feature `html_support`) — populated by `with_color_named`; skipped in serde output.

### Color Type

`Color` is a semantic color enum exported at the crate root. Use it with `with_color_named()` to avoid hand-crafting raw ANSI strings. See [api/002 — Color Type](002_color_type.md) for the full variant table and `to_ansi()` contract.

### Rendering Targets

`render()` and `Display` produce **ANSI SGR terminal sequences** only. They are interpreted by xterm-compatible terminal emulators (xterm, iTerm2, Windows Terminal, GNOME Terminal). No other rendering target is built in.

#### Translating to Other Targets

The `text` and `color` fields are `pub`. A caller targeting a non-terminal output (HTML, RTF, log files with color tags, etc.) can read them directly and produce the required format:

```rust
// Example: naive HTML translator
fn to_html( ct : &DecoratedText ) -> String
{
  match &ct.color
  {
    // caller must parse the ANSI SGR string to extract CSS color
    Some( _ansi ) => format!( "<span class=\"colored\">{}</span>", ct.text ),
    None          => ct.text.clone(),
  }
}
```

**Translation note:** For HTML output, prefer `with_color_named(Color)` with the `html_support` feature — `render_html()` produces a typed `<span>` without any ANSI parsing. `with_color(raw_str)` stores raw SGR bytes; `render_html()` cannot derive CSS from them and returns plain escaped text.

### Error Handling

All operations are infallible by design. No ANSI validation is performed — `with_color` accepts any string. This is deliberate: validation belongs at the call site, not in the data type, keeping the type lightweight and allocation-free beyond the text itself.

### Compatibility Guarantees

- **Public fields:** `text: String`, `color: Option<String>`, and (under `html_support`) `named_color: Option<Color>` are public. Changing their types is a breaking change.
- **Semantic versioning:** Major version bump required for field type changes, method removal, or behavioral changes to `render()`.
- **No ANSI validation contract:** `with_color` will never validate or reject input — callers may rely on this pass-through behavior.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| feature/001 | [DecoratedText](../feature/001_decorated_text.md) | Feature specification |
| feature/002 | [serde Support](../feature/002_serde_support.md) | Optional serde derives |
| invariant/001 | [Transparent Conversion](../invariant/001_transparent_conversion.md) | Conversion guarantee |
| invariant/002 | [Render Reset Contract](../invariant/002_render_reset_contract.md) | Render behavior |
| invariant/003 | [Emptiness Semantics](../invariant/003_emptiness_semantics.md) | is_empty semantics |
| invariant/004 | [Render Is Canonical](../invariant/004_render_is_canonical.md) | Single render path |
| feature/003 | [HTML Rendering](../feature/003_html_rendering.md) | `render_html()` design, raw-vs-named boundary, CSS mapping |
| api/002 | [Color Type](002_color_type.md) | Full variant table, `to_ansi()` and `to_css()` contracts |
