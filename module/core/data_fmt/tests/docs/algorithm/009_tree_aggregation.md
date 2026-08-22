# Algorithm: Tree Aggregation

### Scope

- **Purpose**: Drive test coverage for the recursive subtree-total and percentage computation used by `TreeFormatter::format_with_aggregation`.
- **Responsibility**: Documents test cases for the algorithm in `docs/algorithm/009_tree_aggregation.md`.
- **In Scope**: Directory subtree totals, leaf value preservation, recursive summation, percentage derivation, zero-`grand_total` behavior, root-suppression, `show_root`/`show_branches`/`max_depth` interaction (or lack thereof).
- **Out of Scope**: Column-width alignment (see `algorithm/003_tree_column_alignment.md`), no-memoization time-complexity characteristics (a performance property, not a behavioral one — see `benches/`), tree construction (see `builder/002_tree_builder.md`).

### Case Index

| ID | Name | Status |
|----|------|--------|
| AC-1 | directory nodes render aggregated subtree totals | ✅ |
| AC-2 | leaf values preserved alongside their own value | ✅ |
| AC-3 | aggregate values computed recursively through nested directories | ✅ |
| AC-4 | single-leaf tree renders without aggregation noise | ✅ |
| AC-5 | percentage computed as aggregate/grand_total*100, passed to render closures | ⏳ |
| AC-6 | zero `grand_total` produces NaN/inf percentage rather than panicking | ⏳ |
| AC-7 | root directory's own line is never rendered; its aggregate is never computed | ⏳ |
| AC-8 | `show_root` has no effect on aggregated output | ⏳ |
| AC-9 | `show_branches` has no effect on aggregated output | ⏳ |
| AC-10 | `max_depth` cuts off at `depth >= max_depth`, one level earlier than `format_aligned` | ⏳ |

---

### AC-1: directory nodes render aggregated subtree totals

- **Given:** A tree with two leaves under a `src` directory (`main.rs` = 150, `lib.rs` = 300) and a `grand_total` of 450.
- **When:** `format_with_aggregation` renders the tree.
- **Then:** The output contains the directory's summed total (450) alongside both leaf values (150, 300).
- **Note:** Covered by VT-1 (`variant_014_vt_01_directory_aggregated_totals`) in `tests/variant_014_tree_aggregated_test.rs`.

---

### AC-2: leaf values preserved alongside their own value

- **Given:** A single-leaf tree (`test.rs` = 50) with `grand_total` = 50.
- **When:** `format_with_aggregation` renders the tree.
- **Then:** The leaf's own value (50) appears in the output, unmodified by aggregation.
- **Note:** Covered by VT-2 (`variant_014_vt_02_leaf_values_preserved`).

---

### AC-3: aggregate values computed recursively through nested directories

- **Given:** A tree with two sibling directories (`src/main.rs` = 100, `tests/test.rs` = 50), `grand_total` = 150.
- **When:** `format_with_aggregation` renders the tree.
- **Then:** Each directory's own subtotal (100 for `src`, 50 for `tests`) appears in the output, each computed independently via its own `calculate_aggregate` recursion.
- **Note:** Covered by VT-3 (`variant_014_vt_03_recursive_aggregation`). The root total is relied upon implicitly (not asserted) to never appear — see AC-7 for the gap in direct coverage of that property.

---

### AC-4: single-leaf tree renders without aggregation noise

- **Given:** A tree with exactly one leaf (`file.rs` = 42) directly under the root, `grand_total` = 42.
- **When:** `format_with_aggregation` renders the tree.
- **Then:** Both the leaf's value (42) and name (`file.rs`) appear in the output; no spurious directory line is rendered for the root.
- **Note:** Covered by VT-4 (`variant_014_vt_04_single_leaf`).

---

### AC-5: percentage computed as aggregate/grand_total*100, passed to render closures

- **Given:** Any tree with a nonzero `grand_total` and at least one leaf or directory node.
- **When:** `format_with_aggregation` renders the tree, with a `render_file`/`render_directory` closure that surfaces the `pct` argument (e.g. formats it into the output string).
- **Then:** The rendered percentage equals `100.0 * node_total / grand_total` for each node, computed via `convert_to_f64`.
- **Note:** **Gap** — VT-1..VT-4's render closures all discard the `_pct` parameter (`| v, _total, _pct | format!( "{v}" )`), so no existing test asserts percentage correctness at all, only that raw totals/values are substring-present in the output.

---

### AC-6: zero `grand_total` produces NaN/inf percentage rather than panicking

- **Given:** A tree with at least one leaf, `grand_total` = 0 (or a type whose `convert_to_f64` maps to `0.0`).
- **When:** `format_with_aggregation` renders the tree, with a render closure that surfaces `pct`.
- **Then:** No panic occurs; percentages are IEEE-754 `NaN` (`0.0/0.0`) or `inf` (nonzero/0.0), propagated verbatim per `docs/algorithm/009_tree_aggregation.md § Key Properties : Zero-grand_total is unguarded`.
- **Note:** **Gap** — no existing test passes a zero `grand_total`.

---

### AC-7: root directory's own line is never rendered; its aggregate is never computed

- **Given:** Any tree with at least one child under the root.
- **When:** `format_with_aggregation` renders the tree.
- **Then:** No directory line is emitted for the root itself (the `is_root` guard short-circuits before `calculate_aggregate` is invoked for the root), regardless of `show_root`.
- **Note:** **Gap** — relied upon implicitly by every VT test (none show a root percentage/total line), but no test explicitly asserts the root's own line is absent or that `calculate_aggregate` is never invoked on it.

---

### AC-8: `show_root` has no effect on aggregated output

- **Given:** Two otherwise-identical `TreeConfig`s differing only in `show_root` (`true` vs `false`), used to construct the `TreeFormatter`.
- **When:** `format_with_aggregation` renders the same tree under each config.
- **Then:** The two outputs are byte-identical — `format_with_aggregation` never reads `self.config.show_root`, unlike `format_aligned`.
- **Note:** **Gap** — no existing test varies `show_root` while calling `format_with_aggregation`.

---

### AC-9: `show_branches` has no effect on aggregated output

- **Given:** Two otherwise-identical `TreeConfig`s differing only in `show_branches` (`true` vs `false`).
- **When:** `format_with_aggregation` renders the same multi-level tree under each config.
- **Then:** The two outputs are byte-identical — children are unconditionally recursed into regardless of this flag.
- **Note:** **Gap** — no existing test varies `show_branches` while calling `format_with_aggregation`.

---

### AC-10: `max_depth` cuts off at `depth >= max_depth`, one level earlier than `format_aligned`

- **Given:** A tree at least 2 levels deep, with `max_depth` set to a value that would include the deepest level under `format_aligned`'s `depth > max_depth` rule.
- **When:** `format_with_aggregation` renders the tree with that `max_depth`.
- **Then:** The deepest level is excluded — `format_with_aggregation` cuts off at `depth >= max_depth`, rendering one fewer level than `format_aligned` would for the same `max_depth` value.
- **Note:** **Gap** — no existing test exercises `max_depth` in combination with `format_with_aggregation`.

---

### Sources

| File | Relationship |
|------|-------------|
| [`docs/algorithm/009_tree_aggregation.md`](../../../docs/algorithm/009_tree_aggregation.md) | Source algorithm spec — recursive summation, percentage formula, root suppression, `show_root`/`show_branches`/`max_depth` interaction |

### Tests

| File | Relationship |
|------|-------------|
| [`tests/variant_014_tree_aggregated_test.rs`](../../variant_014_tree_aggregated_test.rs) | VT-1..VT-4: directory totals, leaf preservation, recursive aggregation, single-leaf (AC-1..AC-4 only) |
