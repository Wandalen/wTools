# Variant: SQL MySQL

### Scope

- **Purpose**: Drive test coverage for the MySQL SQL INSERT statement output variant.
- **Responsibility**: Documents test cases for the MySQL SQL variant in `docs/variant/026_sql_mysql.md`.
- **In Scope**: MySQL-specific quoting (backtick identifiers), valid INSERT syntax, dialect differences.
- **Out of Scope**: Formatter internals (see `../formatter/`), attribute schema (see `../data_structure/`).

### Case Index

| ID | Name | Status |
|----|------|--------|
| VT-1 | output uses MySQL backtick quoting | ✅ |
| VT-2 | column identifiers are backtick-quoted | ✅ |
| VT-3 | valid MySQL INSERT syntax | ✅ |
| VT-4 | empty table produces no INSERT statements | ✅ |

---

### VT-1: output uses MySQL backtick quoting

- **Given:** A `TableView` with headers `["Name", "Age"]` and one row.
- **When:** Formatted with `SqlFormatter` using `SqlVariant::MySQL`.
- **Then:** Identifiers are quoted with backticks; differs from PostgreSQL (double quotes) and ANSI.

---

### VT-2: column identifiers are backtick-quoted

- **Given:** A `TableView` with headers `["user name", "age"]` and one row.
- **When:** Formatted with `SqlVariant::MySQL`.
- **Then:** Column names use backtick quoting (e.g., `` `user name` ``); the backtick is the MySQL identifier quote character.

---

### VT-3: valid MySQL INSERT syntax

- **Given:** A `TableView` with headers `["A", "B"]` and rows `[["1", "2"]]`.
- **When:** Formatted with `SqlVariant::MySQL`.
- **Then:** Output is a valid MySQL INSERT statement; column list and VALUES clause are correctly formed.

---

### VT-4: empty table produces no INSERT statements

- **Given:** A `TableView` with headers `["Col"]` and zero data rows.
- **When:** Formatted with `SqlVariant::MySQL`.
- **Then:** Output is empty or contains only a comment; no INSERT statements generated.

---

### Sources

| File | Relationship |
|------|-------------|
| [`docs/variant/026_sql_mysql.md`](../../../docs/variant/026_sql_mysql.md) | Source variant doc — SQL MySQL attributes and example |

### Tests

| File | Relationship |
|------|-------------|
| [`tests/sql.rs`](../../sql.rs) | SQL formatter test implementation |
| [`tests/variant_026_sql_mysql_test.rs`](../../variant_026_sql_mysql_test.rs) | Spec tests for VT-1..VT-4 — sql_mysql variant |
