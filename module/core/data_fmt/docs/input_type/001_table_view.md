# Input Type: TableView

### Scope

- **Purpose**: Document the `TableView` Rust struct as the canonical tabular input type for the `Format` trait.
- **Responsibility**: Document TableView struct definition, components, construction, and formatter coverage.
- **In Scope**: Struct fields, TableMetadata, construction patterns, Format trait consumption, and backward compatibility.
- **Out of Scope**: Conceptual shape (see `../input_model/`), formatter behavior (see `../feature/`).

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | `src/data.rs` | TableView struct definition |
| test | `tests/data.rs` | TableView tests |
| doc | `../input_model/001_tabular.md` | Conceptual data shape |
| doc | `../api/001_data_types.md` | Public API surface |

### Type Definition

```rust
pub struct TableView
{
  pub metadata : TableMetadata,
  pub rows : Vec< Vec< String > >,
  pub row_details : Vec< Option< DecoratedText > >,
}
```

### Components

| Field | Type | Role |
|-------|------|------|
| `metadata` | `TableMetadata` | Column names and data types |
| `rows` | `Vec<Vec<String>>` | Cell data, one inner vec per row |
| `row_details` | `Vec<Option<DecoratedText>>` | Optional per-row annotation line (parallel to `rows`) |

`TableMetadata` contains:
- `column_names: Vec<String>` — header labels
- `column_types: Vec<DataType>` — per-column semantic types (String, Integer, Boolean, Path)

### Construction

```rust
// Via RowBuilder (preferred)
let view = RowBuilder::new( vec![ "Name".into(), "Age".into() ] )
  .add_row( vec![ "Alice".into(), "30".into() ] )
  .build_view();

// Direct construction
let view = TableView::new(
  TableMetadata::new( vec![ "Name".into(), "Age".into() ] ),
  vec![ vec![ "Alice".into(), "30".into() ] ],
);
```

### Trait

Consumed by `Format` trait:

```rust
pub trait Format
{
  fn format( &self, data : &TableView ) -> Result< String, FormatError >;
}
```

### Formatter Coverage

| Formatter | Implements `Format` |
|-----------|:-------------------:|
| `TableFormatter` | yes |
| `LogfmtFormatter` | yes |
| `HtmlFormatter` | yes |
| `SqlFormatter` | yes |
| `JsonFormatter` | yes |
| `YamlFormatter` | yes |
| `TomlFormatter` | yes |
| `TextFormatter` | yes |
| `ExpandedFormatter` | **no** |
| `TreeFormatter` | **no** |

8 of 10 formatters accept `TableView` via `Format`.

### Backward Compatibility

`TableView::to_tree_node()` converts back to `TreeNode<Vec<String>>` for formatters that don't implement `Format`.
