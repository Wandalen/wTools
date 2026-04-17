# Feature: ColorfulText

## Purpose

Provide a typed wrapper for text that may optionally carry an ANSI color prefix, enabling per-instance terminal coloring without global configuration.

## Behavior

| Scenario | Behavior |
|----------|----------|
| `ColorfulText::from("text")` | `color: None`; `render()` returns raw text clone |
| `ColorfulText::from("text").with_color("\x1b[33m")` | `color: Some(...)`, `render()` returns `"\x1b[33mtext\x1b[0m"` |
| `ColorfulText::from("")` | `is_empty()` returns `true`; `render()` returns `""` |
| `ColorfulText::from("").with_color("\x1b[33m")` | `is_empty()` returns `true` (text is empty regardless of color) |

## Invariants

1. `From<String>` and `From<&str>` always set `color: None`
2. `.render()` appends `"\x1b[0m"` if and only if `color` is `Some`
3. `.is_empty()` tests `self.text.is_empty()`, not `self.render().is_empty()`
4. `Display` output always equals `.render()` output
5. `From<ColorfulText> for String` always delegates to `.render()`

## Integration with `tree_fmt`

`tree_fmt`'s `row_details: Vec<Option<ColorfulText>>` uses this type so per-row detail lines can carry independent ANSI color without affecting the table's `TableConfig`.

## Serde Support

When the `serde_support` feature is enabled, `ColorfulText` derives `Serialize` and `Deserialize`. Both `text` and `color` fields are serialized as-is.

```toml
color_tools = { workspace = true, features = [ "enabled", "serde_support" ] }
```
