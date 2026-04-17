# Invariant: Transparent Conversion

## Statement

`ColorfulText::from(text)` — whether `text` is `String` or `&str` — MUST produce a `ColorfulText` with `color: None`. No ANSI prefix is injected, no allocation beyond the text itself occurs.

## Rationale

This invariant enables `ColorfulText` to be a transparent drop-in for `String` at all existing call sites. Code written as `"detail text".into()` or `String::from("x").into()` continues to compile and behave identically after a type migration from `Option<String>` to `Option<ColorfulText>`.

## Violations

- Setting `color` to any value in `From<String>` or `From<&str>` implementations
- Adding any escape codes to `text` in the conversion

## Verification

```rust
let ct = ColorfulText::from( "test".to_string() );
assert_eq!( ct.color, None );
assert_eq!( ct.render(), "test" );  // no escape codes
```
