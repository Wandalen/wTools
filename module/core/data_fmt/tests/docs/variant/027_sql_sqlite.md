# Variant: SQL SQLite

### Scope

- **Purpose**: Drive test coverage for the SQLite SQL INSERT statement output variant.
- **Responsibility**: Documents test cases for the SQLite SQL variant in `docs/variant/027_sql_sqlite.md`.
- **In Scope**: SQLite-specific quoting, valid INSERT syntax, dialect differences from MySQL/PostgreSQL.
- **Out of Scope**: Formatter internals (see `../formatter/`), attribute schema (see `../data_structure/`).

### Case Index

| ID | Name | Status |
|----|------|--------|
| VT-1 | output uses SQLite quoting conventions | ⏳ |
| VT-2 | valid SQLite INSERT syntax | ⏳ |
| VT-3 | string values are single-quote escaped | ⏳ |
| VT-4 | empty table produces no INSERT statements | ⏳ |

---

### VT-1: output uses SQLite quoting conventions

- **Given:** A `TableView` with headers `["Name", "Age"]` and one row.
- **When:** Formatted with `SqlFormatter` using `SqlVariant::SQLite`.
- **Then:** Identifiers use SQLite-appropriate quoting; the quoting style may differ from MySQL (backticks) and PostgreSQL (double quotes).

---

### VT-2: valid SQLite INSERT syntax

- **Given:** A `TableView` with headers `["A", "B"]` and rows `[["1", "2"]]`.
- **When:** Formatted with `SqlVariant::SQLite`.
- **Then:** Output is a valid SQLite INSERT statement; executable against a SQLite database without syntax errors.

---

### VT-3: string values are single-quote escaped

- **Given:** A `TableView` with a cell containing `"it's"` (embedded single quote).
- **When:** Formatted with `SqlVariant::SQLite`.
- **Then:** The single quote is escaped (doubled to `''`); the INSERT statement is syntactically valid.

---

### VT-4: empty table produces no INSERT statements

- **Given:** A `TableView` with headers `["Col"]` and zero data rows.
- **When:** Formatted with `SqlVariant::SQLite`.
- **Then:** Output is empty or contains only a comment; no INSERT statements generated.

---

### Sources

| File | Relationship |
|------|-------------|
| [`docs/variant/027_sql_sqlite.md`](../../../docs/variant/027_sql_sqlite.md) | Source variant doc — SQL SQLite attributes and example |

### Tests

| File | Relationship |
|------|-------------|
| [`tests/sql.rs`](../../sql.rs) | SQL formatter test implementation |
