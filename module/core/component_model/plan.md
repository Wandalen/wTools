# Project Plan: Refine Component Model Crates

## Goal

Refine the `component_model`, `component_model_meta`, and `component_model_types` crates to be production-ready, ensuring complete isolation from the original `former` crate where appropriate, consistency, clarity, conciseness, correctness, and adherence to all specified rules (codestyle, clippy).

## Crates Involved

*   `component_model` (User-facing facade)
*   `component_model_meta` (Proc-macro implementation)
*   `component_model_types` (Core traits and types)

## Increments

*   ✅ **Increment 1: Initial Analysis, File Cleanup & Basic Renaming**
    *   **Goal:** Perform a first pass across all three crates to remove obvious unused files/code, rename files/directories from "former" to "component_model", and identify major areas needing renaming/rewriting in subsequent increments.
    *   **Rationale:** Establish a cleaner baseline and consistent file structure before deeper refactoring.
    *   **Detailed Steps:**
        *   `component_model_types`:
            *   Rename `examples/former_types_trivial.rs` to `examples/component_model_types_trivial.rs`.
            *   Delete `tests/inc/component_model_tests/` directory (tests seem former-specific, will re-evaluate later if component model tests are needed here).
            *   Delete `tests/inc/components_tests/` directory (these test derives, belong in `_meta` or `_model` tests).
            *   Review `src/` files for any obvious `// former_...` comments or unused code blocks leftover from split - remove if clearly garbage.
        *   `component_model_meta`:
            *   Rename `src/derive_former.rs` to `src/derive_component_model.rs`.
            *   Could not rename `src/derive_former/` directory to `src/component/`.
            *   Rename `src/derive_component_model/former_struct.rs` to `src/derive_component_model/component_model_struct.rs`.
            *   Rename `src/derive_component_model/former_enum.rs` to `src/derive_component_model/component_model_enum.rs`.
            *   Rename `src/derive_component_model/former_enum/` directory to `src/derive_component_model/component_model_enum/`.
            *   Update `mod` declarations in `src/lib.rs` and `src/derive_component_model.rs` to reflect renames.
            *   Review `src/` files for obvious `// former_...` comments or unused code blocks - remove if clearly garbage.
            *   Review `plan.md` - assess if the file splitting plan is still relevant or completed. Update/remove as necessary.
        *   `component_model`:
            *   Rename `examples/former_*.rs` files to `examples/component_model_*.rs`.
            *   Rename `tests/inc/former_struct_tests/` to `tests/inc/component_model_struct_tests/`.
            *   Rename `tests/inc/former_enum_tests/` to `tests/inc/component_model_enum_tests/`.
            *   Update `mod` declarations in `tests/inc/mod.rs` to reflect renames.
            *   Review `examples/` and `tests/` for obvious `// former_...` comments or unused code blocks - remove if clearly garbage.
            *   Initialize `plan.md` with this plan structure.
            *   Delete `advanced.md` (will be rewritten later).
    *   **Verification Strategy:** All crates compile (`cargo check --all-targets` in workspace). File structure is updated. Git diff shows primarily renames and deletions.

## Notes & Insights

*   **Decision Point:** The most critical decision is the core terminology (`Former` vs. `ComponentModel` vs. something else). This needs to be made in Increment 2. The rest of the plan assumes `ComponentModel` is chosen, but can be adapted.
*   **Subform Naming:** Decided to keep `subform_*` attribute names as they describe the *mechanism*, but rename generated types/methods related to them (e.g., `ParentSubformScalarChildEnd` -> `ParentSubcomponentScalarChildEnd`).
*   **Attribute Renaming:** Decided to rename `#[former(...)]` to `#[component_model(...)]` for consistency.
*   **Testing Strategy:** Tests related to derive macros should primarily reside in `component_model_meta` or `component_model` (integration tests), while `component_model_types` tests should focus on the traits themselves. Existing tests need careful migration/adaptation.
*   **Documentation Focus:** Documentation needs a significant shift from a "builder pattern" focus (former) to a "component model / type-based assignment" focus, potentially still including the builder pattern as one application.
