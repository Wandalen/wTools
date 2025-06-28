# Change Proposal for Workspace Root

### Task ID
*   `TASK-20250628-081259-ClippyWildcardImports`

### Requesting Context
*   **Requesting Crate/Project:** `module/core/macro_tools`
*   **Driving Feature/Task:** To allow `clippy::wildcard_imports` at the workspace level to prevent warnings in macro-generated code.
*   **Link to Requester's Plan:** `module/core/macro_tools/task.md`
*   **Date Proposed:** 2025-06-28

### Overall Goal of Proposed Change
*   To configure the workspace `Cargo.toml` to allow the `clippy::wildcard_imports` lint, which is currently too aggressive for some macro-generated code.

### Problem Statement / Justification
*   The `clippy::wildcard_imports` lint generates warnings in macro-generated code, even when the wildcard imports are necessary or idiomatic within the generated context. This can lead to noisy clippy output and make it harder to identify genuine issues. Allowing this lint at the workspace level will prevent these warnings without disabling other important clippy checks, improving the developer experience and focusing attention on more critical linting issues.

### Proposed Solution / Specific Changes
*   **File:** `Cargo.toml` (at workspace root)
*   **Section:** `[workspace.lints.clippy]`
*   **Change:** Add or modify the `wildcard_imports` entry to `"allow"`.

    ```toml
    # Cargo.toml (workspace root)

    [workspace.lints.clippy]
    # ... other clippy lints ...
    wildcard_imports = "allow" # Proposed addition/modification
    ```

### Expected Behavior & Usage Examples (from Requester's Perspective)
*   After this change is applied to the workspace `Cargo.toml`, running `cargo clippy --workspace` should no longer report `wildcard_imports` warnings originating from macro-generated code within any workspace member crate that inherits workspace lints.

### Acceptance Criteria (for this proposed change)
*   The `Cargo.toml` file at the workspace root is updated to include `wildcard_imports = "allow"` under `[workspace.lints.clippy]`.
*   Running `cargo clippy --workspace` (after the change is applied) completes without `wildcard_imports` warnings.

### Potential Impact & Considerations
*   **Breaking Changes:** No breaking changes are anticipated, as this is a lint configuration change.
*   **Dependencies:** No new dependencies are introduced.
*   **Performance:** No performance implications for compilation or runtime.
*   **Security:** No security considerations.
*   **Testing:** Verification will involve running `cargo clippy --workspace` and confirming the absence of the specific lint warnings.

### Alternatives Considered (Optional)
*   Disabling the lint at the crate level: This would require modifying each crate's `Cargo.toml` or `lib.rs`, which is less centralized and harder to maintain across a large workspace.
*   Refactoring macro-generated code to avoid wildcard imports: While ideal in some cases, it might be overly complex or impossible for certain macro patterns, especially when dealing with `quote!` and `syn` where generated code often benefits from wildcard imports for brevity and clarity.

### Notes & Open Questions
*   This change is specifically aimed at reducing noise from clippy in a workspace with extensive macro usage.