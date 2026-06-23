# Variant: Expanded Postgres

### Scope

- **Purpose**: Drive test coverage for the PostgreSQL-style expanded output variant.
- **Responsibility**: Documents test cases for the postgres_style variant in `docs/variant/010_expanded_postgres_style.md`.
- **In Scope**: Vertical record layout, pipe field separator, dash record dividers, one-row-per-field rendering.
- **Out of Scope**: Formatter internals (see `../formatter/`), attribute schema (see `../data_structure/`).

### Case Index

| ID | Name | Status |
|----|------|--------|
| VT-1 | vertical record layout with one field per line | ⏳ |
| VT-2 | field label and value separated by pipe | ⏳ |
| VT-3 | record separator between multiple records | ⏳ |
| VT-4 | empty table produces no records | ⏳ |

---

### VT-1: vertical record layout with one field per line

- **Given:** A `TableView` with headers `["Name", "Age"]` and one row `["Alice", "30"]`.
- **When:** Formatted with `ExpandedConfig::postgres_style()`.
- **Then:** Output shows field names and values vertically (one field per line), not in columnar layout; `"Name"` and `"Age"` each appear as labels on separate lines.

---

### VT-2: field label and value separated by pipe

- **Given:** A `TableView` with headers `["key", "val"]` and one row `["a", "b"]`.
- **When:** Formatted with `ExpandedConfig::postgres_style()`.
- **Then:** Each field line contains a `|` separator between the label and value; labels are left-aligned before the pipe.

---

### VT-3: record separator between multiple records

- **Given:** A `TableView` with headers `["Name"]` and rows `[["Alice"], ["Bob"]]`.
- **When:** Formatted with `ExpandedConfig::postgres_style()`.
- **Then:** A dash-based separator line appears between the two records; each record's fields are grouped together.

---

### VT-4: empty table produces no records

- **Given:** A `TableView` with headers `["Col"]` and zero data rows.
- **When:** Formatted with `ExpandedConfig::postgres_style()`.
- **Then:** Output is empty or contains only a header marker; no record blocks appear.

---

### Sources

| File | Relationship |
|------|-------------|
| [`docs/variant/010_expanded_postgres_style.md`](../../../docs/variant/010_expanded_postgres_style.md) | Source variant doc — postgres_style attributes and example |

### Tests

| File | Relationship |
|------|-------------|
| [`tests/expanded_behavior.rs`](../../expanded_behavior.rs) | Expanded formatter test implementation |
