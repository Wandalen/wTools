# Invariant Spec: execution_isolation

### Scope

- **Purpose**: Verify that each `program_tools run` invocation uses an isolated workspace with no cross-run contamination.
- **Responsibility**: Sequential runs produce separate workspaces; concurrent runs do not bleed source files.
- **In Scope**: Workspace path uniqueness per invocation; source file isolation between concurrent runs.
- **Out of Scope**: Artifact caching with shared `--target-dir` (→ `param/target_dir.md`); output isolation (→ `invariant/output_determinism.md`).

### IC-1: Sequential runs use separate workspaces

**Given:** Two compilable programs `a.rs` and `b.rs` that print different output
**When:** `program_tools run a.rs` followed by `program_tools run b.rs` (sequential, no `--keep`)
**Then:** Both exit `0`; each run produces only its own output; no source or artifact from the first run appears in the second run's workspace

### IC-2: Concurrent runs do not bleed source files

**Given:** Two compilable programs `a.rs` and `b.rs` launched simultaneously
**When:** Both `program_tools run a.rs` and `program_tools run b.rs` run concurrently
**Then:** Both exit `0`; each run produces only its own correct output; workspace directories are distinct; neither run's sources appear in the other's workspace
**Commands:** run
