# Pattern: Three-Layer Architecture

### Scope

- **Purpose**: Drive test coverage for the three-layer architecture pattern.
- **Responsibility**: Documents test cases for the architectural layering in `docs/pattern/001_three_layer_architecture.md`.
- **In Scope**: Layer 1 data types existence, Layer 2 builder and trait presence, Layer 3 formatter consumption, no backward dependencies from formatters to data internals.
- **Out of Scope**: Per-formatter output correctness (see `../formatter/`), API signatures (see `../api/`).

### Case Index

| ID | Name | Status |
|----|------|--------|
| PT-1 | Layer 1 data types are public | ✅ |
| PT-2 | Layer 2 builders produce Layer 1 types | ✅ |
| PT-3 | Layer 3 formatters consume Layer 1/2 output | ✅ |

---

### PT-1: Layer 1 data types are public

- **Given:** The `data_fmt` crate public API.
- **When:** Importing `TreeNode` and `TableView` from the crate root.
- **Then:** Both types are publicly accessible without feature gates; `TreeNode` has `name`, `data`, and `children` fields; `TableView` has `metadata`, `rows`, and `row_details` fields.

---

### PT-2: Layer 2 builders produce Layer 1 types

- **Given:** `RowBuilder` constructed with headers `["A", "B"]` and one row added.
- **When:** `build_view()` is called on the builder.
- **Then:** The result is a `TableView` (Layer 1 type); `TreeBuilder` similarly produces a `TreeNode` via `build()`; builders do not expose formatter-specific details.

---

### PT-3: Layer 3 formatters consume Layer 1/2 output

- **Given:** A `TableView` produced by `RowBuilder::build_view()`.
- **When:** Passed to `TableFormatter`, `JsonFormatter`, and `ExpandedFormatter` via the `Format` trait.
- **Then:** All three formatters accept the same `TableView` without modification; `TreeFormatter` separately accepts `TreeNode` directly; no formatter requires builder-internal types.

---

### Sources

| File | Relationship |
|------|-------------|
| [`docs/pattern/001_three_layer_architecture.md`](../../../docs/pattern/001_three_layer_architecture.md) | Source pattern doc — layer decomposition, module file structure |
