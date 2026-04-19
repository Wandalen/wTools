# Invariant: Data Model

### Scope

- **Purpose**: Define the structural contracts and validation rules that `TreeNode< T >`, `RowBuilder`, and `TableView` maintain across all operations.
- **Responsibility**: Documents data model invariants for tree nodes, row builders, and table views.
- **In Scope**: TreeNode design invariants, RowBuilder contracts, TableView interchange format, edge case contracts.
- **Out of Scope**: Rendering algorithms (see `algorithm/` docs) and ANSI/Unicode handling (see `invariant/002_ansi_unicode.md`).

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | `src/data.rs` | Core data type definitions |
| source | `src/builder.rs` | Builder contract enforcement |
| test | `tests/data.rs` | Data type invariant tests |
| test | `tests/builder.rs` | Builder contract tests |

### Invariant Statement

#### TreeNode Design

Directory nodes hold `data = None` and may have any number of children. File (leaf) nodes hold `data = Some(T)` and typically have no children. Hierarchical trees allow unlimited nesting depth; any node may have zero or more children; the tree is generic over `T` with minimal trait bounds.

Table-shaped trees encode tabular data as a specific tree structure. The root holds row nodes as direct children. Each row node holds column-named children that carry cell values as `data`. The table validation invariant requires that all row nodes have identical child structure — same column names in the same order.

#### RowBuilder

Every row added via any `add_row*` method must have length exactly equal to `headers.len()`. The builder enforces this at construction time so that downstream formatters never encounter ragged rows.

The parallel vectors invariant holds throughout the builder's lifetime: `rows` and `row_details` are always the same length. Every internal row insertion pushes to both vectors simultaneously. Rows added without an explicit detail receive `None`.

#### TableView

`TableView` is the format-agnostic data structure consumed by all formatters. It holds extracted headers and rows as plain string vectors, decoupled from `TreeNode` internals. The `TableShapedView` trait provides generic extraction from any `TreeNode< T >` where `T : Display`, converting `T` values to `String` via the `Display` trait.

### Enforcement Mechanism

Row length validation is enforced by `RowBuilder` at the point of insertion — the builder panics immediately if row length does not match `headers.len()`. The parallel vectors invariant is maintained by the single internal insertion method that always updates both vectors atomically. `TableView` is constructed only via `build_view()` or `TableShapedView::to_table_view()`, both of which produce a well-formed state.

### Violation Consequences

- **EC-1**: Empty tables return empty string in all formats
- **EC-2**: Empty trees return empty string when formatted
- **EC-3**: Single-row tables display correctly in all formats
- **EC-4**: Generic `TableShapedView` works with any `T : Display`

Violating the row length invariant causes an immediate panic at insertion time. Violating the parallel vectors invariant would produce mismatched row/detail rendering — detail annotations would appear on incorrect rows. Violating the table-shaped tree column structure invariant causes formatters to produce incorrect or misaligned column output.
