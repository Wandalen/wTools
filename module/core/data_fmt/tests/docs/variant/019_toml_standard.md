# Variant: TOML Standard

### Scope

- **Purpose**: Drive test coverage for the TOML output variant.
- **Responsibility**: Documents test cases for the Standard TOML variant in `docs/variant/019_toml_standard.md`.
- **In Scope**: Valid TOML output, array-of-tables notation, bracket headers, UTF-8 charset.
- **Out of Scope**: Formatter internals (see `../formatter/`), attribute schema (see `../data_structure/`).

### Case Index

| ID | Name | Status |
|----|------|--------|
| VT-1 | output is valid parseable TOML | ✅ |
| VT-2 | rows use array-of-tables notation | ✅ |
| VT-3 | header names become TOML keys | ✅ |
| VT-4 | empty table produces valid TOML | ✅ |

---

### VT-1: output is valid parseable TOML

- **Given:** A `TableView` with headers `["Name", "Age"]` and rows `[["Alice", "30"]]`.
- **When:** Formatted with `TomlFormatter`.
- **Then:** The output is valid TOML; parsing with a TOML parser succeeds; data content matches the input.

---

### VT-2: rows use array-of-tables notation

- **Given:** A `TableView` with headers `["key", "val"]` and rows `[["a", "1"], ["b", "2"]]`.
- **When:** Formatted with `TomlFormatter`.
- **Then:** Each row is represented as a TOML array-of-tables entry (using `[[rows]]` or similar bracket notation); rows are distinguishable.

---

### VT-3: header names become TOML keys

- **Given:** A `TableView` with headers `["Name", "City"]` and one row `["Alice", "NYC"]`.
- **When:** Formatted with `TomlFormatter`.
- **Then:** Each field uses the header name as a TOML key (e.g., `Name = "Alice"`); key names match the header strings.

---

### VT-4: empty table produces valid TOML

- **Given:** A `TableView` with headers `["Col"]` and zero data rows.
- **When:** Formatted with `TomlFormatter`.
- **Then:** Output is valid TOML representing an empty structure; parseable without error.

---

### Sources

| File | Relationship |
|------|-------------|
| [`docs/variant/019_toml_standard.md`](../../../docs/variant/019_toml_standard.md) | Source variant doc — TOML Standard attributes and example |

### Tests

| File | Relationship |
|------|-------------|
| [`tests/toml_fmt.rs`](../../toml_fmt.rs) | TOML formatter test implementation |
| [`tests/variant_019_toml_test.rs`](../../variant_019_toml_test.rs) | Spec tests for VT-1..VT-4 TOML variant |
