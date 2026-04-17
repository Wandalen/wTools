# API: ColorfulText

## Struct

```rust
pub struct ColorfulText {
  pub text  : String,
  pub color : Option< String >,
}
```

Derives: `Debug`, `Clone`, `PartialEq`, `Eq`, `Default`
Optional: `Serialize`, `Deserialize` (feature `serde_support`)

## Methods

| Method | Signature | Behavior |
|--------|-----------|----------|
| `with_color` | `fn with_color( self, ansi : impl Into< String > ) -> Self` | Attach ANSI prefix; returns `self` for builder chaining |
| `render` | `fn render( &self ) -> String` | `color + text + "\x1b[0m"` when colored; `text.clone()` otherwise |
| `is_colored` | `fn is_colored( &self ) -> bool` | Returns `self.color.is_some()` |
| `is_empty` | `fn is_empty( &self ) -> bool` | Returns `self.text.is_empty()` |

## Trait Implementations

| Trait | Behavior |
|-------|----------|
| `From< String >` | `color: None`, `text: value` |
| `From< &str >` | `color: None`, `text: value.to_owned()` |
| `From< ColorfulText > for String` | Delegates to `.render()` |
| `Display` | Delegates to `.render()` |
| `Default` | `text: ""`, `color: None` |

## Features

| Feature | Effect |
|---------|--------|
| `enabled` | Compile `ColorfulText` |
| `serde_support` | Add `Serialize` + `Deserialize` derives |
