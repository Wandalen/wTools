# API: CLI Interface

### Scope

- **Purpose**: Document the `will` CLI commands, parameters, and binary entry points available to workspace maintainers.
- **Responsibility**: Complete reference for all willbe CLI commands with their parameters and usage.
- **In Scope**: Binary names, command syntax, parameters with types and defaults, exit codes, dry-run behavior.
- **Out of Scope**: Implementation details (see `../../src/`), architectural patterns (see `../pattern/`), usage workflows (see `../feature/`).

### Abstract

willbe exposes three binary entry points (`will`, `willbe`, `cargo-will`) that delegate to the same command logic via the `wca` framework. Commands are prefixed with `.` following wca conventions.

### Operations

**`.publish [path_glob] [dry:bool]`**
Publish workspace crates in topological dependency order. `path_glob` filters which crates to consider (default: all crates under current directory). `dry:1` prints the publish plan without executing; `dry:0` (default) executes. On execution, each package is version-bumped in Cargo.toml, committed with `git commit -m <crate>-v<version>`, and pushed before the `cargo publish` call.

**`.test [path_glob] [power:int] [dry:bool]`**
Run tests across workspace crates. `power` controls feature combination breadth (1=default features only, higher values test more combinations). `path_glob` and `dry` behave as in `.publish`.

**`.list [path_glob] [format:str]`**
List workspace crates. `format:tree` renders the dependency tree; default renders a flat list.

**`.cicd.renew`**
Regenerate GitHub Actions workflow files for all workspace crates from templates.

**`.readme.health.table.renew`**
Update README health/status badge table across workspace crates.

**`.readme.headers.renew`**
Update module headers across all workspace crates.

**`.features [path_glob]`**
Analyze and list feature configurations for workspace crates.

**`.publish.diff [path_glob]`**
Show differences between local crate state and published crates.io version.

### Error Handling

Commands return a non-zero exit code on failure with an error message to stderr. The `.publish` command fails fast: if any package in the publish plan fails (pack, publish, git, or network), the entire run is aborted and subsequent packages are not published. No rollback is performed for already-published packages.

### Compatibility Guarantees

Semantic versioning. No backward compatibility guarantees while willbe is at `0.x.y`. willbe is a legacy crate; the CLI interface is stable for existing usage but no new commands or parameters will be added.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | `../../src/command/publish.rs` | .publish command definition |
| source | `../../src/command/test.rs` | .test command definition |
| source | `../../src/command/list.rs` | .list command definition |
| source | `../../src/action/publish.rs` | Publish action implementation |
| doc | `../feature/001_workspace_management.md` | Feature-level publish workflow documentation |
