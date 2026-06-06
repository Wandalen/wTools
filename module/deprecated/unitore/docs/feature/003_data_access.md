# Feature: Data Access

### Scope

- **Purpose**: Enable direct inspection of the embedded SQL database.
- **Responsibility**: Documents the raw SQL query and table inspection commands.
- **In Scope**: `.query.execute`, `.tables.list`, `.table.list` commands.
- **Out of Scope**: High-level feed and frame browsing (→ `feature/002`).

### Design

`.query.execute` accepts a GlueSQL-compatible SQL string and executes it against the embedded sled database. All three tables (config, feed, frame) are queryable. Results are rendered as formatted tabular output covering all GlueSQL payload variants.

`.tables.list` lists all three storage tables with descriptions of what each stores and the columns they expose.

`.table.list <name>` shows the column names and types for a specific named table, queried from the GlueSQL system catalog.

| Command | Phrase | Description |
|---------|--------|-------------|
| Execute query | `.query.execute query::'SELECT * FROM feed'` | Run arbitrary GlueSQL query |
| List tables | `.tables.list` | Show all storage tables with column descriptions |
| List columns | `.table.list name::frame` | Show columns and types for a specific table |

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | `src/action/query.rs` | Business logic for query execution and result rendering |
| source | `src/action/table.rs` | Business logic for table and column listing |
| source | `src/command/query.rs` | wca command builder for query execute |
| source | `src/command/table.rs` | wca command builders for table commands |
| test | `tests/query_execute.rs` | Integration tests for query execution against real storage |
| doc | [api/001_storage_port.md](../api/001_storage_port.md) | Store and TableStore trait contracts |
