# Trait: Format

### Scope

- **Purpose**: Document the Format interface contract, implementors, and coverage.
- **Responsibility**: Define the unified formatting trait, its error type, and implementor matrix.
- **In Scope**: Trait definition, error variants, implementor table, non-implementors, usage example.
- **Out of Scope**: Formatter implementation (see `../feature/`), variant output (see `../variant/`).

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | `src/formatters/format_trait.rs` | Format trait definition |
| test | `tests/unified_format_trait.rs` | Format trait tests |
| doc | `../feature/003_unified_format_interface.md` | Feature-level documentation |

### Definition

```rust
pub trait Format
{
  fn format( &self, data : &TableView ) -> Result< String, FormatError >;
}
```

### Error Type

```rust
pub enum FormatError
{
  Serialization( String ),      // serde_support only
  InvalidData( String ),
  UnsupportedOperation( String ),
}
```

### Implementors

| Formatter | Feature Gate | Dependencies |
|-----------|-------------|--------------|
| `TableFormatter` | `table_*` (9 flags) | none |
| `LogfmtFormatter` | `format_logfmt` | none |
| `HtmlFormatter` | `html_*` (4 flags) | none |
| `SqlFormatter` | `sql_*` (4 flags) | none |
| `JsonFormatter` | `format_json` | serde, serde_json |
| `YamlFormatter` | `format_yaml` | serde, serde_yaml |
| `TomlFormatter` | `format_toml` | serde, toml |
| `TextFormatter` | `format_text` | none |

### Not Implemented By

| Formatter | Reason | Alternative |
|-----------|--------|-------------|
| `ExpandedFormatter` | Uses `TableShapedFormatter` only | `TableShapedFormatter::format( &TreeNode<String> )` |
| `TreeFormatter` | Requires generic `T` + render closure | Direct methods: `format()`, `format_aligned()`, `format_with_aggregation()` |

### Input Type

`TableView` — see `input_type/table_view.md`.

### Usage

```rust
use data_fmt::{ RowBuilder, Format };

let view = RowBuilder::new( vec![ "Name".into() ] )
  .add_row( vec![ "Alice".into() ] )
  .build_view();

// Any Format implementor works
let formatter = TableFormatter::new();
let output = Format::format( &formatter, &view )?;
```
