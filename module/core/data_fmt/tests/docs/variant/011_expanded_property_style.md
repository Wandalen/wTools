# Variant: Expanded Property

### Scope

- **Purpose**: Drive test coverage for the property-style expanded output variant.
- **Responsibility**: Documents test cases for the property_style variant in `docs/variant/011_expanded_property_style.md`.
- **In Scope**: Colon-separated key-value pairs, no record headers, compact vertical layout.
- **Out of Scope**: Formatter internals (see `../formatter/`), attribute schema (see `../data_structure/`).

### Case Index

| ID | Name | Status |
|----|------|--------|
| VT-1 | vertical layout with colon separator | ⏳ |
| VT-2 | no record header line | ⏳ |
| VT-3 | multiple records separated by blank line | ⏳ |
| VT-4 | empty table produces no output | ⏳ |

---

### VT-1: vertical layout with colon separator

- **Given:** A `TableView` with headers `["Name", "Age"]` and one row `["Alice", "30"]`.
- **When:** Formatted with `ExpandedConfig::property_style()`.
- **Then:** Each field is rendered as `label: value` with a colon separator; no pipe characters appear.

---

### VT-2: no record header line

- **Given:** A `TableView` with headers `["key", "val"]` and one row `["a", "b"]`.
- **When:** Formatted with `ExpandedConfig::property_style()`.
- **Then:** No dash-line record header appears before the fields; the record starts directly with the first field.

---

### VT-3: multiple records separated by blank line

- **Given:** A `TableView` with headers `["Name"]` and rows `[["Alice"], ["Bob"]]`.
- **When:** Formatted with `ExpandedConfig::property_style()`.
- **Then:** A blank line or minimal separator appears between the two records; each record contains one field line.

---

### VT-4: empty table produces no output

- **Given:** A `TableView` with headers `["Col"]` and zero data rows.
- **When:** Formatted with `ExpandedConfig::property_style()`.
- **Then:** Output is empty; no record blocks or field lines appear.

---

### Sources

| File | Relationship |
|------|-------------|
| [`docs/variant/011_expanded_property_style.md`](../../../docs/variant/011_expanded_property_style.md) | Source variant doc — property_style attributes and example |

### Tests

| File | Relationship |
|------|-------------|
| [`tests/expanded_behavior.rs`](../../expanded_behavior.rs) | Expanded formatter test implementation |
