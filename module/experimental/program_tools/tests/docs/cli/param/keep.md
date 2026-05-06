# Parameter Spec: keep

### Scope

- **Purpose**: Verify `--keep` controls whether the temporary workspace is deleted after execution.
- **Responsibility**: Edge cases for absent (cleanup), retained workspace, cleanup on error, and project-mode no-op.
- **In Scope**: Temp workspace lifecycle; deletion guarantee on success; retention when `--keep` supplied.
- **Out of Scope**: Persistent target-dir (→ `param/target_dir.md`); `invariant/001_cleanup_guarantee.md`.

### EC-1 (Divergence A): Absent — workspace removed after run

**Given:** A compilable single-file program; no `--keep` flag
**When:** `program_tools run main.rs`
**Then:** Exit code `0`; no temporary workspace directory remains on the filesystem after the run
**Commands:** run

### EC-2 (Divergence B): `--keep` supplied — workspace retained

**Given:** A compilable single-file program
**When:** `program_tools run --keep main.rs`
**Then:** Exit code `0`; the temporary workspace directory is retained on the filesystem; it contains the generated `Cargo.toml` and source files
**Commands:** run

### EC-3: Workspace directory exists in OS temp after `--keep`

**Given:** A compilable single-file program with `--keep`
**When:** `program_tools run --keep main.rs`
**Then:** Exit code `0`; a directory named `program_tools_<pid>_<nanos>` remains in the system temp directory; it contains the generated `Cargo.toml` and source files; the CLI does not print the workspace path — the caller must locate it via the naming pattern
**Commands:** run

### EC-4: Workspace removed even on program failure

**Given:** A program that exits with code `1`
**When:** `program_tools run main.rs` (no `--keep`; program exits non-zero)
**Then:** Exit code non-zero (forwarded from program); temporary workspace is still removed; no orphaned dirs
**Commands:** run

### EC-5: `--keep` retains workspace on program failure

**Given:** A program that exits with code `1`
**When:** `program_tools run --keep main.rs`
**Then:** Exit code non-zero; workspace is retained for post-mortem inspection; the directory remains in the OS temp dir under the `program_tools_<pid>_<nanos>` naming pattern
**Commands:** run

### EC-6: `--keep` in project mode — flag has no effect

**Given:** An existing Cargo project directory; `--keep` flag supplied
**When:** `program_tools run --keep ./my_project/`
**Then:** Exit code `0`; `--keep` has no effect; no temporary workspace is created in project mode so there is nothing to retain; Cargo's own target directory (inside the project) is unaffected by this flag
**Commands:** run

### EC-7: `--keep` with `--target-dir` — both paths retained

**Given:** A compilable single-file program; both `--keep` and `--target-dir /tmp/my_cache` supplied
**When:** `program_tools run --keep --target-dir /tmp/my_cache main.rs`
**Then:** Exit code `0`; workspace retained; `/tmp/my_cache` also retained; both paths are valid
**Commands:** run
