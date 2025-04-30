# Project Plan: Refactor Enum Variant Handling in Former Derive

## Initial Task

Refactor the `former_for_enum` function in `former_meta/src/derive_former/former_enum.rs` to improve readability, maintainability, and testability. Extract the logic for handling each distinct variant case (Unit, Tuple(0/N), Struct(0/N)) into its own dedicated handler function within a new submodule (`former_meta/src/derive_former/former_enum/`). Ensure the refactoring adheres strictly to the documented "Enum Variant Handling Rules" and passes all relevant tests. Fix any existing test failures in the `former` crate as a prerequisite.

**Enum Variant Handling Rules (Specification):**

*   **`#[scalar]` Attribute:**
    *   Unit Variant: `Enum::variant() -> Enum` (Handler: `unit`)
    *   Tuple(0) Variant: `Enum::variant() -> Enum` (Handler: `tuple_zero`)
    *   Struct(0) Variant: `Enum::variant {} -> Enum` (Handler: `struct_zero`)
    *   Tuple(1) Variant: `Enum::variant(InnerType) -> Enum` (Handler: `tuple_non_zero`)
    *   Struct(1) Variant: `Enum::variant { field: InnerType } -> Enum` (Handler: `struct_non_zero`)
    *   Tuple(N) Variant: `Enum::variant(T1, T2, ...) -> Enum` (Handler: `tuple_non_zero`)
    *   Struct(N) Variant: `Enum::variant { f1: T1, f2: T2, ... } -> Enum` (Handler: `struct_non_zero`)
    *   Error: Cannot be combined with `#[subform_scalar]`.
*   **`#[subform_scalar]` Attribute:**
    *   Unit Variant: Error (Handler: `unit`)
    *   Tuple(0)/Struct(0) Variant: Error (Handlers: `tuple_zero`, `struct_zero`)
    *   Tuple(1) Variant: `Enum::variant() -> InnerFormer<...>` (Requires path type deriving `Former`) (Handler: `tuple_non_zero`)
    *   Struct(1)/Struct(N) Variant: `Enum::variant() -> VariantFormer<...>` (Implicit former) (Handler: `struct_non_zero`)
    *   Tuple(N) Variant: Error (Handler: `tuple_non_zero`)
*   **Default Behavior (No Attribute):**
    *   Unit Variant: `Enum::variant() -> Enum` (Handler: `unit`)
    *   Tuple(0) Variant: `Enum::variant() -> Enum` (Handler: `tuple_zero`)
    *   Struct(0) Variant: Error (Requires `#[scalar]`) (Handler: `struct_zero`)
    *   Tuple(1) Variant: `Enum::variant() -> InnerFormer<...>` (Requires path type deriving `Former`) (Handler: `tuple_non_zero`)
    *   Struct(1)/Struct(N) Variant: `Enum::variant() -> VariantFormer<...>` (Implicit former) (Handler: `struct_non_zero`)
    *   Tuple(N) Variant: `Enum::variant(T1, T2, ...) -> Enum` (Like `#[scalar]`) (Handler: `tuple_non_zero`)
*   **`#[standalone_constructors]` Attribute:** Generates top-level constructors based on the above rules and `#[arg_for_constructor]` on fields *within* variants. Logic to be integrated into each handler.

## Increments

*   ⏳ **Increment 1: Diagnose and fix current test failures in the `former` crate.**
    *   Detailed Plan Step 1: Execute `cargo test` within the `module/core/former` crate directory to capture the current test failures and error messages.
    *   Detailed Plan Step 2: Analyze the `cargo test` output critically, focusing on the specific errors, failing test names, and code locations. Pay attention to potential issues related to the recent `WhereClause` fix or the partially refactored state (skipped/stuck increments).
    *   Detailed Plan Step 3: Based on the analysis, identify the root cause(s) of the failures.
    *   Detailed Plan Step 4: Propose and implement code changes in the relevant files (likely within `former_meta` or `former` test files) to address the identified issues. (This might involve multiple sub-steps depending on the errors).
    *   **Rule Adherence Checkpoint:** Confirm strict adherence to `code/gen` instructions, Design Rules, and **especially Codestyle Rules (overriding existing style)** during implementation.
    *   **Crucial Design Rules:** [Error Handling: Use a Centralized Approach](#error-handling-use-a-centralized-approach), [Testing: Avoid Writing Automated Tests Unless Asked](#testing-avoid-writing-tests-unless-asked) (focus on fixing existing tests).
    *   **Verification Strategy:** Run `cargo test` within the `module/core/former` crate directory. **Analyze logs critically**. Ensure all tests pass.
*   ⚫ **Increment 2: Create submodule structure `former_meta/src/derive_former/former_enum/`**
    *   Detailed Plan Step 1: Create the directory `module/core/former_meta/src/derive_former/former_enum`.
    *   Detailed Plan Step 2: Create the file `module/core/former_meta/src/derive_former/former_enum/mod.rs`.
    *   Detailed Plan Step 3: Add `mod unit; pub(super) use unit::*;` etc. lines within `former_enum/mod.rs` for all planned handler modules.
    *   Detailed Plan Step 4: Add `mod former_enum;` to `module/core/former_meta/src/derive_former/mod.rs`.
    *   **Rule Adherence Checkpoint:** Confirm strict adherence to `code/gen` instructions, Design Rules, and **especially Codestyle Rules (overriding existing style)** during implementation.
    *   **Crucial Design Rules:** [Structuring: Organize by Feature or Layer](#structuring-organize-by-feature-or-layer), [Structuring: Add Module Declaration Before Content](#structuring-add-module-declaration-before-content).
    *   **Verification Strategy:** Compile checks (`cargo check --package former_meta`). Ensure the module structure is recognized without errors.
*   ⚫ **Increment 3: Extract handler for Unit variants (`handle_unit_variant`)**
    *   Detailed Plan Step 1: Create file `module/core/former_meta/src/derive_former/former_enum/unit.rs`.
    *   Detailed Plan Step 2: Define the `pub(super) fn handle_unit_variant(...) -> Result<()>` function signature, accepting necessary parameters (ast, variant, attrs, names, generics, etc.).
    *   Detailed Plan Step 3: Move the code block handling `syn::Fields::Unit` from `former_enum.rs` into `handle_unit_variant`.
    *   Detailed Plan Step 4: Integrate logic for `#[standalone_constructors]` within `handle_unit_variant` for unit variants.
    *   Detailed Plan Step 5: Update the `match variant.fields` arm for `syn::Fields::Unit` in `former_enum.rs` to call `handle_unit_variant`.
    *   **Rule Adherence Checkpoint:** Confirm strict adherence to `code/gen` instructions, Design Rules, and **especially Codestyle Rules (overriding existing style)** during implementation. Ensure no semantic changes.
    *   **Crucial Design Rules:** [Structuring: Organize by Feature or Layer](#structuring-organize-by-feature-or-layer), [Visibility: Keep Implementation Details Private](#visibility-keep-implementation-details-private).
    *   **Verification Strategy:** Compile checks (`cargo check --package former_meta`). Run enum tests (`cargo test --package former --test former_enum_test`). **Analyze logs critically**. Ensure tests related to unit variants still pass and no regressions occurred.
*   ⚫ **Increment 4: Extract handler for Tuple variants with zero fields (`handle_tuple_zero_variant`)**
    *   Detailed Plan Step 1: Create file `module/core/former_meta/src/derive_former/former_enum/tuple_zero.rs`.
    *   Detailed Plan Step 2: Define `pub(super) fn handle_tuple_zero_variant(...) -> Result<()>` function signature.
    *   Detailed Plan Step 3: Move the code block handling `syn::Fields::Unnamed` with `len() == 0` from `former_enum.rs` into `handle_tuple_zero_variant`.
    *   Detailed Plan Step 4: Integrate logic for `#[standalone_constructors]` within `handle_tuple_zero_variant`.
    *   Detailed Plan Step 5: Update the `match fields.unnamed.len()` arm for `0` in `former_enum.rs` to call `handle_tuple_zero_variant`.
    *   **Rule Adherence Checkpoint:** Confirm strict adherence to `code/gen` instructions, Design Rules, and **especially Codestyle Rules (overriding existing style)** during implementation. Ensure no semantic changes.
    *   **Crucial Design Rules:** [Structuring: Organize by Feature or Layer](#structuring-organize-by-feature-or-layer), [Visibility: Keep Implementation Details Private](#visibility-keep-implementation-details-private).
    *   **Verification Strategy:** Compile checks (`cargo check --package former_meta`). Run enum tests (`cargo test --package former --test former_enum_test`). **Analyze logs critically**. Ensure tests related to zero-field tuple variants still pass.
*   ⚫ **Increment 5: Extract handler for Struct variants with zero fields (`handle_struct_zero_variant`)**
    *   Detailed Plan Step 1: Create file `module/core/former_meta/src/derive_former/former_enum/struct_zero.rs`.
    *   Detailed Plan Step 2: Define `pub(super) fn handle_struct_zero_variant(...) -> Result<()>` function signature.
    *   Detailed Plan Step 3: Move the code block handling `syn::Fields::Named` with `len() == 0` from `former_enum.rs` into `handle_struct_zero_variant`.
    *   Detailed Plan Step 4: Integrate logic for `#[standalone_constructors]` within `handle_struct_zero_variant`.
    *   Detailed Plan Step 5: Update the `match fields.named.len()` arm for `0` in `former_enum.rs` to call `handle_struct_zero_variant`.
    *   **Rule Adherence Checkpoint:** Confirm strict adherence to `code/gen` instructions, Design Rules, and **especially Codestyle Rules (overriding existing style)** during implementation. Ensure no semantic changes.
    *   **Crucial Design Rules:** [Structuring: Organize by Feature or Layer](#structuring-organize-by-feature-or-layer), [Visibility: Keep Implementation Details Private](#visibility-keep-implementation-details-private).
    *   **Verification Strategy:** Compile checks (`cargo check --package former_meta`). Run enum tests (`cargo test --package former --test former_enum_test`). **Analyze logs critically**. Ensure tests related to zero-field struct variants still pass.
*   ⚫ **Increment 6: Extract handler for Tuple variants with non-zero fields (`handle_tuple_non_zero_variant`)** (Revisit skipped increment)
    *   Detailed Plan Step 1: Create file `module/core/former_meta/src/derive_former/former_enum/tuple_non_zero.rs`.
    *   Detailed Plan Step 2: Define `pub(super) fn handle_tuple_non_zero_variant(...) -> Result<()>` function signature.
    *   Detailed Plan Step 3: Move the code block handling `syn::Fields::Unnamed` with `len() >= 1` from `former_enum.rs` into `handle_tuple_non_zero_variant`.
    *   Detailed Plan Step 4: Integrate logic for `#[standalone_constructors]` within `handle_tuple_non_zero_variant`.
    *   Detailed Plan Step 5: Update the `match fields.unnamed.len()` arm for `_` (or `1..`) in `former_enum.rs` to call `handle_tuple_non_zero_variant`.
    *   **Rule Adherence Checkpoint:** Confirm strict adherence to `code/gen` instructions, Design Rules, and **especially Codestyle Rules (overriding existing style)** during implementation. Ensure no semantic changes. Pay attention to the `WhereClause` handling fix noted previously.
    *   **Crucial Design Rules:** [Structuring: Organize by Feature or Layer](#structuring-organize-by-feature-or-layer), [Visibility: Keep Implementation Details Private](#visibility-keep-implementation-details-private), [Handling Panics vs Recoverable Errors](#handling-panics-vs-recoverable-errors) (for attribute misuse).
    *   **Verification Strategy:** Compile checks (`cargo check --package former_meta`). Run enum tests (`cargo test --package former --test former_enum_test`). **Analyze logs critically**. Ensure tests related to non-zero-field tuple variants pass.
*   ⚫ **Increment 7: Extract handler for Struct variants with non-zero fields (`handle_struct_non_zero_variant`)** (Revisit previously stuck increment)
    *   Detailed Plan Step 1: Create file `module/core/former_meta/src/derive_former/former_enum/struct_non_zero.rs`.
    *   Detailed Plan Step 2: Define `pub(super) fn handle_struct_non_zero_variant(...) -> Result<()>` function signature.
    *   Detailed Plan Step 3: Move the code block handling `syn::Fields::Named` with `len() >= 1` from `former_enum.rs` into `handle_struct_non_zero_variant`.
    *   Detailed Plan Step 4: Integrate logic for `#[standalone_constructors]` within `handle_struct_non_zero_variant`.
    *   Detailed Plan Step 5: Update the `match fields.named.len()` arm for `_` (or `1..`) in `former_enum.rs` to call `handle_struct_non_zero_variant`.
    *   **Rule Adherence Checkpoint:** Confirm strict adherence to `code/gen` instructions, Design Rules, and **especially Codestyle Rules (overriding existing style)** during implementation. Ensure no semantic changes. Pay attention to the `WhereClause` handling fix noted previously.
    *   **Crucial Design Rules:** [Structuring: Organize by Feature or Layer](#structuring-organize-by-feature-or-layer), [Visibility: Keep Implementation Details Private](#visibility-keep-implementation-details-private), [Handling Panics vs Recoverable Errors](#handling-panics-vs-recoverable-errors) (for attribute misuse).
    *   **Verification Strategy:** Compile checks (`cargo check --package former_meta`). Run enum tests (`cargo test --package former --test former_enum_test`). **Analyze logs critically**. Ensure tests related to non-zero-field struct variants pass.
*   ⚫ **Increment 8: Refactor main `former_for_enum` function.**
    *   Detailed Plan Step 1: Review the `former_for_enum` function in `former_meta/src/derive_former/former_enum.rs`.
    *   Detailed Plan Step 2: Remove any remaining logic that was moved into handlers.
    *   Detailed Plan Step 3: Ensure the function primarily acts as a dispatcher, parsing top-level attributes and variant information, then calling the appropriate handler based on `variant.fields`.
    *   Detailed Plan Step 4: Clean up any unused variables or imports.
    *   **Rule Adherence Checkpoint:** Confirm strict adherence to `code/gen` instructions, Design Rules, and **especially Codestyle Rules (overriding existing style)** during implementation.
    *   **Crucial Design Rules:** Code clarity, maintainability.
    *   **Verification Strategy:** Compile checks (`cargo check --package former_meta`). Run enum tests (`cargo test --package former --test former_enum_test`). **Analyze logs critically**. Ensure no regressions were introduced during cleanup.
*   ⚫ **Increment 9: Verify `standalone_constructors` logic.**
    *   Detailed Plan Step 1: Review the implementation of standalone constructor generation within each handler function (added in Increments 3-7).
    *   Detailed Plan Step 2: Ensure the logic correctly handles the `#[standalone_constructors]` struct attribute and the `#[arg_for_constructor]` field attribute according to the "Option 2" rules (return `Self` if all fields are args, otherwise return `Former`).
    *   Detailed Plan Step 3: Manually inspect generated code snippets (using `#[debug]` if necessary) for a few representative enum variants to confirm correctness.
    *   **Rule Adherence Checkpoint:** Confirm strict adherence to `code/gen` instructions, Design Rules, and **especially Codestyle Rules (overriding existing style)** during implementation.
    *   **Crucial Design Rules:** Correctness, adherence to specified constructor logic.
    *   **Verification Strategy:** Compile checks (`cargo check --package former_meta`). Run tests specifically targeting standalone constructors (`cargo test --package former --test former_standalone_constructor_test` - assuming such tests exist or are added). **Analyze logs critically**.
*   ⚫ **Increment 10: Final review, cleanup, and documentation updates.**
    *   Detailed Plan Step 1: Run `cargo clippy --package former_meta --fix --allow-dirty` to address lints.
    *   Detailed Plan Step 2: Run the full test suite (`cargo test --all-targets` in workspace root or relevant crates) one last time.
    *   Detailed Plan Step 3: Review all code changes made during the refactoring for clarity, consistency, and adherence to rules.
    *   Detailed Plan Step 4: Update the documentation comments within the refactored code (e.g., the "Refactoring Plan" comment in `former_enum.rs`, comments in handlers).
    *   Detailed Plan Step 5: Check if `Readme.md` or `advanced.md` in the `former` crate need updates (unlikely for this internal refactor, but good practice to check).
    *   **Rule Adherence Checkpoint:** Confirm strict adherence to `code/gen` instructions, Design Rules, and **especially Codestyle Rules (overriding existing style)** during implementation.
    *   **Crucial Design Rules:** [Lints and warnings](#lints-and-warnings), [Comments and Documentation](#comments-and-documentation).
    *   **Verification Strategy:** All tests pass (`cargo test --all-targets`). Clippy passes (`cargo clippy`). Manual code review confirms quality and documentation updates.

## Notes & Insights

*   *(No notes yet)*
*   **[2025-04-29] Skipped Increment:** Increment 5 (Extract handler for Tuple variants with non-zero fields) was skipped due to persistent issues with applying automated changes to `module/core/former_meta/src/derive_former/former_enum.rs`. Manual intervention is required to complete this increment.
*   **[2025-04-29] Stuck in Increment 6:** Encountered persistent compilation errors after moving code into `handle_struct_non_zero_variant`. Initiating Stuck Resolution Process.
*   **[2025-04-29] Hypotheses for Increment 6:**
    *   Hypothesis 1: The generated `Storage` struct or its implementations contain a brace mismatch or syntax error.
    *   Hypothesis 2: The generated `DefinitionTypes` struct or its implementations contain a brace mismatch or syntax error.
    *   Hypothesis 3: The generated `Definition` struct or its implementations contain a brace mismatch or syntax error.
    *   Hypothesis 4: The generated `Former` struct contains a brace mismatch or syntax error.
    *   Hypothesis 5: The issue arises from the combination or interaction of the individually generated components, not the components themselves.
*   **[2025-04-29/Increment 6] Hypothesis Test:** Hypothesis 1: The generated `Storage` struct or its implementations contain a brace mismatch or syntax error. - **Result:** Rejected - **Reasoning:** Manual code review of the `quote!` blocks generating the `Storage` struct and its `impl` blocks (`storage_def`, `storage_default_impl`, `storage_trait_impl`, `storage_preform_impl`) in `struct_non_zero.rs` did not reveal any obvious brace mismatches or syntax errors violating codestyle rules.
*   **[2025-04-29/Increment 6] Hypothesis Test:** Hypothesis 2: The generated `DefinitionTypes` struct or its implementations contain a brace mismatch or syntax error. - **Result:** Rejected - **Reasoning:** Manual code review of the `quote!` blocks generating the `DefinitionTypes` struct and its `impl` blocks (`def_types_struct`, `def_types_default_impl`, `def_types_former_impl`, `def_types_mutator_impl`) in `struct_non_zero.rs` did not reveal any obvious brace mismatches or syntax errors violating codestyle rules.
*   **[2025-04-29/Increment 6] Hypothesis Test:** Hypothesis 3: The generated `Definition` struct or its implementations contain a brace mismatch or syntax error. - **Result:** Rejected - **Reasoning:** Manual code review of the `quote!` blocks generating the `Definition` struct and its `impl` blocks (`def_struct`, `def_default_impl`, `def_former_impl`) in `struct_non_zero.rs` did not reveal any obvious brace mismatches or syntax errors violating codestyle rules.
*   **[2025-04-29/Increment 6] Hypothesis Test:** Hypothesis 4: The generated `Former` struct contains a brace mismatch or syntax error. - **Result:** Rejected - **Reasoning:** Manual code review of the `quote!` block generating the `Former` struct definition (`former_struct_def`) in `struct_non_zero.rs` did not reveal any obvious brace mismatches or syntax errors violating codestyle rules.
*   **[2024-04-30/Increment 6] Fix:** Resolved E0599 compilation errors by changing how `merged_where_clause` is passed to handler functions (passing `Option<&WhereClause>` instead of `Option<&Punctuated<...>>`).
*   **Insight:** Debugging revealed syntax errors in generated code related to comma placement in generic argument lists (e.g., `< () Enum<> >` vs `< (), Enum<> >`) and function signatures (trailing comma in `call` parameters). Careful construction of generic lists in `quote!` is crucial, especially when combining potentially empty enum generics with other parameters. Don't miss comma!
