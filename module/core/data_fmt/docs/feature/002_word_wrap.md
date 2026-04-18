# Feature: Word Wrap

### Scope

- **Purpose**: Provide general-purpose word-wrapping utilities for string consumers within the data_fmt ecosystem via `WrapFormatter` and `WrapConfig`.
- **Responsibility**: Document word wrap configuration, strategies, and formatter behavior.
- **In Scope**: Break strategies, overflow handling, WrapConfig fields, WrapFormatter API, and behavior contracts.
- **Out of Scope**: Algorithm details (see `../algorithm/`), API signatures (see `../api/`).

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | `src/wrap.rs` | WrapFormatter implementation |
| test | `tests/word_wrap.rs` | Word wrap test suite |
| doc | `../algorithm/002_word_wrapping.md` | Word wrapping algorithm |
| doc | `005_auto_fit.md` | Auto-fit uses WrapFormatter for cell budget wrapping |

### BreakStrategy Enum

Controls how lines are broken when text exceeds the configured width.

```rust
#[ derive( Debug, Clone, Default, PartialEq ) ]
pub enum BreakStrategy
{
  Word,
  Hard,
  #[ default ] WordThenHard,
}
```

| Variant | Description |
|---------|-------------|
| `Word` | Break at last space before limit; if no space exists in the segment, the whole word wraps to the next line |
| `Hard` | Split at exactly `width` characters regardless of word boundaries |
| `WordThenHard` | Word-boundary first; hard-break only when a single token exceeds available width (default) |

### Overflow Enum

Controls what happens when output exceeds `max_lines`.

```rust
#[ derive( Debug, Clone, PartialEq ) ]
pub enum Overflow
{
  Truncate,
  Ellipsis( String ),
}
```

| Variant | Description |
|---------|-------------|
| `Truncate` | Discard lines beyond `max_lines` |
| `Ellipsis( String )` | Append the given string to the last kept line, truncating content so total line length does not exceed `width` |

### WrapConfig Fields and Defaults

| Field | Type | Default |
|-------|------|---------|
| `width` | `usize` | `80` |
| `initial_indent` | `String` | `""` |
| `subsequent_indent` | `String` | `""` |
| `break_strategy` | `BreakStrategy` | `WordThenHard` |
| `break_long_words` | `bool` | `true` |
| `preserve_newlines` | `bool` | `true` |
| `max_lines` | `Option< usize >` | `None` |
| `overflow` | `Overflow` | `Truncate` |
| `tab_width` | `usize` | `4` |

### Builder Methods

All builder methods are `#[ must_use ]` and return `Self`:

1. `width`
2. `initial_indent`
3. `subsequent_indent`
4. `indent` -- sets both `initial_indent` and `subsequent_indent` to the same value
5. `break_strategy`
6. `break_long_words`
7. `preserve_newlines`
8. `max_lines`
9. `overflow`
10. `tab_width`

### WrapFormatter

```rust
pub struct WrapFormatter
{
  config : WrapConfig,
}

impl WrapFormatter
{
  pub fn new() -> Self;
  pub fn with_config( config : WrapConfig ) -> Self;
  pub fn wrap( &self, text : &str ) -> Vec< String >;
  pub fn wrap_joined( &self, text : &str ) -> String;  // wrap().join( "\n" )
}
```

- `new()` -- creates a formatter with default `WrapConfig`.
- `with_config( config )` -- creates a formatter with the given config.
- `wrap( text )` -- returns wrapped lines as a vector.
- `wrap_joined( text )` -- returns wrapped lines joined by `"\n"`.

### Behavior Contracts

1. A line in `wrap()` output never exceeds `width` chars (measured as char count) **except** when `break_long_words=false` and a single token is longer than the available space.
2. `initial_indent` is prepended to line 0; `subsequent_indent` to lines 1+. Both count toward `width`.
3. `preserve_newlines=true`: `\n` in input is a hard break; wrapping restarts with `subsequent_indent`.
4. `preserve_newlines=false`: `\n` treated as a single space.
5. `tab_width`: each `\t` in input expanded to `tab_width` spaces before processing.
6. `max_lines=Some(n)` + `Overflow::Truncate`: output has at most `n` lines.
7. `max_lines=Some(n)` + `Overflow::Ellipsis(s)`: last kept line has `s` appended, truncating content so the total line length does not exceed `width`.

