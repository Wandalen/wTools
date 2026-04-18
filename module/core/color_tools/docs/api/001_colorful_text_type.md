# API: ColorfulText

### Scope

- **Purpose**: Document the public interface of `ColorfulText` — all methods, trait implementations, and compatibility guarantees.
- **Responsibility**: Provides the canonical API reference for callers using `ColorfulText` in their own code.
- **In Scope**: Method signatures, conversion trait implementations, error handling policy, and semver stability guarantees.
- **Out of Scope**: Internal implementation details (→ `src/colorful_text.rs`); behavioral contracts (→ `invariant/`).

### Abstract

Public interface of `ColorfulText` — a typed text wrapper with optional ANSI color. Provides builder-style color attachment, rendering to terminal-ready strings, and transparent conversions to and from plain strings.

### Operations

| Operation | Purpose | Parameters | Returns |
|-----------|---------|------------|---------|
| `with_color` | Attach an ANSI color prefix | ANSI escape string (e.g. `"\x1b[33m"`) | `Self` for builder chaining |
| `render` | Produce terminal-ready string | — | Colored: `prefix + text + reset`; uncolored: plain text |
| `is_colored` | Query whether a color is attached | — | Boolean |
| `is_empty` | Query whether the text content is empty | — | Boolean (tests text field, not render output) |

**Conversions:**

| Conversion | Direction | Behavior |
|------------|-----------|----------|
| `From<String>` | String → ColorfulText | Transparent — `color: None` |
| `From<&str>` | &str → ColorfulText | Transparent — `color: None`, text owned |
| `From<ColorfulText> for String` | ColorfulText → String | Delegates to `render()` |
| `Display` | ColorfulText → formatted output | Delegates to `render()` |
| `Default` | — → ColorfulText | Empty text, no color |

**Derives:** `Debug`, `Clone`, `PartialEq`, `Eq`, `Default`.
Optional: `Serialize`, `Deserialize` (feature `serde_support`).

### Error Handling

All operations are infallible by design. No ANSI validation is performed — `with_color` accepts any string. This is deliberate: validation belongs at the call site, not in the data type, keeping the type lightweight and allocation-free beyond the text itself.

### Compatibility Guarantees

- **Public fields:** `text: String` and `color: Option<String>` are public. Changing their types is a breaking change.
- **Semantic versioning:** Major version bump required for field type changes, method removal, or behavioral changes to `render()`.
- **No ANSI validation contract:** `with_color` will never validate or reject input — callers may rely on this pass-through behavior.

### Cross-References

| Entity | File | Relationship |
|--------|------|-------------|
| feature/001 | [ColorfulText](../feature/001_colorful_text.md) | Feature specification |
| invariant/001 | [Transparent Conversion](../invariant/001_transparent_conversion.md) | Conversion guarantee |
| invariant/002 | [Render Reset Contract](../invariant/002_render_reset_contract.md) | Render behavior |
| invariant/003 | [Emptiness Semantics](../invariant/003_emptiness_semantics.md) | is_empty semantics |
| invariant/004 | [Render Is Canonical](../invariant/004_render_is_canonical.md) | Single render path |
