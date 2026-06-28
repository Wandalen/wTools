# Variant: Tree Aggregated

### Scope

- **Purpose**: Drive test coverage for the aggregated tree output variant with computed directory totals.
- **Responsibility**: Documents test cases for the aggregated variant in `docs/variant/014_tree_aggregated.md`.
- **In Scope**: Aggregated directory totals, user-defined aggregation function, leaf values preserved, hierarchy rendering.
- **Out of Scope**: Formatter internals (see `../formatter/`), attribute schema (see `../data_structure/`).

### Case Index

| ID | Name | Status |
|----|------|--------|
| VT-1 | directory nodes show aggregated totals | ✅ |
| VT-2 | leaf values preserved alongside aggregation | ✅ |
| VT-3 | aggregation function applied recursively | ✅ |
| VT-4 | single-leaf tree shows leaf value without aggregation | ✅ |

---

### VT-1: directory nodes show aggregated totals

- **Given:** A `TreeNode<i64>` with directory `"src"` containing leaves `"main.rs" → 150` and `"lib.rs" → 300`.
- **When:** Formatted with `TreeFormatter::format_with_aggregation()` using a sum function.
- **Then:** The `"src"` directory line displays the aggregated total `450`; individual leaf values are also visible.

---

### VT-2: leaf values preserved alongside aggregation

- **Given:** A tree with leaf `"test.rs" → 50`.
- **When:** Formatted with aggregation.
- **Then:** The leaf line still shows `50` as its value; aggregation adds data to directory nodes without removing leaf values.

---

### VT-3: aggregation function applied recursively

- **Given:** A 3-level tree: root → `"src"` → `"main.rs" → 100`; root → `"tests"` → `"test.rs" → 50`.
- **When:** Formatted with sum aggregation.
- **Then:** `"src"` shows `100`, `"tests"` shows `50`, and the root shows the total `150`; aggregation propagates from leaves up through all directory levels.

---

### VT-4: single-leaf tree shows leaf value without aggregation

- **Given:** A tree with only root and one leaf `"file.rs" → 42`.
- **When:** Formatted with aggregation.
- **Then:** The leaf shows `42`; the root shows `42` (aggregated from its single child); output is minimal and correct.

---

### Sources

| File | Relationship |
|------|-------------|
| [`docs/variant/014_tree_aggregated.md`](../../../docs/variant/014_tree_aggregated.md) | Source variant doc — aggregated tree attributes and example |

### Tests

| File | Relationship |
|------|-------------|
| [`tests/aligned_tree_basic.rs`](../../aligned_tree_basic.rs) | Tree alignment and rendering tests |
| [`tests/variant_014_tree_aggregated_test.rs`](../../variant_014_tree_aggregated_test.rs) | Spec tests for VT-1..VT-4 — tree_aggregated variant |
