# Invariant Spec: cleanup_guarantee

### Scope

- **Purpose**: Verify that temporary workspaces are always removed after a run unless `--keep` is supplied.
- **Responsibility**: Normal success, program failure, build failure, and interrupted execution.
- **In Scope**: Temp workspace deletion on all exit paths; no orphaned directories left on disk.
- **Out of Scope**: Persistent target-dir (→ `param/target_dir.md`); kept workspace (→ `param/keep.md`).

### IC-1: Normal run removes workspace

**Given:** A compilable program that exits `0`; no `--keep` flag
**When:** `program_tools run main.rs` completes
**Then:** Exit code `0`; no temp workspace directory exists on disk after completion; the invariant holds on the success path
**Commands:** run

### IC-2: Error run still removes workspace

**Given:** A program that exits with a non-zero code; no `--keep` flag
**When:** `program_tools run main.rs` completes
**Then:** Exit code non-zero (forwarded); no temp workspace directory exists on disk; the invariant holds on the failure path

**Given:** A program that fails to compile (syntax error); no `--keep` flag
**When:** `program_tools run broken.rs` terminates with a build error
**Then:** Exit code non-zero; no temp workspace directory exists on disk; the invariant holds on the build-failure path
**Commands:** run

**Note — project mode exception**: This invariant applies to single-file and multi-file mode only. In project mode (`run ./my_project/`), no temporary workspace is created; there is nothing to clean up, and the `--keep` flag has no effect.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| doc | `docs/invariant/001_cleanup_guarantee.md` | Canonical cleanup guarantee contract |
| test | `tests/inc/runner_test.rs` | Integration tests verifying workspace cleanup behavior |
| param | `tests/docs/cli/param/keep.md` | `--keep` flag: opt-out from cleanup |
| param | `tests/docs/cli/param/target_dir.md` | `--target-dir` flag: persistent artifact directory |
