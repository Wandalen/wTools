# Variant: YAML Standard

### Scope

- **Purpose**: Drive test coverage for the YAML output variant.
- **Responsibility**: Documents test cases for the Standard YAML variant in `docs/variant/018_yaml_standard.md`.
- **In Scope**: Valid YAML output, indentation-based structure, serde_yaml_ng serialization, UTF-8 charset.
- **Out of Scope**: Formatter internals (see `../formatter/`), attribute schema (see `../data_structure/`).

### Case Index

| ID | Name | Status |
|----|------|--------|
| VT-1 | output is valid parseable YAML | ✅ |
| VT-2 | indentation-based nesting structure | ✅ |
| VT-3 | header names used as keys | ✅ |
| VT-4 | empty table produces valid YAML | ✅ |

---

### VT-1: output is valid parseable YAML

- **Given:** A `TableView` with headers `["Name", "Age"]` and rows `[["Alice", "30"]]`.
- **When:** Formatted with `YamlFormatter`.
- **Then:** The output is valid YAML; parsing with a YAML parser succeeds; row data is accessible by header-name keys.

---

### VT-2: indentation-based nesting structure

- **Given:** A `TableView` with headers `["key", "val"]` and one row.
- **When:** Formatted with `YamlFormatter`.
- **Then:** The output uses YAML indentation (spaces) to denote structure; no braces or brackets for nesting; rows are represented as YAML list items.

---

### VT-3: header names used as keys

- **Given:** A `TableView` with headers `["Name", "City"]` and one row `["Alice", "NYC"]`.
- **When:** Formatted with `YamlFormatter`.
- **Then:** Each row item contains `Name: Alice` and `City: NYC`; header names become YAML mapping keys.

---

### VT-4: empty table produces valid YAML

- **Given:** A `TableView` with headers `["Col"]` and zero data rows.
- **When:** Formatted with `YamlFormatter`.
- **Then:** Output is valid YAML representing an empty list or structure; parseable without error.

---

### Sources

| File | Relationship |
|------|-------------|
| [`docs/variant/018_yaml_standard.md`](../../../docs/variant/018_yaml_standard.md) | Source variant doc — YAML Standard attributes and example |

### Tests

| File | Relationship |
|------|-------------|
| [`tests/yaml.rs`](../../yaml.rs) | YAML formatter test implementation |
| [`tests/variant_018_yaml_test.rs`](../../variant_018_yaml_test.rs) | Spec tests for VT-1..VT-4 YAML variant |
