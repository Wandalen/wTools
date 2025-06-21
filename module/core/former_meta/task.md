# Change Proposal for `former_meta`

### Task ID
*   `TASK-20250524-FORMER-META-COMPILATION-FIX`

### Requesting Context
*   **Requesting Crate/Project:** `module/move/unilang_instruction_parser` (and potentially other workspace crates)
*   **Driving Feature/Task:** Final verification of `unilang_instruction_parser` requires a clean workspace build, which is currently blocked by compilation errors and warnings in `former_meta`.
*   **Link to Requester's Plan:** `../../move/unilang_instruction_parser/plan.md`
*   **Date Proposed:** 2025-05-24

### Overall Goal of Proposed Change
*   Resolve compilation error `E0554` and clippy warnings in `former_meta` to allow successful compilation on stable Rust.

### Problem Statement / Justification
*   During `cargo test --workspace`, `former_meta` fails to compile with `error[E0554]: #![feature]` may not be used on the stable release channel` due to `#![ feature( proc_macro_totokens ) ]` being used. This unstable feature is not available on stable Rust, blocking compilation for any dependent crates.
*   Additionally, `former_meta` generates clippy warnings: `unused import: quote::quote_spanned`, `unreachable expression`, and `unused variable: attr_property`. These warnings prevent clean builds when `-D warnings` is enabled.

### Proposed Solution / Specific Changes
*   **File:** `src/lib.rs`
    *   **Change:** Remove or conditionally compile `#![ feature( proc_macro_totokens ) ]`. If `proc_macro_totokens` is strictly necessary, `former_meta` should require a nightly toolchain, or an alternative stable API should be used.
*   **File:** `src/derive_former/former_enum/unit_variant_handler.rs`
    *   **Change:** Remove `quote::quote_spanned` import if unused.
    *   **Change:** Refactor `return diag::return_syn_err!( ... )` to avoid `unreachable expression` warning.
    *   **Change:** Prefix `attr_property` with `_` if it's intentionally unused, or use it.

### Expected Behavior & Usage Examples (from Requester's Perspective)
*   `cargo build -p former_meta` and `cargo clippy -p former_meta -- -D warnings` should complete successfully on a stable Rust toolchain.
*   Dependent crates like `unilang_instruction_parser` should be able to compile without errors or warnings originating from `former_meta`.

### Acceptance Criteria (for this proposed change)
*   `cargo build -p former_meta` exits with code 0.
*   `cargo clippy -p former_meta -- -D warnings` exits with code 0 and no warnings.
*   The functionality of `former_meta` remains unchanged.

### Potential Impact & Considerations
*   **Breaking Changes:** No breaking changes are anticipated if the `proc_macro_totokens` feature can be removed or replaced without affecting core functionality.
*   **Dependencies:** No new dependencies.
*   **Performance:** No significant performance impact.
*   **Security:** No security implications.
*   **Testing:** Existing tests for `former_meta` should continue to pass.

### Notes & Open Questions
*   Clarification is needed on the necessity of `proc_macro_totokens`. If it's critical, the crate might need to explicitly state nightly toolchain requirement.