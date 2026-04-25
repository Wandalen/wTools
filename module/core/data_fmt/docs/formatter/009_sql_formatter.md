# Formatter: SqlFormatter

### Scope

- **Purpose**: Render tabular data as SQL `INSERT` statements targeting a specific database dialect.
- **Responsibility**: Document the `SqlFormatter` struct — its `Format` trait implementation, `SqlVariant` enum selection, and 4 dialect variants.
- **In Scope**: Trait implementation, `SqlVariant` enum values, table name parameter, per-variant feature flags.
- **Out of Scope**: Variant output details (see `../variant/024_sql_ansi.md` through `027_sql_sqlite.md`), operation signatures (see `../api/004_formatters.md`).

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | `src/formatters/sql.rs` | `SqlFormatter` and `SqlVariant` implementation |
| doc | `../api/004_formatters.md` | Operation signatures |
| doc | `../trait/001_format.md` | `Format` trait contract |
| doc | `../variant/024_sql_ansi.md` | Variant: ansi |
| doc | `../variant/025_sql_postgresql.md` | Variant: postgresql |
| doc | `../variant/026_sql_mysql.md` | Variant: mysql |
| doc | `../variant/027_sql_sqlite.md` | Variant: sqlite |

### Trait

| Trait | Status | Note |
|-------|--------|------|
| `Format` | ✅ Active | Takes a `TableView`, returns formatted string or error |

### Input

| Input Type | Path | Via |
|------------|------|-----|
| `TableView` | Modern | `Format` trait |

### Variants

Selection mechanism: pass a `SqlVariant` enum value and a table name string to `SqlFormatter::with_variant(table_name, variant)`. Each variant is independently feature-gated.

| Variant | Selector | Feature Flag | Identifier quoting |
|---------|----------|--------------|-------------------|
| ansi | `SqlVariant::Ansi` | `sql_ansi` | double quotes |
| postgresql | `SqlVariant::PostgreSQL` | `sql_postgres` | double quotes |
| mysql | `SqlVariant::MySQL` | `sql_mysql` | backticks |
| sqlite | `SqlVariant::SQLite` | `sql_sqlite` | double quotes |

`SqlFormatter::new()` defaults to `SqlVariant::Ansi` with table name `"table"`.
