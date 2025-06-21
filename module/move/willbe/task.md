# Change Proposal for `willbe` and `cargo_will`

### Task ID
*   `TASK-20250524-WILLBE-CARGO-WILL-COLLISION-FIX`

### Requesting Context
*   **Requesting Crate/Project:** Workspace-wide build (`wTools`)
*   **Driving Feature/Task:** Final verification of `unilang_instruction_parser` (and overall workspace health) is affected by output filename collisions between `willbe` and `cargo_will`.
*   **Link to Requester's Plan:** `../unilang_instruction_parser/plan.md`
*   **Date Proposed:** 2025-05-24

### Overall Goal of Proposed Change
*   Resolve output filename collisions between `willbe` and `cargo_will` crates to ensure a clean workspace build.

### Problem Statement / Justification
*   During `cargo test --workspace` (and `cargo build --workspace`), Cargo reports multiple warnings about "output filename collision" for binary targets named `cargo-will` and `will` and `willbe` from both `willbe` and `cargo_will` crates. This indicates that both crates are trying to produce executables with the same names, leading to conflicts in the `target/debug/` (or `target/release/`) directory. While currently warnings, Cargo explicitly states this "may become a hard error in the future". This issue affects the cleanliness and reliability of workspace builds.

### Proposed Solution / Specific Changes
*   **Option 1 (Preferred): Rename binary targets in one of the crates.**
    *   For example, in `module/alias/cargo_will/Cargo.toml`, rename the `[[bin]]` sections to have unique names (e.g., `cargo-will-alias`, `will-alias`, `willbe-alias`). This is generally preferred if `cargo_will` is intended as an alias or wrapper.
*   **Option 2: Configure `Cargo.toml` to compile separately.**
    *   If both crates are intended to produce binaries with the same names but are used in different contexts, their `Cargo.toml` files could be configured to compile them separately (e.g., by using `package.default-run` or by ensuring they are not built simultaneously in a way that causes collision). However, renaming is usually simpler.

### Expected Behavior & Usage Examples (from Requester's Perspective)
*   `cargo test --workspace` and `cargo build --workspace` should complete without any "output filename collision" warnings.
*   The functionality of both `willbe` and `cargo_will` should remain as intended, with their respective binaries accessible by their (potentially new) names.

### Acceptance Criteria (for this proposed change)
*   `cargo test --workspace` and `cargo build --workspace` exit with code 0 and no "output filename collision" warnings.
*   The binaries produced by `willbe` and `cargo_will` are distinct and functional.

### Potential Impact & Considerations
*   **Breaking Changes:** Renaming binary targets would be a breaking change for any scripts or users directly invoking `cargo-will`, `will`, or `willbe` from the affected crate by its old name. This should be communicated.
*   **Dependencies:** No new dependencies.
*   **Performance:** No significant performance impact.
*   **Security:** No security implications.
*   **Testing:** Existing tests for both `willbe` and `cargo_will` should continue to pass.

### Notes & Open Questions
*   Which crate should be prioritized for renaming? `cargo_will` seems like a more likely candidate for renaming its binaries if `willbe` is the primary tool.