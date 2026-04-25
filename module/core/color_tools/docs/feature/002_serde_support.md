# Feature: serde Support

### Scope

- **Purpose**: Document the `serde_support` opt-in feature that adds `Serialize` and `Deserialize` to `DecoratedText`.
- **Responsibility**: Covers feature flag activation, serialized field format, and round-trip fidelity contract.
- **In Scope**: Feature flag, what gets serialized, JSON wire format, round-trip guarantee, and activation snippet.
- **Out of Scope**: serde version pinning (caller's Cargo.toml); ANSI-to-color-name translation during serialization (not provided); non-JSON formats (callers choose their serde backend).

### Design

`serde_support` is a compile-time opt-in. When enabled, `DecoratedText` gains `Serialize` and `Deserialize` via conditional derives:

```rust
#[ cfg_attr( feature = "serde_support", derive( serde::Serialize, serde::Deserialize ) ) ]
pub struct DecoratedText { ... }
```

**Activation:**

```toml
color_tools = { workspace = true, features = [ "enabled", "serde_support" ] }
```

| Aspect | Detail |
|--------|--------|
| Derives | `serde::Serialize`, `serde::Deserialize` (conditional on `serde_support` feature) |
| Serialized fields | `text: String` and `color: Option<String>` — both serialized as-is |
| JSON example (uncolored) | `{"text":"hello","color":null}` |
| JSON example (colored) | `{"text":"warn","color":"\u001b[33m"}` |
| Round-trip contract | `from_str(&to_string(&ct).unwrap()).unwrap() == ct` for any `DecoratedText` |
| ANSI bytes in JSON | Raw SGR bytes stored verbatim; no color-name encoding or escape translation |

**Round-trip constraint:** `text` and `color` serialize as plain JSON strings. A `color` of `"\x1b[33m"` appears as the Unicode escape `"\u001b[33m"` in JSON; the bytes round-trip correctly through any conforming serde backend. No semantic loss occurs.

**No translation:** serde support does not provide color-name encoding (`Color::Yellow`), CSS conversion, or any format other than raw field values. Callers needing non-ANSI outputs must translate the `color` field themselves using the public `color: Option<String>` field.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| doc | [DecoratedText](001_decorated_text.md) | Parent feature — base type this feature extends |
| doc | [DecoratedText Type](../api/001_decorated_text_type.md) | serde derives listed in the public API reference |
