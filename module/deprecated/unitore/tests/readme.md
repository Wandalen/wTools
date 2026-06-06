# Tests

## Scope

Integration tests for unitore CLI commands and storage operations.

### Responsibility Table

| File | Responsibility |
|------|----------------|
| `basic.rs` | Smoke test for storage initialization |
| `config_add.rs` | Integration test for config add command |
| `config_delete.rs` | Integration test for config delete command |
| `fixtures/` | Static test data — XML feeds and TOML configs |
| `frames_download.rs` | Integration tests for feed download and deduplication |
| `query_execute.rs` | Integration tests for raw SQL query execution |
| `table_list.rs` | Integration test for single table column listing |
| `tables_list.rs` | Integration test for all-tables listing |
