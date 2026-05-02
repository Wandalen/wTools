# Parameter Spec: cargo

### Scope

- **Purpose**: Verify `--cargo` controls which Cargo binary is used for the build.
- **Responsibility**: Edge cases for valid binary, invalid path, non-executable file, directory path, and explicit default.
- **In Scope**: Binary path resolution; not-found error; non-executable file error.
- **Out of Scope**: Build profile selection (→ `param/profile.md`).

### EC-1 (Divergence A): Absent — `cargo` resolved via PATH

**Given:** `cargo` is available in the current PATH; a compilable single-file program
**When:** `program_tools run main.rs` (no `--cargo` flag)
**Then:** Exit code `0`; build succeeds using the system Cargo; program output on stdout
**Commands:** run

### EC-2 (Divergence B): Invalid path — binary not found

**Given:** A path that does not exist on the filesystem
**When:** `program_tools run --cargo /nonexistent/cargo main.rs`
**Then:** Exit code `1`; `stderr` contains a diagnostic about the Cargo binary not being found; `stdout` is empty
**Commands:** run

### EC-3: Explicit default value — same as omitting

**Given:** `cargo` is available in PATH; a compilable single-file program
**When:** `program_tools run --cargo cargo main.rs`
**Then:** Exit code `0`; behaviour is identical to omitting the flag
**Commands:** run

### EC-4: Non-executable file

**Given:** A file exists at the path but is not marked executable
**When:** `program_tools run --cargo /tmp/not_executable main.rs`
**Then:** Exit code `1`; `stderr` contains a permission or not-executable error; `stdout` is empty
**Commands:** run

### EC-5: Path is a directory

**Given:** A directory path is supplied instead of a binary
**When:** `program_tools run --cargo /usr/bin main.rs`
**Then:** Exit code `1`; `stderr` contains an error (not a valid executable); `stdout` is empty
**Commands:** run

### EC-6: Alternate Cargo binary with same version

**Given:** A copy of the system Cargo binary exists at `/usr/local/bin/cargo_copy`
**When:** `program_tools run --cargo /usr/local/bin/cargo_copy main.rs`
**Then:** Exit code `0`; build succeeds; output is identical to using the system Cargo
**Commands:** run
