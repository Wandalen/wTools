# src/handlers/

Command handler implementations — one file per command domain.

### Responsibility Table

| File | Responsibility |
|------|----------------|
| mod.rs | Handler module declarations and public re-exports |
| shared_state.rs | Thread-local archive state shared across handlers |
| archive.rs | Archive lifecycle command handlers |
| file.rs | File add, remove, and list command handlers |
| value.rs | Template value set, get, and list command handlers |
| parameter.rs | Parameter define, list, and validate command handlers |
| content.rs | Content transformation command handlers |
| materialize.rs | Template materialization command handler |
| pack.rs | Archive pack and unpack command handlers |
| analysis.rs | Archive analysis and inspection command handlers |
