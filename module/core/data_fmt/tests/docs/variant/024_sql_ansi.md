# Variant: SQL ANSI

### Scope

- **Purpose**: Drive test coverage for the ANSI SQL INSERT statement output variant.
- **Responsibility**: Documents test cases for the ANSI SQL variant in `docs/variant/024_sql_ansi.md`.
- **In Scope**: Valid INSERT syntax, ANSI standard quoting, comma-separated values, table name handling.
- **Out of Scope**: Formatter internals (see `../formatter/`), attribute schema (see `../data_structure/`).

### Case Index

| ID | Name | Status |
|----|------|--------|
| VT-1 | output is valid ANSI SQL INSERT statement | ✅ |
| VT-2 | values are single-quote escaped | ✅ |
| VT-3 | column names listed in INSERT | ✅ |
| VT-4 | empty table produces no INSERT statements | ✅ |

---

### VT-1: output is valid ANSI SQL INSERT statement

- **Given:** A `TableView` with headers `["Name", "Age"]` and one row `["Alice", "30"]`.
- **When:** Formatted with `SqlFormatter` using `SqlVariant::Ansi`.
- **Then:** Output contains `INSERT INTO` with column names and `VALUES` clause; syntax conforms to ANSI SQL standard.

---

### VT-2: values are single-quote escaped

- **Given:** A `TableView` with a cell containing `"O'Brien"` (embedded single quote).
- **When:** Formatted with `SqlVariant::Ansi`.
- **Then:** The single quote is escaped (doubled to `''`); the output remains valid SQL.

---

### VT-3: column names listed in INSERT

- **Given:** A `TableView` with headers `["name", "city"]` and one row.
- **When:** Formatted with `SqlVariant::Ansi`.
- **Then:** The INSERT statement includes `(name, city)` or equivalent column list before `VALUES`.

---

### VT-4: empty table produces no INSERT statements

- **Given:** A `TableView` with headers `["Col"]` and zero data rows.
- **When:** Formatted with `SqlVariant::Ansi`.
- **Then:** Output contains no INSERT statements; output is empty or contains only a comment.

---

### Sources

| File | Relationship |
|------|-------------|
| [`docs/variant/024_sql_ansi.md`](../../../docs/variant/024_sql_ansi.md) | Source variant doc — SQL ANSI attributes and example |

### Tests

| File | Relationship |
|------|-------------|
| [`tests/sql.rs`](../../sql.rs) | SQL formatter test implementation |
| [`tests/variant_024_sql_ansi_test.rs`](../../variant_024_sql_ansi_test.rs) | Spec tests for VT-1..VT-4 SQL ANSI variant |
