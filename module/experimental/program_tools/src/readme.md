# Src

### Responsibility Table

| File | Responsibility |
|------|----------------|
| `lib.rs` | Re-export public API via mod_interface |
| `main.rs` | Thin entry point delegating to cli::run_cli |
| `cli.rs` | CLI structs and run_cli shared by both binaries |
| `output.rs` | Define CapturedOutput with assertion predicates |
| `program.rs` | Define Source/Program/Plan builder structs |
| `run_options.rs` | Define RunOptions execution configuration |
| `runner.rs` | Execute scripts in isolated temporary workspaces |
| `bin/` | Short-alias entry points (pt binary) |
