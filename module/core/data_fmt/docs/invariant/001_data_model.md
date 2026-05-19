# Invariant: Data Model

### Scope

- **Purpose**: Define the structural contracts and validation rules that `TreeNode`, `RowBuilder`, and `TableView` maintain across all operations.
- **Responsibility**: Documents data model invariants for tree nodes, row builders, and table views.
- **In Scope**: TreeNode design invariants, RowBuilder contracts, TableView interchange format, edge case contracts.
- **Out of Scope**: Rendering algorithms (see `algorithm/` docs) and ANSI/Unicode handling (see `invariant/002_ansi_unicode.md`).

### Sources

| File | Relationship |
|------|-------------|
| [`src/data.rs`](../../src/data.rs) | Core data type definitions (`TreeNode<T>`) |
| [`src/builder.rs`](../../src/builder.rs) | `TreeBuilder<T>` — path-based tree construction |
| [`src/table_tree.rs`](../../src/table_tree.rs) | `RowBuilder` — row-length enforcement (`validate_row_length`) |

### Tests

| File | Relationship |
|------|-------------|
| [`tests/data.rs`](../../tests/data.rs) | Data type invariant tests |
| [`tests/builder.rs`](../../tests/builder.rs) | Builder contract tests |

### Invariant Statement

#### TreeNode Design

Directory nodes hold no data payload and may have any number of children. Leaf nodes hold a typed data payload and typically have no children. Hierarchical trees allow unlimited nesting depth; any node may have zero or more children.

Table-shaped trees encode tabular data as a specific tree structure. The root holds row nodes as direct children. Each row node holds column-named children that carry cell values as data. The table validation invariant requires that all row nodes have identical child structure — same column names in the same order.

#### RowBuilder

Every row added via any `add_row*` method must have length exactly equal to `headers.len()`. The builder enforces this at construction time so that downstream formatters never encounter ragged rows.

The parallel vectors invariant holds throughout the builder's lifetime: `rows` and `row_details` are always the same length. Every internal row insertion pushes to both vectors simultaneously. Rows added without an explicit detail receive no annotation.

#### TableView

`TableView` is the format-agnostic data structure consumed by all formatters. It holds extracted headers and rows as plain string vectors, decoupled from `TreeNode` internals. The `TableShapedView` trait provides generic extraction from any tree node whose data type supports display formatting, converting data values to strings automatically.

#### Edge Case Contracts

- **EC-1**: Empty tables return empty string in all formats
- **EC-2**: Empty trees return empty string when formatted
- **EC-3**: Single-row tables display correctly in all formats
- **EC-4**: Generic display-capable data types work with the tabular extraction trait

### Enforcement Mechanism

Row length validation is enforced by `RowBuilder` at the point of insertion — the builder panics immediately if row length does not match `headers.len()`. The parallel vectors invariant is maintained by the single internal insertion method that always updates both vectors atomically. `TableView` is constructed only via `build_view()` or `TableShapedView::to_table_view()`, both of which produce a well-formed state.

### Violation Consequences

Violating the row length invariant causes an immediate panic at insertion time. Violating the parallel vectors invariant would produce mismatched row/detail rendering — detail annotations would appear on incorrect rows. Violating the table-shaped tree column structure invariant causes formatters to produce incorrect or misaligned column output.
