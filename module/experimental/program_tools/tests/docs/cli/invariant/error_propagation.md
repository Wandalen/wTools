# Invariant Spec: error_propagation

### Scope

- **Purpose**: Verify that errors from the build step and the executed program are faithfully propagated to the caller.
- **Responsibility**: Cargo not found, compilation errors, and program runtime errors each produce non-zero exit codes with diagnostics on stderr.
- **In Scope**: Exit code propagation from program; build error surfacing; tool infrastructure errors.
- **Out of Scope**: Timeout handling (→ `param/timeout.md`); workspace cleanup on error (→ `invariant/cleanup_guarantee.md`).

### IC-1: Cargo not found produces non-zero exit with diagnostic

**Given:** `--cargo /nonexistent/cargo` supplied; any source file
**When:** `program_tools run --cargo /nonexistent/cargo main.rs`
**Then:** Exit code non-zero; `stderr` contains a diagnostic identifying that the Cargo binary was not found; `stdout` is empty; the tool does not silently succeed

### IC-2: Compilation error produces non-zero exit with diagnostics

**Given:** A source file containing a syntax error that prevents compilation
**When:** `program_tools run broken.rs`
**Then:** Exit code non-zero; `stderr` contains the compiler error output (at minimum the first fatal error); `stdout` is empty or contains only compile-step output; the tool does not silently succeed
**Commands:** run
