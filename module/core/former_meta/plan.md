# Project Plan: Refactor Enum Unit Variant Handling in `former_meta`

### Goal
*   Refactor the implementation of `#[derive(Former)]` for **enum unit variants** within the `former_meta` crate, assuming necessary generalizations are made in the `proc_macro_tools` crate.

### Progress
*   🚀 Phase 1 Complete (Increment 1)
*   🚧 Phase 2 In Progress (Increment 2)

### Target Crate
*   `module/core/former_meta`

### Relevant Context
*   Files to Include (for AI's reference, if `read_file` is planned, primarily from Target Crate):
    *   `module/core/former_meta/src/derive_former/former_enum/unit_variant_handler.rs`
    *   `module/core/former_meta/src/derive_former/former_enum.rs`
*   Crates for Documentation (for AI's reference, if `read_file` on docs is planned):
    *   `former`
    *   `proc_macro_tools`
*   External Crates Requiring `task.md` Proposals (if any identified during planning):
    *   `module/alias/proc_macro_tools` (Reason: Needs new generalized utilities for identifier case conversion and generics handling to support the refactoring in `former_meta`.)

### Expected Behavior Rules / Specifications (for Target Crate)
*   **Rule 1a (Unit + `#[scalar]`):** Generates `Enum::variant() -> Enum`.
*   **Rule 2a (Unit + `#[subform_scalar]`):** Must produce a compile-time error.
*   **Rule 3a (Unit + Default):** Generates `Enum::variant() -> Enum`.
*   **Rule 4a (`#[standalone_constructors]` on Enum):** For unit variants, generates a top-level function `fn variant_name() -> EnumName` (name in snake_case).

### Target File Structure (If Applicable, within Target Crate)
*   No major file structure changes are planned for `former_meta`.

### Increments

*   [✅] Increment 1: Propose API additions to `proc_macro_tools` via `task.md`
    *   Detailed Plan Step 1: Draft the content for `module/alias/proc_macro_tools/task.md` using the "External Crate Change Proposal Structure". The content will specify the addition of `cased_ident_from_ident` and `GenericsRef` utilities.
    *   Detailed Plan Step 2: Use `write_to_file` to create/update `module/alias/proc_macro_tools/task.md`.
    *   Pre-Analysis: The `former_meta` crate currently has manual and repetitive logic for converting identifier cases and handling generic parameters. Generalizing this into `proc_macro_tools` will improve code reuse and maintainability.
    *   Crucial Design Rules: [Traits: Encourage Modular Design], [Visibility: Keep Implementation Details Private]
    *   Relevant Behavior Rules: N/A for this increment.
    *   Verification Strategy: Confirm `task.md` is written successfully by analyzing the `write_to_file` tool output.
    *   Commit Message: "chore: Propose API additions to proc_macro_tools for former refactoring"

*   [⚫] Increment 2: Implement Improved Refactoring (Enum Unit Variants in `former_meta`)
    *   Detailed Plan Step 1: Modify `former_meta/src/derive_former/former_enum/unit_variant_handler.rs` to use the (proposed) new utilities from `proc_macro_tools`.
    *   Verification Strategy: Execute `cargo check -p former_meta` and `cargo test -p former_meta`.
    *   Commit Message: "refactor(former_meta): Improve unit variant handling using macro_tools"

*   [⚫] Increment 3: Final Verification
    *   Detailed Plan Step 1: Run `cargo clippy --workspace --all-targets -- -D warnings`.
    *   Detailed Plan Step 2: Run `cargo test --workspace`.
    *   Verification Strategy: Analyze output of `execute_command` for both commands to ensure no new issues.
    *   Commit Message: "chore(former): Final verification after unit variant refactor"

### Task Requirements
*   The refactoring must not change the externally observable behavior of the `Former` macro for enum unit variants.
*   All new and modified code must adhere to the system prompt's Design and Codestyle Rules.

### Project Requirements
*   (This section is reused and appended to across tasks for the same project. Never remove existing project requirements.)
*   Must use Rust 2021 edition.
*   All public APIs must be documented.

### Notes & Insights
*   This plan supersedes the one in `module/core/former/plan.md` for the execution of this task.
*   The successful completion of Increment 2 depends on the eventual implementation of the changes proposed in Increment 1's `task.md`.