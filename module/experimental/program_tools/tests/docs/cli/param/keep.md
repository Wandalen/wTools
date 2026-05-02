# Parameter Spec: keep

### Scope

- **Purpose**: Verify `--keep` controls whether the temporary workspace is deleted after execution.
- **Responsibility**: Edge cases for absent (cleanup), retained workspace, cleanup on error, and path inspection.
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

### EC-3: Workspace path surfaced when kept

**Given:** A compilable single-file program with `--keep`
**When:** `program_tools run --keep main.rs`
**Then:** Exit code `0`; the path to the retained workspace is surfaced (e.g., in `stderr` or a structured result); the path is valid and navigable
**Commands:** run

### EC-4: Workspace removed even on program failure

**Given:** A program that exits with code `1`
**When:** `program_tools run main.rs` (no `--keep`; program exits non-zero)
**Then:** Exit code non-zero (forwarded from program); temporary workspace is still removed; no orphaned dirs
**Commands:** run

### EC-5: `--keep` retains workspace on program failure

**Given:** A program that exits with code `1`
**When:** `program_tools run --keep main.rs`
**Then:** Exit code non-zero; workspace is retained for post-mortem inspection; workspace path is surfaced
**Commands:** run

### EC-6: `--keep` with `--target-dir` — both paths retained

**Given:** A compilable single-file program; both `--keep` and `--target-dir /tmp/my_cache` supplied
**When:** `program_tools run --keep --target-dir /tmp/my_cache main.rs`
**Then:** Exit code `0`; workspace retained; `/tmp/my_cache` also retained; both paths are valid
**Commands:** run
