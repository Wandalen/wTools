# Src

### Responsibility Table

| File | Responsibility |
|------|----------------|
| `lib.rs` | Re-export public API via mod_interface |
| `main.rs` | Dispatch `run` subcommand via clap |
| `output.rs` | Define CapturedOutput with assertion predicates |
| `program.rs` | Define Source/Program/Plan builder structs |
| `run_options.rs` | Define RunOptions execution configuration |
| `runner.rs` | Execute scripts in isolated temporary workspaces |
