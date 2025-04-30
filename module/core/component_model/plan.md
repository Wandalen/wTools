# Project Plan: Refine Component Model Crates

## Goal

Refine the `component_model`, `component_model_meta`, and `component_model_types` crates to be production-ready, ensuring complete isolation from the original `former` crate where appropriate, consistency, clarity, conciseness, correctness, and adherence to all specified rules (codestyle, clippy).

## Crates Involved

*   `component_model` (User-facing facade)
*   `component_model_meta` (Proc-macro implementation)
*   `component_model_types` (Core traits and types)

## Increments

*   ⚫ **Increment 1: Initial Analysis, File Cleanup & Basic Renaming**
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
            *   Rename `src/derive_former/` directory to `src/derive_component_model/`.
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

*   ⚫ **Increment 2: Terminology Decision & Global Rename (`Former*` -> `ComponentModel*`)**
    *   **Goal:** Decide on the final core terminology (e.g., keep `Former` or change to `ComponentModel` or similar) and apply this consistently across all three crates in code identifiers (structs, traits, functions, variables), documentation, and user-facing messages.
    *   **Rationale:** Establish consistent naming early to avoid confusion and rework later. This is a fundamental decision affecting the entire API surface.
    *   **Decision Point:** Choose the core name. Let's assume `ComponentModel` for this plan. User must confirm.
        *   `Former` -> `ComponentModel` (derive macro name)
        *   `former()` -> `component_model()` (constructor method)
        *   `*Former` -> `*ComponentModel` (generated struct names)
        *   `*FormerStorage` -> `*ComponentModelStorage`
        *   `*FormerDefinition*` -> `*ComponentModelDefinition*`
        *   `FormerMutator` -> `ComponentModelMutator`
        *   `FormerBegin` -> `ComponentModelBegin`
        *   `subform_*` attributes -> `subcomponent_*`? (Or keep `subform` as it describes the *mechanism*?) - **Decision:** Keep `subform_*` for attributes as it describes the nesting mechanism, but rename generated types/methods.
        *   `component_model(...)` attribute -> Keep as is, or rename `former(...)` to `component_model(...)`? **Decision:** Rename `#[former(...)]` to `#[component_model(...)]`.
    *   **Detailed Steps:**
        *   **Apply Renames (Code):**
            *   `component_model_types`: Rename traits/structs in `definition.rs`, `forming.rs`, `collection/*.rs`. Update `*Ext` trait methods.
            *   `component_model_meta`: Rename derive macro entry point in `src/lib.rs`. Rename main function in `derive_component_model.rs`. Rename generated structs/types/methods within all `src/derive_component_model/**/*.rs` files (including generated code in `quote!`). Rename `#[former(...)]` attribute parsing logic to `#[component_model(...)]`.
            *   `component_model`: Update re-exports in `src/lib.rs`.
        *   **Apply Renames (Docs & Comments):**
            *   Search and replace "Former", "former", "subformer" (where appropriate) with "ComponentModel", "component_model", "subcomponent" (or chosen terms) in all `*.rs`, `*.md` files across the three crates. Pay close attention to context.
        *   **Apply Renames (Examples & Tests):**
            *   Update all example code (`component_model/examples/*.rs`) to use the new derive name, constructor method, and attribute name.
            *   Update all test code (`component_model/tests/inc/**/*.rs`, `component_model_meta/tests/inc/**/*.rs`, `component_model_types/tests/inc/**/*.rs`) to use the new names and attributes.
    *   **Verification Strategy:** All crates compile (`cargo check --all-targets`). Grep for old terms ("Former", "former") yields no results in relevant code/doc contexts. Run tests (`cargo test --all-targets`) - expect many failures due to changed names in tests/examples, but compilation should pass.

*   ⚫ **Increment 3: `component_model_types` Refinement (Part 1: Core Traits & Structs)**
    *   **Goal:** Refine core traits and structs (`definition.rs`, `forming.rs`, `storage.rs`, `component.rs`), focusing on documentation, codestyle, and clippy lints.
    *   **Rationale:** Solidify the foundation types before refining collections and macros.
    *   **Detailed Steps:**
        *   **File:** `src/definition.rs`
            *   Review/update module documentation.
            *   Review/update docs for `EntityToDefinition`, `EntityToDefinitionTypes`, `EntityToFormer`, `EntityToStorage`, `FormerDefinitionTypes`, `FormerDefinition` (using new names). Explain purpose and relationships clearly in the context of the component model.
            *   Apply strict codestyle rules (spacing, newlines, etc.).
            *   Run clippy and address warnings for this file.
        *   **File:** `src/forming.rs`
            *   Review/update module documentation.
            *   Review/update docs for `FormerMutator`, `FormingEnd`, `ReturnPreformed`, `ReturnStorage`, `NoEnd`, `FormingEndClosure`, `FormerBegin` (using new names). Explain purpose clearly. Clarify `FormerMutator` vs `FormingEnd`.
            *   Apply strict codestyle rules.
            *   Run clippy and address warnings for this file.
        *   **File:** `src/storage.rs`
            *   Review/update module documentation.
            *   Review/update docs for `Storage`, `StoragePreform`. Explain `Preformed` type clearly.
            *   Apply strict codestyle rules.
            *   Run clippy and address warnings for this file.
        *   **File:** `src/component.rs`
            *   Review/update module documentation.
            *   Review/update docs for `Assign`, `OptionExt`, `AssignWithType`. Ensure examples are clear and focused.
            *   Apply strict codestyle rules.
            *   Run clippy and address warnings for this file.
    *   **Verification Strategy:** Crate compiles (`cargo check --package component_model_types`). Clippy passes for modified files (`cargo clippy --package component_model_types`). Documentation is clear and accurate. Codestyle rules are met.

*   ⚫ **Increment 4: `component_model_types` Refinement (Part 2: Collections)**
    *   **Goal:** Refine collection-related traits and implementations (`collection.rs`, `collection/*.rs`), focusing on documentation, codestyle, and clippy lints.
    *   **Rationale:** Ensure collection handling is robust and well-documented.
    *   **Detailed Steps:**
        *   **File:** `src/collection.rs`
            *   Review/update module documentation.
            *   Review/update docs for `EntryToVal`, `CollectionValToEntry`, `ValToEntry`, `Collection`, `CollectionAdd`, `CollectionAssign`, `CollectionFormer`. Explain purpose clearly.
            *   Apply strict codestyle rules.
            *   Run clippy and address warnings for this file.
        *   **Files:** `src/collection/*.rs` (for each collection type)
            *   Review/update file-level documentation.
            *   Review/update docs for `*Definition`, `*DefinitionTypes`, `*Former` alias, `*Ext` trait (using new names).
            *   Ensure `*Ext` trait method uses the chosen constructor name (e.g., `component_model()`).
            *   Apply strict codestyle rules.
            *   Run clippy and address warnings for each file.
    *   **Verification Strategy:** Crate compiles (`cargo check --package component_model_types`). Clippy passes for modified files (`cargo clippy --package component_model_types`). Documentation is clear and accurate. Codestyle rules are met.

*   ⚫ **Increment 5: `component_model_meta` Refinement (Part 1: Component Derives)**
    *   **Goal:** Refine the component derive implementations (`Assign`, `ComponentFrom`, `ComponentsAssign`, `FromComponents`), focusing on documentation, codestyle, and clippy lints.
    *   **Rationale:** Ensure the simpler component derives are clean before tackling the main `ComponentModel` derive.
    *   **Detailed Steps:**
        *   **Files:** `src/component/*.rs`
            *   Review/update file-level and function/struct documentation. Explain the purpose and usage of each derive clearly.
            *   Apply strict codestyle rules (including generated code in `quote!`).
            *   Remove temporary comments.
            *   Run clippy and address warnings for these files.
            *   Ensure generated code uses correct paths to `component_model_types`.
    *   **Verification Strategy:** Crate compiles (`cargo check --package component_model_meta`). Clippy passes for modified files (`cargo clippy --package component_model_meta`). Documentation is clear. Codestyle rules are met.

*   ⚫ **Increment 6: `component_model_meta` Refinement (Part 2: `ComponentModel` Derive - Setup & Attributes)**
    *   **Goal:** Refine the setup, attribute parsing, and initial dispatch logic for the main `ComponentModel` derive.
    *   **Rationale:** Clean up the entry point and attribute handling for the core derive.
    *   **Detailed Steps:**
        *   **File:** `src/derive_component_model.rs` (renamed from `derive_former.rs`)
            *   Review/update file-level documentation.
            *   Review/update docs for the main derive function (e.g., `component_model`).
            *   Review/update docs for `mutator` helper function.
            *   Review/update docs for `doc_generate` helper function.
            *   Apply strict codestyle rules.
            *   Remove temporary comments.
            *   Run clippy and address warnings.
        *   **File:** `src/derive_component_model/struct_attrs.rs`
            *   Review/update file-level documentation.
            *   Review/update docs for `ItemAttributes` and its fields/methods.
            *   Review/update docs for `Attribute*` structs/types defined here (`StorageFields`, `Mutator`, `Perform`, `StandaloneConstructors`). Ensure names and keywords are consistent with the global rename.
            *   Apply strict codestyle rules.
            *   Remove temporary comments.
            *   Run clippy and address warnings.
        *   **File:** `src/derive_component_model/field_attrs.rs`
            *   Review/update file-level documentation.
            *   Review/update docs for `FieldAttributes` and its fields/methods.
            *   Review/update docs for `Attribute*` structs/types defined here (`Config`, `ScalarSetter`, `Subform*Setter`, `ArgForConstructor`). Ensure names and keywords are consistent (e.g., `#[component_model(...)]` keyword, `subform_*` keywords kept).
            *   Apply strict codestyle rules.
            *   Remove temporary comments.
            *   Run clippy and address warnings.
    *   **Verification Strategy:** Crate compiles (`cargo check --package component_model_meta`). Clippy passes for modified files. Documentation is clear. Codestyle rules are met.

*   ⚫ **Increment 7: `component_model_meta` Refinement (Part 3: `ComponentModel` Derive - Struct Logic)**
    *   **Goal:** Refine the code generation logic for structs within the `ComponentModel` derive.
    *   **Rationale:** Ensure struct handling is clean, correct, and uses updated types/names.
    *   **Detailed Steps:**
        *   **File:** `src/derive_component_model/component_model_struct.rs` (renamed from `former_struct.rs`)
            *   Review/update file-level documentation.
            *   Review/update docs for `component_model_for_struct` function.
            *   Go through the function logic step-by-step:
                *   Ensure all generated identifiers (`*ComponentModel`, `*ComponentModelStorage`, etc.) use the new naming convention.
                *   Ensure all references to types/traits from `component_model_types` use the new names (e.g., `component_model_types::ComponentModelMutator`).
                *   Apply strict codestyle rules to all generated code within `quote!` blocks.
                *   Update documentation comments within the generated code (e.g., for the `component_model()` method, the `*ComponentModel` struct).
                *   Remove temporary comments.
                *   Address clippy warnings within this file's logic.
        *   **File:** `src/derive_component_model/field/mod.rs` (and its submodules `preform.rs`, `setter_*.rs`)
            *   Review/update file-level documentation for `mod.rs` and submodules.
            *   Review/update docs for `FormerField` (rename to `ComponentModelField`?) and its methods.
            *   Go through the logic in each file:
                *   Ensure generated code uses new naming conventions for types/traits/methods.
                *   Apply strict codestyle rules to generated code.
                *   Update documentation comments within generated code.
                *   Remove temporary comments.
                *   Address clippy warnings.
    *   **Verification Strategy:** Crate compiles (`cargo check --package component_model_meta`). Clippy passes for modified files. Documentation is clear. Codestyle rules are met. Generated code structure looks correct (manual inspection of a `#[debug]` output might be needed).

*   ⚫ **Increment 8: `component_model_meta` Refinement (Part 4: `ComponentModel` Derive - Enum Logic)**
    *   **Goal:** Refine the code generation logic for enums within the `ComponentModel` derive.
    *   **Rationale:** Ensure enum handling is clean, correct, uses updated types/names, and incorporates the context struct refactoring.
    *   **Detailed Steps:**
        *   **File:** `src/derive_component_model/component_model_enum.rs` (renamed from `former_enum.rs`)
            *   Review/update file-level documentation.
            *   Review/update docs for `component_model_for_enum` function.
            *   Refactor the function to use the `EnumVariantHandlerContext` struct defined in the *other* plan (Increment 9 of that plan). This involves:
                *   Defining the `EnumVariantHandlerContext` struct (likely near the top or in `mod.rs`).
                *   Populating the context struct instance within `component_model_for_enum`.
                *   Updating the calls to handler functions to pass the context struct.
            *   Apply strict codestyle rules.
            *   Remove temporary comments.
            *   Address clippy warnings.
        *   **Files:** `src/derive_component_model/component_model_enum/*.rs` (handlers: `unit.rs`, `tuple_zero.rs`, etc.)
            *   Review/update file-level documentation for each handler.
            *   Refactor each handler function (`handle_*_variant`) to accept the `EnumVariantHandlerContext` struct as its primary argument.
            *   Update the function bodies to access necessary data (AST, attributes, generics, output vectors) via the context struct fields.
            *   Ensure all generated code within `quote!` uses the new naming conventions (`*ComponentModel*`, etc.) and correct paths to `component_model_types`.
            *   Apply strict codestyle rules to generated code.
            *   Update documentation comments within generated code.
            *   Remove temporary comments.
            *   Address clippy warnings.
    *   **Verification Strategy:** Crate compiles (`cargo check --package component_model_meta`). Clippy passes for modified files. Documentation is clear. Codestyle rules are met. Refactoring to context struct is complete.

*   ⚫ **Increment 9: `component_model` Refinement (Facade & Re-exports)**
    *   **Goal:** Refine the main user-facing `component_model` crate.
    *   **Rationale:** Ensure the facade is clean, exports the correct items, and has good top-level documentation.
    *   **Detailed Steps:**
        *   **File:** `src/lib.rs`
            *   Review/update crate-level documentation. Ensure it points to the new `Readme.md`.
            *   Review `own`, `orphan`, `exposed`, `prelude` modules. Ensure they re-export the *correct* items (with new names) from `component_model_types` and `component_model_meta`. Remove unnecessary re-exports.
            *   Apply strict codestyle rules.
            *   Run clippy and address warnings.
        *   **File:** `Readme.md`
            *   Write clear, concise documentation explaining the component model concept and the purpose of this crate ecosystem.
            *   Provide minimal, correct usage examples for the main derives (`ComponentModel`, `Assign`, `ComponentFrom`, etc.).
            *   Explain feature flags.
            *   Link to `advanced.md` (once written) and examples.
        *   **File:** `Cargo.toml`
            *   Final review of metadata (`description`, `keywords`, `categories`, `documentation`, `repository`, `homepage`).
            *   Final review of feature flags and dependencies.
    *   **Verification Strategy:** Crate compiles (`cargo check --package component_model`). Documentation is clear and accurate. Re-exports are correct. Clippy passes.

*   ⚫ **Increment 10: Examples Refinement**
    *   **Goal:** Ensure all examples are minimal, focused, correct, well-documented, and demonstrate core `component_model` features.
    *   **Rationale:** Examples are crucial for user understanding and adoption.
    *   **Detailed Steps:**
        *   Review all files in `component_model/examples/`.
        *   For each example:
            *   Verify it uses the final naming conventions and API.
            *   Ensure it demonstrates a specific feature clearly and concisely. Remove unrelated complexity.
            *   Add or improve documentation comments explaining the example's purpose and code.
            *   Apply strict codestyle rules.
            *   Remove temporary comments.
            *   Ensure it compiles and runs correctly (`cargo run --example ...`).
        *   Remove any examples that are redundant, overly complex, or irrelevant to the core component model features.
        *   Add new minimal examples if core features (like each component derive) are not adequately demonstrated.
        *   Update `component_model/examples/readme.md` to accurately list and describe the final set of examples.
    *   **Verification Strategy:** All examples compile and run. Examples are clear, concise, and well-documented. `examples/readme.md` is accurate.

*   ⚫ **Increment 11: Tests Refinement**
    *   **Goal:** Ensure comprehensive test coverage, update tests to reflect final API, and remove irrelevant tests.
    *   **Rationale:** Guarantee correctness and prevent regressions.
    *   **Detailed Steps:**
        *   Review all tests in `component_model/tests/inc/`.
        *   Update tests to use the final naming conventions and API (e.g., `component_model()`, `ComponentModel`, `#[component_model(...)]`).
        *   Remove tests that are redundant or were specific to `former` features not present or relevant in `component_model`.
        *   Add tests for core component model functionality if coverage is lacking (e.g., specific scenarios for `Assign`, `ComponentFrom`, etc.).
        *   Ensure tests for derive macros (`ComponentModel`, component derives) cover various struct/enum types, generics, attributes, and edge cases.
        *   Apply strict codestyle rules to test code.
        *   Remove temporary comments from tests.
    *   **Verification Strategy:** All tests pass (`cargo test --all-targets` in workspace or relevant crates). Test coverage is adequate for core features.

*   ⚫ **Increment 12: Documentation Overhaul (`advanced.md` & API Docs)**
    *   **Goal:** Create comprehensive advanced documentation and ensure high-quality API documentation.
    *   **Rationale:** Provide users with in-depth information and a good reference.
    *   **Detailed Steps:**
        *   **File:** `component_model/advanced.md`
            *   Write new content explaining advanced concepts relevant to the *component model* crates. This might include:
                *   Detailed explanation of `Assign`, `ComponentFrom`, `ComponentsAssign`, `FromComponents` derives and use cases.
                *   If `ComponentModel` (Former) derive remains: Explain its attributes (`#[component_model(default=...)]`, `#[storage_fields]`, `#[mutator]`, `#[perform]`, `#[subform_*]`, `#[scalar]`, `#[standalone_constructors]`, `#[arg_for_constructor]`) with clear examples using the final naming.
                *   Explain core concepts like Storage, Definition, Context, End, Mutator (using new names).
                *   Explain custom collections integration.
                *   Explain custom definitions/end handlers.
        *   **API Docs:**
            *   Review all public items (structs, enums, traits, functions, macros) across all three crates.
            *   Ensure all public items have clear, concise, and accurate documentation comments (`///` or `//!`).
            *   Add examples within documentation comments where helpful (`#[doc = include_str!(...)]` or ```rust ... ``` blocks).
            *   Ensure documentation uses the final naming conventions.
        *   **READMEs:** Perform a final review of all `Readme.md` files for consistency and clarity.
    *   **Verification Strategy:** `advanced.md` is comprehensive and accurate. API documentation is complete and renders correctly (`cargo doc --open`). READMEs are consistent.

*   ⚫ **Increment 13: Final Polish & Release Preparation**
    *   **Goal:** Address all remaining issues, ensure consistency, and prepare for a potential release.
    *   **Rationale:** Final quality check.
    *   **Detailed Steps:**
        *   Run `cargo clippy --all-targets -- -D warnings` across the workspace (or relevant crates) and fix *all* lints/warnings.
        *   Run `cargo fmt --all` to ensure code formatting is consistent.
        *   Run `cargo test --all-targets` one last time.
        *   Test building/testing with different feature flag combinations (e.g., `no_std`, `use_alloc`, specific derives disabled/enabled) if applicable.
        *   Review `Cargo.toml` files for final version numbers, author lists, licenses, repository links, etc.
        *   Remove any remaining temporary files or comments.
    *   **Verification Strategy:** All checks pass (clippy, fmt, test, features). Codebase is clean and consistent. Metadata is accurate.

## Notes & Insights

*   **Decision Point:** The most critical decision is the core terminology (`Former` vs. `ComponentModel` vs. something else). This needs to be made in Increment 2. The rest of the plan assumes `ComponentModel` is chosen, but can be adapted.
*   **Subform Naming:** Decided to keep `subform_*` attribute names as they describe the *mechanism*, but rename generated types/methods related to them (e.g., `ParentSubformScalarChildEnd` -> `ParentSubcomponentScalarChildEnd`).
*   **Attribute Renaming:** Decided to rename `#[former(...)]` to `#[component_model(...)]` for consistency.
*   **Testing Strategy:** Tests related to derive macros should primarily reside in `component_model_meta` or `component_model` (integration tests), while `component_model_types` tests should focus on the traits themselves. Existing tests need careful migration/adaptation.
*   **Documentation Focus:** Documentation needs a significant shift from a "builder pattern" focus (former) to a "component model / type-based assignment" focus, potentially still including the builder pattern as one application.
