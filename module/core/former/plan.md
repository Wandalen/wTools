# Project Plan: Refactor Enum Variant Handling in Former Derive

## Initial Task

Refactor the `former_for_enum` function in `former_meta/src/derive_former/former_enum.rs` to improve readability, maintainability, and testability. Extract the logic for handling each distinct variant case (Unit, Tuple(0/N), Struct(0/N)) into its own dedicated handler function within a new submodule (`former_meta/src/derive_former/former_enum/`). Ensure the refactoring adheres strictly to the documented "Enum Variant Handling Rules" and passes all relevant tests. Fix any existing test failures in the `former` crate as a prerequisite.

**Refactoring Principles:**

*   **Reuse Existing Patterns:** All refactoring steps must prioritize reusing existing code structures, helper functions, and patterns already present within the `former_meta` crate and the broader `former` ecosystem (`macro_tools`, `former_types`).
*   **Minimal Necessary Changes:** Implement the context struct refactoring by making only the essential modifications to achieve the goal. Avoid unnecessary restructuring or logic changes within the handlers beyond adapting them to use the context struct.

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

*   ✅ **Increment 1: Diagnose and fix current test failures in the `former` crate.**
    *   Detailed Plan Step 1: Execute `cargo test` within the `module/core/former` crate directory to capture the current test failures and error messages.
    *   Detailed Plan Step 2: Analyze the `cargo test` output critically, focusing on the specific errors, failing test names, and code locations. Pay attention to potential issues related to the recent `WhereClause` fix or the partially refactored state (skipped/stuck increments).
    *   Detailed Plan Step 3: Based on the analysis, identify the root cause(s) of the failures.
    *   Detailed Plan Step 4: Propose and implement code changes in the relevant files (likely within `former_meta` or `former` test files) to address the identified issues. (This might involve multiple sub-steps depending on the errors).
    *   **Rule Adherence Checkpoint:** Confirm strict adherence to `code/gen` instructions, Design Rules, and **especially Codestyle Rules (overriding existing style)** during implementation.
    *   **Crucial Design Rules:** [Error Handling: Use a Centralized Approach](#error-handling-use-a-centralized-approach), [Testing: Avoid Writing Automated Tests Unless Asked](#testing-avoid-writing-tests-unless-asked) (focus on fixing existing tests).
    *   **Verification Strategy:** Compile checks (`cargo check --package former_meta`). Ensure the module structure is recognized without errors.
*   ✅ **Increment 2: Create submodule structure `former_meta/src/derive_former/former_enum/`**
    *   Detailed Plan Step 1: Create the directory `module/core/former_meta/src/derive_former/former_enum`.
    *   Detailed Plan Step 2: Create the file `module/core/former_meta/src/derive_former/former_enum/mod.rs`.
    *   Detailed Plan Step 3: Add `mod unit; pub(super) use unit::*;` etc. lines within `former_enum/mod.rs` for all planned handler modules.
    *   Detailed Plan Step 4: Add `mod former_enum;` to `module/core/former_meta/src/derive_former.rs`.
    *   **Rule Adherence Checkpoint:** Confirm strict adherence to `code/gen` instructions, Design Rules, and **especially Codestyle Rules (overriding existing style)** during implementation. Ensure no semantic changes.
    *   **Crucial Design Rules:** [Structuring: Organize by Feature or Layer](#structuring-organize-by-feature-or-layer), [Structuring: Add Module Declaration Before Content](#structuring-add-module-declaration-before-content).
    *   **Verification Strategy:** Compile checks (`cargo check --package former_meta`). Ensure the module structure is recognized without errors.
*   ✅ **Increment 3: Extract handler for Unit variants (`handle_unit_variant`)**
    *   Detailed Plan Step 1: Create file `module/core/former_meta/src/derive_former/former_enum/unit.rs`.
    *   Detailed Plan Step 2: Define the `pub(super) fn handle_unit_variant(...) -> Result<()>` function signature, accepting necessary parameters (ast, variant, attrs, names, generics, etc.).
    *   Detailed Plan Step 3: Move the code block handling `syn::Fields::Unit` from `former_enum.rs` into `handle_unit_variant`.
    *   Detailed Plan Step 4: Integrate logic for `#[standalone_constructors]` within `handle_unit_variant` for unit variants.
    *   Detailed Plan Step 5: Update the `match variant.fields` arm for `syn::Fields::Unit` in `former_enum.rs` to call `handle_unit_variant`.
    *   **Rule Adherence Checkpoint:** Confirm strict adherence to `code/gen` instructions, Design Rules, and **especially Codestyle Rules (overriding existing style)** during implementation. Ensure no semantic changes.
    *   **Crucial Design Rules:** [Structuring: Organize by Feature or Layer](#structuring-organize-by-feature-or-layer), [Visibility: Keep Implementation Details Private](#visibility-keep-implementation-details-private).
    *   **Verification Strategy:** Compile checks (`cargo check --package former_meta`). Run enum tests (`cargo test --package former --test tests`). **Analyze logs critically**. Ensure tests related to unit variants still pass and no regressions occurred.
*   ✅ **Increment 4: Extract handler for Tuple variants with zero fields (`handle_tuple_zero_variant`)**
    *   Detailed Plan Step 1: Create file `module/core/former_meta/src/derive_former/former_enum/tuple_zero.rs`.
    *   Detailed Plan Step 2: Define `pub(super) fn handle_tuple_zero_variant(...) -> Result<()>` function signature.
    *   Detailed Plan Step 3: Move the code block handling `syn::Fields::Unnamed` with `len() == 0` from `former_enum.rs` into `handle_tuple_zero_variant`.
    *   Detailed Plan Step 4: Integrate logic for `#[standalone_constructors]` within `handle_tuple_zero_variant`.
    *   Detailed Plan Step 5: Update the `match fields.unnamed.len()` arm for `0` in `former_enum.rs` to call `handle_tuple_zero_variant`.
    *   **Rule Adherence Checkpoint:** Confirm strict adherence to `code/gen` instructions, Design Rules, and **especially Codestyle Rules (overriding existing style)** during implementation. Ensure minimal necessary changes.
    *   **Crucial Design Rules:** [Structuring: Organize by Feature or Layer](#structuring-organize-by-feature-or-layer), [Visibility: Keep Implementation Details Private](#visibility-keep-implementation-details-private).
    *   **Verification Strategy:** Compile checks (`cargo check --package former_meta`). Run enum tests (`cargo test --package former --test tests`). **Analyze logs critically**. Ensure tests related to zero-field tuple variants still pass.
*   ✅ **Increment 5: Extract handler for Struct variants with zero fields (`handle_struct_zero_variant`)**
    *   Detailed Plan Step 1: Create file `module/core/former_meta/src/derive_former/former_enum/struct_zero.rs`.
    *   Detailed Plan Step 2: Define `pub(super) fn handle_struct_zero_variant(...) -> Result<()>` function signature.
    *   Detailed Plan Step 3: Move the code block handling `syn::Fields::Named` with `len() == 0` from `former_enum.rs` into `handle_struct_zero_variant`.
    *   Detailed Plan Step 4: Integrate logic for `#[standalone_constructors]` within `handle_struct_zero_variant`.
    *   Detailed Plan Step 5: Update the `match fields.named.len()` arm for `0` in `former_enum.rs` to call `handle_struct_zero_variant`.
    *   **Rule Adherence Checkpoint:** Confirm strict adherence to `code/gen` instructions, Design Rules, and **especially Codestyle Rules (overriding existing style)** during implementation. Ensure minimal necessary changes.
    *   **Crucial Design Rules:** [Structuring: Organize by Feature or Layer](#structuring-organize-by-feature-or-layer), [Visibility: Keep Implementation Details Private](#visibility-keep-implementation-details-private).
    *   **Verification Strategy:** Compile checks (`cargo check --package former_meta`). Run enum tests (`cargo test --package former --test tests`). **Analyze logs critically**. Ensure tests related to zero-field struct variants still pass.
*   ✅ **Increment 6: Extract handler for Tuple variants with non-zero fields (`handle_tuple_non_zero_variant`)** (Revisit skipped increment)
    *   Detailed Plan Step 1: Create file `module/core/former_meta/src/derive_former/former_enum/tuple_non_zero.rs`.
    *   Detailed Plan Step 2: Define `pub(super) fn handle_tuple_non_zero_variant(...) -> Result<()>` function signature.
    *   Detailed Plan Step 3: Move the code block handling `syn::Fields::Unnamed` with `len() >= 1` from `former_enum.rs` into `handle_tuple_non_zero_variant`.
    *   Detailed Plan Step 4: Integrate logic for `#[standalone_constructors]` within `handle_tuple_non_zero_variant`.
    *   Detailed Plan Step 5: Update the `match fields.unnamed.len()` arm for `_` (or `1..`) in `former_enum.rs` to call `handle_tuple_non_zero_variant`.
    *   **Rule Adherence Checkpoint:** Confirm strict adherence to `code/gen` instructions, Design Rules, and **especially Codestyle Rules (overriding existing style)** during implementation. Ensure no semantic changes. Pay attention to the `WhereClause` handling fix noted previously.
    *   **Crucial Design Rules:** [Structuring: Organize by Feature or Layer](#structuring-organize-by-feature-or-layer), [Visibility: Keep Implementation Details Private](#visibility-keep-implementation-details-private), [Handling Panics vs Recoverable Errors](#handling-panics-vs-recoverable-errors) (for attribute misuse).
    *   **Verification Strategy:** Compile checks (`cargo check --package former_meta`). Run enum tests (`cargo test --package former --test tests`). **Analyze logs critically**. Ensure tests related to non-zero-field tuple variants pass.
*   ❌ **Increment 15: Refactor `handle_struct_non_zero_variant` to use context struct.** (New)
    *   **Goal:** Adapt the `handle_struct_non_zero_variant` function.
    *   **Rationale:** Implement the new handler signature.
    *   **Detailed Steps:**
        *   Modify `handle_struct_non_zero_variant` in `former_meta/src/derive_former/former_enum/struct_non_zero.rs`.
        *   Change signature to accept `ctx: &mut EnumVariantHandlerContext<'_>`.
        *   Update body to access data via `ctx`.
        *   **Minimal Change:** Adapt data access; keep core logic. **Fix pre-existing compilation errors identified in Increment 14 verification.**
    *   **Rule Adherence Checkpoint:** Confirm strict adherence to `code/gen` instructions, Design Rules, and **especially Codestyle Rules (overriding existing style)** during implementation. Ensure minimal necessary changes.
    *   **Crucial Design Rules:** Code clarity, maintainability.
    *   **Verification Strategy:** Compile checks (`cargo check --package former_meta`). Run enum tests (`cargo test --package former --test tests`). **Analyze logs critically**. Ensure tests still pass after all handlers are refactored.
*   ⚫ **Increment 16: Verify `standalone_constructors` logic.** (Was 9)
    *   Detailed Plan Step 1: Review the implementation of standalone constructor generation within each handler function (now accessed via the context struct).
    *   Detailed Plan Step 2: Ensure the logic correctly handles the `#[standalone_constructors]` struct attribute and the `#[arg_for_constructor]` field attribute according to the "Option 2" rules (return `Self` if all fields are args, otherwise return `Former`).
    *   Detailed Plan Step 3: Manually inspect generated code snippets (using `#[debug]` if necessary) for a few representative enum variants to confirm correctness.
    *   **Rule Adherence Checkpoint:** Confirm strict adherence to `code/gen` instructions, Design Rules, and **especially Codestyle Rules (overriding existing style)** during implementation. Ensure no semantic changes.
    *   **Crucial Design Rules:** Correctness, adherence to specified constructor logic.
    *   **Verification Strategy:** Compile checks (`cargo check --package former_meta`). Run tests specifically targeting standalone constructors (`cargo test --package former --test tests` - assuming such tests exist or are added). **Analyze logs critically**.
*   ⚫ **Increment 17: Apply strict codestyle, remove temporary comments, address clippy warnings, add documentation.** (Updated)
    *   Detailed Plan Step 1: Run `cargo clippy --package former_meta --fix --allow-dirty` to automatically fix simpler lints, focusing on the `former_enum` module.
    *   Detailed Plan Step 2: Review remaining `cargo clippy --package former_meta` warnings for the `former_enum` module and manually address them, ensuring adherence to codestyle and design rules.
    *   Detailed Plan Step 3: Review all refactored files (`former_enum.rs` and handlers in `former_enum/`) for strict adherence to codestyle rules (spacing, newlines, etc.). **Pay special attention to generated code within `quote!` blocks.**
    *   Detailed Plan Step 4: Remove temporary comments (e.g., `// qqq`, `// xxx`, `// FIX:`) from the refactored files. Preserve task comments (`// TODO:`).
    *   Detailed Plan Step 5: Add/update documentation comments for the new `EnumVariantHandlerContext` struct and the refactored handler functions, explaining the context struct approach and rationale.
    *   **Crucial Design Rules:** [Lints and warnings](#lints-and-warnings), [Comments and Documentation](#comments-and-documentation).
    *   **Verification Strategy:** Compile checks (`cargo check --package former_meta`). Clippy passes (`cargo clippy --package former_meta`). Manual code review confirms quality, documentation updates, and comment cleanup.
*   ⚫ **Increment 18: Final review and verification.** (New)
    *   **Goal:** Ensure the entire refactoring is correct and integrated.
    *   **Rationale:** Final check before considering the task complete.
    *   **Detailed Steps:**
        *   Run the full test suite (`cargo test --package former --test tests`).
        *   Perform a final manual review of the changes in the `former_enum` module.
    *   **Verification Strategy:** All enum tests pass. Code review confirms clarity and correctness.

## Notes & Insights

*   *(No notes yet)*
*   **[2025-04-29] Skipped Increment:** Increment 5 (Extract handler for Tuple variants with non-zero fields) was skipped due to persistent issues with applying automated changes to `module/core/former_meta/src/derive_former/former_enum.rs`. Manual intervention is required to complete this increment.
*   **[2025-04-29] Stuck in Increment 6:** Encountered persistent compilation errors after moving code into `handle_struct_non_zero_variant`. Initiating Stuck Resolution Process.
*   **[2025-04-29] Hypotheses for Increment 6:**
    *   Hypothesis 1: The generated `Storage` struct or its implementations contain a brace mismatch or syntax error.
    *   Hypothesis 2: The generated `DefinitionTypes` struct or its implementations contain a brace mismatch or syntax error.
    *   Hypothesis 3: The generated `Definition` struct or its implementations contain a brace mismatch or syntax error.
    *   Hypothesis 4: The generated `Former` struct contains a brace mismatch or syntax error.
*   **[2025-04-30/Increment 14] Verification Failure:** `cargo check --package former_meta` failed due to pre-existing errors in `module/core/former_meta/src/derive_former/former_enum/struct_non_zero.rs`. These errors are unrelated to the changes in Increment 14 and will be addressed in Increment 15.
*   **[2025-05-01/Increment 15] Stuck Point:** Encountered persistent compilation errors (E0277: the trait bound `former_enum::EnumVariantHandlerContext<'a>: macro_tools::quote::ToTokens` is not satisfied) when refactoring `handle_struct_non_zero_variant` to use the context struct within `quote!` macros. This indicates an issue with correctly interpolating fields from the context struct. Status: Unresolved.

## Hypotheses for Increment 15 Stuck Point

*   Hypothesis 1: I am incorrectly interpolating the entire `ctx` variable within `quote!` instead of just the required fields (like `ctx.vis`).
*   Hypothesis 2: The `quote!` macro syntax for interpolating fields from a struct variable is different than I am currently using.
*   Hypothesis 3: There is an issue with the `EnumVariantHandlerContext` struct definition itself that prevents its fields from being correctly interpolated by `quote!`.
