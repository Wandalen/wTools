# API: Color

### Scope

- **Purpose**: Document the public interface of `Color` — all variants, the `to_ansi()` method, and derivable trait set.
- **Responsibility**: Provides the canonical API reference for callers using `Color` with `DecoratedText::with_color_named()`.
- **In Scope**: Variant enumeration, ANSI SGR mapping, CSS mapping (html_support), `to_ansi()` contract, derives, and semver guarantees.
- **Out of Scope**: How `Color` integrates with `DecoratedText` rendering (→ `api/001`); feature design rationale (→ `feature/001`); behavioral contracts (→ `invariant/`).

### Abstract

`Color` is a semantic color enum exported at the crate root. It provides a vocabulary of named colors and encoding schemes — 4-bit named (Black–White, BrightBlack–BrightWhite), 256-color palette (`Ansi256`), and 24-bit true color (`Rgb`) — all convertible to ANSI SGR opening sequences via `to_ansi()`.

### Variants

| Variant | ANSI SGR | CSS (html_support) |
|---------|----------|--------------------|
| `Color::Black` | `\x1b[30m` | `black` |
| `Color::Red` | `\x1b[31m` | `red` |
| `Color::Green` | `\x1b[32m` | `green` |
| `Color::Yellow` | `\x1b[33m` | `yellow` |
| `Color::Blue` | `\x1b[34m` | `blue` |
| `Color::Magenta` | `\x1b[35m` | `magenta` |
| `Color::Cyan` | `\x1b[36m` | `cyan` |
| `Color::White` | `\x1b[37m` | `white` |
| `Color::BrightBlack` | `\x1b[90m` | `black` |
| `Color::BrightRed` | `\x1b[91m` | `red` |
| `Color::BrightGreen` | `\x1b[92m` | `green` |
| `Color::BrightYellow` | `\x1b[93m` | `yellow` |
| `Color::BrightBlue` | `\x1b[94m` | `blue` |
| `Color::BrightMagenta` | `\x1b[95m` | `magenta` |
| `Color::BrightCyan` | `\x1b[96m` | `cyan` |
| `Color::BrightWhite` | `\x1b[97m` | `white` |
| `Color::Ansi256(n)` | `\x1b[38;5;{n}m` | `var(--ansi256-{n})` |
| `Color::Rgb(r,g,b)` | `\x1b[38;2;{r};{g};{b}m` | `rgb(r, g, b)` |

### Operations

| Operation | Purpose | Parameters | Returns |
|-----------|---------|------------|---------|
| `to_ansi` | Produce the ANSI SGR opening sequence | — | `String` — complete escape: ESC + `[` + params + `m` |
| `to_css` | Produce CSS color value (feature `html_support`) | — | `String` — CSS keyword, `rgb(r,g,b)`, or `var(--ansi256-N)` |

**Example:**

```rust
use color_tools::Color;
assert_eq!( Color::Yellow.to_ansi(), "\x1b[33m" );
assert_eq!( Color::Rgb( 255, 165, 0 ).to_ansi(), "\x1b[38;2;255;165;0m" );
assert_eq!( Color::Ansi256( 208 ).to_ansi(), "\x1b[38;5;208m" );
```

**Derives:** `Debug`, `Clone`, `Copy`, `PartialEq`, `Eq`.

### Error Handling

`to_ansi()` is infallible by design — it always returns a valid ANSI SGR string for every variant with no allocation failures possible beyond normal heap pressure.

### Compatibility Guarantees

- **Variant set:** Adding new variants is non-breaking; removing variants is a major breaking change.
- **ANSI SGR mapping:** The mapping of each variant to its SGR sequence is stable — callers may rely on the exact byte sequences produced.
- **`Ansi256` range:** Values 0–255 produce `\x1b[38;5;{n}m`; no range validation is performed (caller responsibility).
- **`Rgb` range:** Values 0–255 per channel produce `\x1b[38;2;{r};{g};{b}m`; no range validation is performed.
- **Derives:** `Debug`, `Clone`, `Copy`, `PartialEq`, `Eq` are stable; removing any is a breaking change.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| doc | [DecoratedText Type](001_decorated_text_type.md) | `with_color_named(Color)` attaches `Color` to `DecoratedText` |
| doc | [DecoratedText](../feature/001_decorated_text.md) | Color parameter syntax specification |
