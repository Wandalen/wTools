# Feature: Script Execution

### Scope

- **Purpose**: Enable compilation and execution of Rust source files and projects as if they were interpreted scripts, hiding all build complexity from the caller.
- **Responsibility**: Documents the script execution feature — its execution model, transparency properties, supported input modes, manifest generation behavior, and architectural boundaries.
- **In Scope**: Single-file execution; multi-file execution; existing Cargo project execution; automatic Cargo manifest generation; temporary workspace lifecycle.
- **Out of Scope**: Output capture and assertion (→ `feature/002`); artifact cache management (→ `feature/003`); test convenience forms (→ `feature/004`).

### Design

Script execution is the central feature of `program_tools`. A caller provides a plan — describing source content and execution parameters — and the runner produces an execution result. From the caller's perspective, the process is identical to invoking an interpreter: submit code, receive output. All Cargo invocation, filesystem management, and process complexity is internal and invisible.

**Three execution modes**:

*Single-file mode*: The caller provides one source file path or inline source content. The runner wraps it in a minimal generated Cargo project, placing the content at the binary entry point path. No manifest authoring is required.

*Multi-file mode*: The caller provides an ordered collection of source files through the Program builder. The runner creates a workspace with the specified file layout. A manifest is generated unless the caller supplies one inline via the Program builder.

*Project mode*: The caller points to an existing Cargo project directory. The runner executes the project as-is, skipping all manifest generation and using the existing source tree.

**The interpreter illusion**: Callers do not invoke Cargo, do not manage target directories, and do not handle process cleanup. The runner presents a call-and-response interface: submit a plan, receive a captured output value. The execution cost of the compilation step is the only visible overhead compared to a true interpreter.

**Manifest generation**: When no manifest is supplied, the runner generates a minimal Cargo.toml using the configured package name and edition. The generated manifest is valid for programs using only the Rust standard library. Programs requiring external dependencies must supply an explicit manifest through the Program builder's manifest field.

**Architectural boundary**: `program_tools` owns plan construction and execution orchestration. Compilation is delegated to Cargo. Filesystem operations use the OS via standard library primitives. Process management invokes a child subprocess. The crate does not embed a Rust compiler or interpreter.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | `src/program.rs` | Plan, Program, Source data structures used as input |
| source | `src/runner.rs` | Runner implementation for all execution entry points |
| test | `tests/inc/runner_test.rs` | Integration tests for run_source, run_file, run_project |
| doc | `docs/api/001_builder_api.md` | Builder API for constructing execution plans |
| doc | `docs/api/002_runner_api.md` | Runner execution entry points |
| doc | `docs/feature/002_output_capture.md` | Output capture: companion to execution |
| doc | `docs/feature/003_artifact_management.md` | Temp workspace and artifact lifecycle |
| doc | `docs/feature/004_programmatic_test_integration.md` | Test convenience layer built on script execution |
| doc | `docs/invariant/001_cleanup_guarantee.md` | Cleanup contract for temporary workspaces |
| doc | `docs/invariant/002_execution_isolation.md` | Isolation guarantee for concurrent runs |
| doc | `docs/invariant/004_error_propagation.md` | Error surfacing from the build and execution pipeline |
| doc | `docs/pattern/001_builder_hierarchy.md` | Builder hierarchy pattern for constructing plans |
