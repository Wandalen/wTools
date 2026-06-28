# Variant: SQL PostgreSQL

### Scope

- **Purpose**: Drive test coverage for the PostgreSQL SQL INSERT statement output variant.
- **Responsibility**: Documents test cases for the PostgreSQL SQL variant in `docs/variant/025_sql_postgresql.md`.
- **In Scope**: PostgreSQL-specific quoting (double-quoted identifiers), valid INSERT syntax, dialect differences from ANSI.
- **Out of Scope**: Formatter internals (see `../formatter/`), attribute schema (see `../data_structure/`).

### Case Index

| ID | Name | Status |
|----|------|--------|
| VT-1 | output uses PostgreSQL quoting conventions | ✅ |
| VT-2 | column identifiers are double-quoted | ✅ |
| VT-3 | valid PostgreSQL INSERT syntax | ✅ |
| VT-4 | empty table produces no INSERT statements | ✅ |

---

### VT-1: output uses PostgreSQL quoting conventions

- **Given:** A `TableView` with headers `["Name", "Age"]` and one row.
- **When:** Formatted with `SqlFormatter` using `SqlVariant::PostgreSQL`.
- **Then:** Output uses PostgreSQL-specific quoting; identifiers may be double-quoted; differs from ANSI variant in quoting style.

---

### VT-2: column identifiers are double-quoted

- **Given:** A `TableView` with headers `["user name", "age"]` and one row.
- **When:** Formatted with `SqlVariant::PostgreSQL`.
- **Then:** Column names with spaces are double-quoted in the INSERT statement (e.g., `"user name"`).

---

### VT-3: valid PostgreSQL INSERT syntax

- **Given:** A `TableView` with headers `["A", "B"]` and rows `[["1", "2"], ["3", "4"]]`.
- **When:** Formatted with `SqlVariant::PostgreSQL`.
- **Then:** Output contains valid PostgreSQL INSERT statements; each row produces one VALUES clause.

---

### VT-4: empty table produces no INSERT statements

- **Given:** A `TableView` with headers `["Col"]` and zero data rows.
- **When:** Formatted with `SqlVariant::PostgreSQL`.
- **Then:** Output contains no INSERT statements; output is empty or contains only a comment.

---

### Sources

| File | Relationship |
|------|-------------|
| [`docs/variant/025_sql_postgresql.md`](../../../docs/variant/025_sql_postgresql.md) | Source variant doc — SQL PostgreSQL attributes and example |

### Tests

| File | Relationship |
|------|-------------|
| [`tests/sql.rs`](../../sql.rs) | SQL formatter test implementation |
| [`tests/variant_025_sql_postgresql_test.rs`](../../variant_025_sql_postgresql_test.rs) | Spec tests for VT-1..VT-4 — sql_postgresql variant |
