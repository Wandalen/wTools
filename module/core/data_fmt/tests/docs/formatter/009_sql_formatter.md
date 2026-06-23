# Formatter: SqlFormatter

### Scope

- **Purpose**: Drive test coverage for the SqlFormatter output contract.
- **Responsibility**: Documents test cases for the `SqlFormatter` struct described in `docs/formatter/009_sql_formatter.md`.
- **In Scope**: SQL INSERT statement output, dialect-specific identifier quoting, table name parameter, Format trait dispatch, empty data handling, SQL injection safety.
- **Out of Scope**: Per-variant visual details (see `tests/docs/variant/`), database execution behavior.

### Case Index

| ID | Name | Status |
|----|------|--------|
| FM-39 | ansi variant produces double-quoted identifiers | ⏳ |
| FM-40 | mysql variant produces backtick-quoted identifiers | ⏳ |
| FM-41 | postgresql variant produces double-quoted identifiers | ⏳ |
| FM-42 | sqlite variant produces double-quoted identifiers | ⏳ |
| FM-43 | custom table name appears in INSERT statement | ⏳ |
| FM-44 | Format trait dispatch returns well-formed string | ⏳ |
| FM-45 | empty data produces no INSERT statements | ⏳ |
| FM-46 | values with single quotes are escaped | ⏳ |

---

### FM-39: ansi variant produces double-quoted identifiers

- **Given:** A `TableView` with headers `["name", "age"]` and one row `["Alice", "30"]`.
- **When:** `SqlFormatter::with_variant("data", SqlVariant::Ansi)` formats the view.
- **Then:** The output contains `INSERT INTO "data"` with double-quoted column names; values are single-quoted strings.

---

### FM-40: mysql variant produces backtick-quoted identifiers

- **Given:** A `TableView` with headers `["name"]` and one row `["Alice"]`.
- **When:** `SqlFormatter::with_variant("users", SqlVariant::MySQL)` formats the view.
- **Then:** The table name and column names are backtick-quoted (`` `users` ``, `` `name` ``); values are single-quoted.

---

### FM-41: postgresql variant produces double-quoted identifiers

- **Given:** A `TableView` with headers `["name"]` and one row `["Alice"]`.
- **When:** `SqlFormatter::with_variant("users", SqlVariant::PostgreSQL)` formats the view.
- **Then:** The table name and column names are double-quoted (`"users"`, `"name"`); values are single-quoted.

---

### FM-42: sqlite variant produces double-quoted identifiers

- **Given:** A `TableView` with headers `["name"]` and one row `["Alice"]`.
- **When:** `SqlFormatter::with_variant("users", SqlVariant::SQLite)` formats the view.
- **Then:** The table name and column names are double-quoted; values are single-quoted.

---

### FM-43: custom table name appears in INSERT statement

- **Given:** A `TableView` with headers `["k"]` and one row `["v"]`; table name `"my_schema.my_table"`.
- **When:** `SqlFormatter::with_variant("my_schema.my_table", SqlVariant::Ansi)` formats the view.
- **Then:** The output contains `INSERT INTO "my_schema.my_table"`; the provided table name is used verbatim inside quotes.

---

### FM-44: Format trait dispatch returns well-formed string

- **Given:** A `TableView` with headers `["a"]` and rows `[["1"]]`.
- **When:** The `Format::fmt` method is called on a `SqlFormatter` instance (ansi variant).
- **Then:** The return value is `Ok(String)` containing a valid SQL INSERT statement; no `FormatError` is returned.

---

### FM-45: empty data produces no INSERT statements

- **Given:** A `TableView` with headers `["col"]` and zero data rows.
- **When:** `SqlFormatter::new()` formats the view.
- **Then:** The output contains no `INSERT` statements; the result is empty or contains only comments.

---

### FM-46: values with single quotes are escaped

- **Given:** A `TableView` with headers `["text"]` and one row containing `"it's a test"`.
- **When:** `SqlFormatter::new()` formats the view.
- **Then:** The single quote in the value is escaped (doubled: `''`); the output is syntactically valid SQL.

---

### Sources

| File | Relationship |
|------|-------------|
| [`docs/formatter/009_sql_formatter.md`](../../../docs/formatter/009_sql_formatter.md) | Source formatter doc — trait, dialect enum, 4 SQL variants |

### Tests

| File | Relationship |
|------|-------------|
| [`tests/sql_tests.rs`](../../sql_tests.rs) | SqlFormatter test implementation |
