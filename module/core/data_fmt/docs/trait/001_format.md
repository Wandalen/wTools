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

### Signature

`Format` is a trait with one required method. It accepts an immutable reference to a `TableView` and returns either a formatted string or a `FormatError`.

### Error Type

`FormatError` has three variants. `Serialization( String )` is available only with the `serde_support` feature and is emitted by JSON/YAML/TOML formatters on serialization failure. `InvalidData( String )` signals structurally invalid input for the requested format. `UnsupportedOperation( String )` signals an operation not supported by the given formatter configuration. Without `serde_support`, only `InvalidData` and `UnsupportedOperation` are present.

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

### Coverage Gaps

| Formatter | Reason | Alternative |
|-----------|--------|-------------|
| `ExpandedFormatter` | Uses deprecated `TableShapedFormatter` only | `TableShapedFormatter::format( &TreeNode<String> )` — deprecated |
| `TreeFormatter` | Requires generic `T` + render closure | Direct methods: `format()`, `format_aligned()`, `format_with_aggregation()` |

### Input Type

`TableView` — see `input_type/table_view.md`.

### Usage

Any `Format`-implementing formatter accepts a `TableView` produced by `RowBuilder::build_view()`. The caller invokes the formatter's `format` method, which returns a `Result<String, FormatError>`.
