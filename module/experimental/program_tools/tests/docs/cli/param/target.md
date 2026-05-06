# Parameter Spec: target

### Scope

- **Purpose**: Verify that `<TARGET>` correctly selects the execution mode based on the provided path type.
- **Responsibility**: Edge cases for file path, directory path, missing argument, invalid path, and directory without manifest.
- **In Scope**: Single-file mode selection; project mode selection; missing and invalid path handling.
- **Out of Scope**: Command integration (→ `command/run.md`); output assertion (→ `param/capture.md`).

### EC-1 (Divergence A): File path — single-file execution mode

**Given:** A valid `.rs` file exists at the specified path containing `fn main() {}`
**When:** `program_tools run main.rs`
**Then:** Exit code `0`; runner generates a manifest; program output appears on stdout; no pre-existing `Cargo.toml` required
**Commands:** run

### EC-2 (Divergence B): Directory path — project execution mode

**Given:** A directory exists at the path containing a valid `Cargo.toml` and `src/main.rs`
**When:** `program_tools run ./my_project/`
**Then:** Exit code `0`; no manifest generation; project's own `Cargo.toml` is used; program output on stdout
**Commands:** run

### EC-3: Omitted — missing required argument

**Given:** No `<TARGET>` argument is provided
**When:** `program_tools run`
**Then:** Exit code `1`; `stderr` contains usage error referencing the missing argument; `stdout` is empty
**Commands:** run

### EC-4: Path does not exist

**Given:** The specified path does not exist on the filesystem
**When:** `program_tools run /nonexistent/script.rs`
**Then:** Exit code `1`; `stderr` describes the path-not-found error; `stdout` is empty
**Commands:** run

### EC-5: Directory without `Cargo.toml`

**Given:** A directory exists at the path but contains no `Cargo.toml`
**When:** `program_tools run ./empty_dir/`
**Then:** Exit code `1`; `stderr` describes the missing manifest; `stdout` is empty
**Commands:** run

### EC-6: File path with non-`.rs` extension

**Given:** A file with a `.txt` extension containing valid Rust source exists at the path
**When:** `program_tools run script.txt`
**Then:** Exit code `0` if execution proceeds without extension validation, or `1` if extension is rejected; exit code is consistent across repeated calls and documented
**Commands:** run
