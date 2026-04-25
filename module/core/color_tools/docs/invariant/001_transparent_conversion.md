# Invariant: Transparent Conversion

### Scope

- **Purpose**: Guarantee that `From<String>` and `From<&str>` produce no ANSI side effects.
- **Responsibility**: Documents the transparent conversion invariant and its enforcement in tests t01, t02, and t10.
- **In Scope**: `From<String>` and `From<&str>` trait implementations; enforcement tests.
- **Out of Scope**: Rendering behavior (→ `invariant/002`); full string conversion path (→ `invariant/004`).

### Abstract

`DecoratedText::from(text)` — whether `text` is `String` or `&str` — produces a value with `color: None`. No ANSI prefix is injected, no allocation beyond the text itself occurs.

### Invariant Statement

`From<String>` and `From<&str>` implementations MUST set `color: None`. The resulting `DecoratedText` is a transparent wrapper around the input text with no escape code side effects.

### Rationale

This invariant enables `DecoratedText` to be a transparent drop-in for `String` at all existing call sites. Code written as `"detail text".into()` or `String::from("x").into()` continues to compile and behave identically after a type migration from `Option<String>` to `Option<DecoratedText>`.

### Enforcement Mechanism

- Test `t01_from_string_no_color` verifies `From<String>` sets `color: None`.
- Test `t02_from_str_no_color` verifies `From<&str>` sets `color: None`.
- Test `t10_roundtrip_uncolored` verifies `String → DecoratedText → String` round-trip preserves text.

### Violation Consequences

Setting `color` to any value in `From<String>` or `From<&str>` would inject invisible ANSI escape codes into all existing call sites that use `.into()` for `String` conversion — silently corrupting plain-text pipelines, log files, and serialized output.

### Verification

```rust
let ct = DecoratedText::from( "test".to_string() );
assert_eq!( ct.color, None );
assert_eq!( ct.render(), "test" );  // no escape codes
```

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| feature/001 | [DecoratedText](../feature/001_decorated_text.md) | Parent feature |
| invariant/002 | [Render Reset Contract](002_render_reset_contract.md) | Sibling — render behavior |
| invariant/004 | [Render Is Canonical](004_render_is_canonical.md) | Sibling — render delegation |
