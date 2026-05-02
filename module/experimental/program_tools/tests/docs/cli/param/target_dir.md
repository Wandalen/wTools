# Parameter Spec: target_dir

### Scope

- **Purpose**: Verify `--target-dir` controls artifact cache directory persistence.
- **Responsibility**: Edge cases for absent (temp cleanup), persistent path, non-existent path, read-only path, and concurrent runs.
- **In Scope**: Temp cleanup behaviour; persistent directory creation; artifact reuse across runs.
- **Out of Scope**: Workspace cleanup (→ `param/keep.md`); `invariant/001_cleanup_guarantee.md`.

### EC-1 (Divergence A): Absent — temp dir created and removed

**Given:** A compilable single-file program; no `--target-dir` flag
**When:** `program_tools run main.rs`
**Then:** Exit code `0`; no persistent target directory remains after the run; artifacts are inside the temp workspace and removed with it
**Note**: In project mode (`run ./my_project/`), no temporary workspace is created; `--target-dir` absent means Cargo uses its implicit target directory inside the project directory, which is not cleaned up by the runner
**Commands:** run

### EC-2 (Divergence B): Persistent path — artifacts retained

**Given:** A writable directory path `/tmp/my_cache`
**When:** `program_tools run --target-dir /tmp/my_cache main.rs`
**Then:** Exit code `0`; `/tmp/my_cache` exists after the run and contains Cargo build artifacts (e.g. `debug/` subdirectory)
**Commands:** run

### EC-3: Second run with same target-dir reuses artifacts

**Given:** `/tmp/my_cache` already populated from a previous run of the same program
**When:** `program_tools run --target-dir /tmp/my_cache main.rs` (second invocation)
**Then:** Exit code `0`; second run completes faster than first (no full recompilation); stdout output is identical
**Commands:** run

### EC-4: Non-existent path — created automatically

**Given:** A path `/tmp/new_cache_dir` that does not exist
**When:** `program_tools run --target-dir /tmp/new_cache_dir main.rs`
**Then:** Exit code `0`; `/tmp/new_cache_dir` is created by the runner; contains Cargo artifacts after the run
**Commands:** run

### EC-5: Read-only directory

**Given:** A directory that exists but the current user cannot write to
**When:** `program_tools run --target-dir /read_only_dir main.rs`
**Then:** Exit code `1`; `stderr` describes a write permission error; `stdout` is empty
**Commands:** run

### EC-6: Concurrent runs with same target-dir

**Given:** Two source files executed simultaneously with the same `--target-dir`
**When:** Both `program_tools run --target-dir /shared main.rs` invocations run concurrently
**Then:** Both exit `0`; Cargo's file-locking ensures no artifact corruption; outputs are correct for their respective source files
**Commands:** run
