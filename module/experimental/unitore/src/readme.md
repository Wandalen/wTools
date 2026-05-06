# src

Source modules for the unitore feed aggregator.

## Responsibility Table

| Item | Responsibility |
|------|----------------|
| `lib.rs` | Crate root; re-exports modules, `Report` trait, `EMPTY_CELL` constant. |
| `main.rs` | Binary entry point; wires commands into CLI executor. |
| `executor.rs` | Initializes storage and runs the wca command aggregator. |
| `feed_config.rs` | Parses TOML subscription config files into subscription structs. |
| `retriever.rs` | Fetches raw feed bytes from remote URLs via HTTP. |
| `action/` | Pure async functions executed when CLI commands are performed. |
| `command/` | wca `Command` builders that bind CLI verbs to action functions. |
| `entity/` | Domain types (Feed, Frame, Config) and their storage traits. |
| `sled_adapter/` | GlueSQL/Sled storage implementations of entity traits. |
| `tool/` | Table display utilities used by report `Display` impls. |
