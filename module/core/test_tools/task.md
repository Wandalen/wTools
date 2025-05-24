# Change Proposal for `test_tools` Crate

### Task ID
*   TASK-20250524-160338-FixTestToolsDependency

### Requesting Context
*   **Requesting Crate/Project:** `module/move/unilang_instruction_parser`
*   **Driving Feature/Task:** Resolving Clippy errors in `unilang_instruction_parser`
*   **Link to Requester's Plan:** `module/move/unilang_instruction_parser/plan.md`
*   **Date Proposed:** 2025-05-24

### Overall Goal of Proposed Change
*   Fix the dependency resolution issue in the `test_tools` crate that prevents it from compiling correctly.

### Problem Statement / Justification
*   The `cargo clippy` command on `unilang_instruction_parser` fails because `test_tools` has a dependency resolution error. The error message "couldn't read `module/core/test_tools/src/standalone/../../../../core/error_tools/src/error/mod.rs`: No such file or directory (os error 2)" indicates that the file path is incorrect or the dependency is not properly configured. This prevents `unilang_instruction_parser` from being fully analyzed by clippy.

### Proposed Solution / Specific Changes
*   Investigate the `test_tools` crate's `Cargo.toml` and `src/lib.rs` to identify the incorrect file path or dependency configuration.
*   Correct the file path or dependency configuration to ensure that `test_tools` can find the `error_tools` module.
*   Consider using workspace dependencies to manage dependencies between crates in the workspace.

### Expected Behavior & Usage Examples (from Requester's Perspective)
*   After this change, `cargo build` and `cargo clippy` should succeed for the `test_tools` crate.

### Acceptance Criteria (for this proposed change)
*   The `test_tools` crate compiles successfully.
*   The `cargo clippy` command on `unilang_instruction_parser` no longer fails due to the `test_tools` dependency.

### Potential Impact & Considerations
*   This change should not introduce any breaking changes to the public API of `test_tools`.
*   This change may require updating the `Cargo.toml` file and/or modifying the `src/lib.rs` file.

### Alternatives Considered (Optional)
*   N/A

### Notes & Open Questions
*   What is the correct path to the `error_tools` module?
*   Is the dependency on `error_tools` correctly configured in `Cargo.toml`?