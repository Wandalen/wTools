# Project Plan: Incrementally Uncomment and Fix Enum Tests in `former` Crate

## Goal

*   Uncomment the `former_enum_tests` module and then incrementally uncomment **groups of related test files** (typically `_derive`, `_manual`, `_only_test` variants for a feature) within `module/core/former/tests/inc/former_enum_tests/`. After uncommenting each group, address any `// xxx :` or `// qqq :` tasks found within those specific files and ensure all their tests pass before proceeding to the next group.

## Context

*   Files to Include in `context.md`:
    *   `module/core/former/tests/inc/mod.rs`
    *   `module/core/former/tests/inc/former_enum_tests/basic_derive.rs`
    *   `module/core/former/tests/inc/former_enum_tests/basic_manual.rs`
    *   `module/core/former/tests/inc/former_enum_tests/basic_only_test.rs`
    *   `module/core/former/tests/inc/former_enum_tests/enum_named_fields_derive.rs`
    *   `module/core/former/tests/inc/former_enum_tests/enum_named_fields_manual.rs`
    *   `module/core/former/tests/inc/former_enum_tests/enum_named_fields_only_test.rs`
    *   `module/core/former/tests/inc/former_enum_tests/generics_independent_struct_derive.rs`
    *   `module/core/former/tests/inc/former_enum_tests/generics_independent_struct_manual.rs`
    *   `module/core/former/tests/inc/former_enum_tests/generics_independent_struct_only_test.rs`
    *   `module/core/former/tests/inc/former_enum_tests/generics_independent_tuple_derive.rs`
    *   `module/core/former/tests/inc/former_enum_tests/generics_independent_tuple_manual.rs`
    *   `module/core/former/tests/inc/former_enum_tests/generics_independent_tuple_only_test.rs`
    *   `module/core/former/tests/inc/former_enum_tests/generics_in_tuple_variant_derive.rs`
    *   `module/core/former/tests/inc/former_enum_tests/generics_in_tuple_variant_manual.rs`
    *   `module/core/former/tests/inc/former_enum_tests/generics_in_tuple_variant_only_test.rs`
    *   `module/core/former/tests/inc/former_enum_tests/generics_shared_struct_derive.rs`
    *   `module/core/former/tests/inc/former_enum_tests/generics_shared_struct_manual.rs`
    *   `module/core/former/tests/inc/former_enum_tests/generics_shared_struct_only_test.rs`
    *   `module/core/former/tests/inc/former_enum_tests/generics_shared_tuple_derive.rs`
    *   `module/core/former/tests/inc/former_enum_tests/generics_shared_tuple_manual.rs`
    *   `module/core/former/tests/inc/former_enum_tests/generics_shared_tuple_only_test.rs`
    *   `module/core/former/tests/inc/former_enum_tests/keyword_variant_derive.rs`
    *   `module/core/former/tests/inc/former_enum_tests/keyword_variant_only_test.rs`
    *   `module/core/former/tests/inc/former_enum_tests/scalar_generic_tuple_derive.rs`
    *   `module/core/former/tests/inc/former_enum_tests/scalar_generic_tuple_manual.rs`
    *   `module/core/former/tests/inc/former_enum_tests/scalar_generic_tuple_only_test.rs`
    *   `module/core/former/tests/inc/former_enum_tests/standalone_constructor_args_derive.rs`
    *   `module/core/former/tests/inc/former_enum_tests/standalone_constructor_args_manual.rs`
    *   `module/core/former/tests/inc/former_enum_tests/standalone_constructor_args_only_test.rs`
    *   `module/core/former/tests/inc/former_enum_tests/standalone_constructor_derive.rs`
    *   `module/core/former/tests/inc/former_enum_tests/standalone_constructor_manual.rs`
    *   `module/core/former/tests/inc/former_enum_tests/standalone_constructor_only_test.rs`
    *   `module/core/former/tests/inc/former_enum_tests/unit_variant_derive.rs`
    *   `module/core/former/tests/inc/former_enum_tests/unit_variant_manual.rs`
    *   `module/core/former/tests/inc/former_enum_tests/unit_variant_only_test.rs`
    *   `module/core/former/tests/inc/former_enum_tests/usecase1.rs`
    *   `module/core/former/tests/inc/former_enum_tests/subform_collection_test.rs`
    *   `module/core/former_meta/src/derive_former/former_enum.rs`
    *   `module/core/former_meta/src/derive_former/field.rs`
    *   `module/core/former_types/src/lib.rs` # (Example: Include key lib files)
    *   `module/core/macro_tools/src/lib.rs` # (Example: Include key lib files)
*   Crates for Documentation in `context.md`:
    *   `former`
    *   `former_meta`
    *   `former_types`
    *   `macro_tools`

## Increments

*   [⚫] **Increment 1:** Uncomment `former_enum_tests` Module Declaration
    *   Detailed Plan Step 1: Modify `module/core/former/tests/inc/mod.rs`. Uncomment *only* the line `mod former_enum_tests;`. Leave all `mod <test_file>;` lines within that block commented.
    *   Detailed Plan Step 2: Request user to run `cargo check --tests --package former` and `cargo test --package former -- former_enum_tests`.
    *   Crucial Design Rules: [Structuring: Add Module Declaration Before Content](#structuring-add-module-declaration-before-content), [Minimal Changes](#enhancements-only-implement-whats-requested)
    *   Verification Strategy: Expect compilation success and zero tests run for `former_enum_tests`. **Analyze logs critically.**

*   [⚫] **Increment 2:** Uncomment and Test Basic Enum (`basic_*`)
    *   Detailed Plan Step 1: Modify `module/core/former/tests/inc/mod.rs`. Uncomment `mod basic_derive;`, `mod basic_manual;`, and `mod basic_only_test;` within `former_enum_tests`.
    *   Detailed Plan Step 2: Read `module/core/former/tests/inc/former_enum_tests/basic_derive.rs`. Address the `// qqq : xxx : uncomment and make it working` task. Propose necessary code changes (if any) and remove the task comment.
    *   Detailed Plan Step 3: Request user to run `cargo check --tests --package former` and `cargo test --package former -- former_enum_tests`.
    *   Detailed Plan Step 4: Analyze results. If failures occur, investigate (potentially in `former_meta` source using `context.md`) and propose fixes. Focus on `basic_derive` and `basic_manual` tests.
    *   Crucial Design Rules: [Proc Macro: Development Workflow](#proc-macro-development-workflow), [Comments: Add Tasks and Label Simplifications](#comments-add-tasks-and-label-simplifications), [Minimal Changes](#enhancements-only-implement-whats-requested)
    *   Verification Strategy: Successful compilation and passing tests for the `basic_derive` and `basic_manual` tests within the `former_enum_tests` module. **Analyze logs critically.**

*   [⚫] **Increment 3:** Uncomment and Test Enum Named Fields (`enum_named_fields_*`)
    *   Detailed Plan Step 1: Modify `module/core/former/tests/inc/mod.rs`. Uncomment `mod enum_named_fields_derive;`, `mod enum_named_fields_manual;`, and `mod enum_named_fields_only_test;`.
    *   Detailed Plan Step 2: Request user to run `cargo check --tests --package former` and `cargo test --package former -- former_enum_tests`.
    *   Detailed Plan Step 3: Analyze results. Propose fixes if needed. Focus on `enum_named_fields_derive` and `enum_named_fields_manual` tests.
    *   Crucial Design Rules: [Proc Macro: Development Workflow](#proc-macro-development-workflow), [Minimal Changes](#enhancements-only-implement-whats-requested)
    *   Verification Strategy: Successful compilation and passing tests for the `enum_named_fields_*` group within `former_enum_tests`. **Analyze logs critically.**

*   [⚫] **Increment 4:** Uncomment and Test Generics Independent Struct (`generics_independent_struct_*`)
    *   Detailed Plan Step 1: Modify `module/core/former/tests/inc/mod.rs`. Uncomment `mod generics_independent_struct_derive;`, `mod generics_independent_struct_manual;`, and `mod generics_independent_struct_only_test;`.
    *   Detailed Plan Step 2: Request user to run `cargo check --tests --package former` and `cargo test --package former -- former_enum_tests`.
    *   Detailed Plan Step 3: Analyze results. Propose fixes if needed (may involve `former_meta` using `context.md`). Focus on `generics_independent_struct_derive` and `generics_independent_struct_manual` tests.
    *   Crucial Design Rules: [Proc Macro: Development Workflow](#proc-macro-development-workflow), [Minimal Changes](#enhancements-only-implement-whats-requested)
    *   Verification Strategy: Successful compilation and passing tests for the `generics_independent_struct_*` group within `former_enum_tests`. **Analyze logs critically.**

*   [⚫] **Increment 5:** Uncomment and Test Generics Independent Tuple (`generics_independent_tuple_*`)
    *   Detailed Plan Step 1: Modify `module/core/former/tests/inc/mod.rs`. Uncomment `mod generics_independent_tuple_derive;`, `mod generics_independent_tuple_manual;`, and `mod generics_independent_tuple_only_test;`.
    *   Detailed Plan Step 2: Read `module/core/former/tests/inc/former_enum_tests/generics_independent_tuple_derive.rs`. Address the `// xxx : qqq : uncomment and make it working` task. Propose necessary code changes and remove the task comment.
    *   Detailed Plan Step 3: Request user to run `cargo check --tests --package former` and `cargo test --package former -- former_enum_tests`.
    *   Detailed Plan Step 4: Analyze results. Propose fixes if needed (may involve `former_meta` using `context.md`). Focus on `generics_independent_tuple_derive` and `generics_independent_tuple_manual` tests.
    *   Crucial Design Rules: [Proc Macro: Development Workflow](#proc-macro-development-workflow), [Comments: Add Tasks and Label Simplifications](#comments-add-tasks-and-label-simplifications), [Minimal Changes](#enhancements-only-implement-whats-requested)
    *   Verification Strategy: Successful compilation and passing tests for the `generics_independent_tuple_*` group within `former_enum_tests`. **Analyze logs critically.**

*   [⚫] **Increment 6:** Uncomment and Test Generics In Tuple Variant (`generics_in_tuple_variant_*`)
    *   Detailed Plan Step 1: Modify `module/core/former/tests/inc/mod.rs`. Uncomment `mod generics_in_tuple_variant_derive;`, `mod generics_in_tuple_variant_manual;`, and `mod generics_in_tuple_variant_only_test;`.
    *   Detailed Plan Step 2: Read `module/core/former/tests/inc/former_enum_tests/generics_in_tuple_variant_derive.rs`. Uncomment the file content if necessary.
    *   Detailed Plan Step 3: Request user to run `cargo check --tests --package former` and `cargo test --package former -- former_enum_tests`.
    *   Detailed Plan Step 4: Analyze results. Propose fixes if needed (may involve `former_meta` using `context.md`). Focus on `generics_in_tuple_variant_derive` and `generics_in_tuple_variant_manual` tests.
    *   Crucial Design Rules: [Proc Macro: Development Workflow](#proc-macro-development-workflow), [Minimal Changes](#enhancements-only-implement-whats-requested)
    *   Verification Strategy: Successful compilation and passing tests for the `generics_in_tuple_variant_*` group within `former_enum_tests`. **Analyze logs critically.**

*   [⚫] **Increment 7:** Uncomment and Test Generics Shared Struct (`generics_shared_struct_*`)
    *   Detailed Plan Step 1: Modify `module/core/former/tests/inc/mod.rs`. Uncomment `mod generics_shared_struct_derive;`, `mod generics_shared_struct_manual;`, and `mod generics_shared_struct_only_test;`.
    *   Detailed Plan Step 2: Read `module/core/former/tests/inc/former_enum_tests/generics_shared_struct_derive.rs`. Uncomment file content if necessary. Address the `// qqq : xxx : uncomment and make it working` task. Propose changes and remove the task comment.
    *   Detailed Plan Step 3: Read `module/core/former/tests/inc/former_enum_tests/generics_shared_struct_manual.rs`. Uncomment file content if necessary. Address the `// xxx : qqq : uncomment and make it working` task. Propose changes and remove the task comment.
    *   Detailed Plan Step 4: Request user to run `cargo check --tests --package former` and `cargo test --package former -- former_enum_tests`.
    *   Detailed Plan Step 5: Analyze results. Propose fixes if needed (may involve `former_meta` using `context.md`). Focus on `generics_shared_struct_derive` and `generics_shared_struct_manual` tests.
    *   Crucial Design Rules: [Proc Macro: Development Workflow](#proc-macro-development-workflow), [Comments: Add Tasks and Label Simplifications](#comments-add-tasks-and-label-simplifications), [Minimal Changes](#enhancements-only-implement-whats-requested)
    *   Verification Strategy: Successful compilation and passing tests for the `generics_shared_struct_*` group within `former_enum_tests`. **Analyze logs critically.**

*   [⚫] **Increment 8:** Uncomment and Test Generics Shared Tuple (`generics_shared_tuple_*`)
    *   Detailed Plan Step 1: Modify `module/core/former/tests/inc/mod.rs`. Uncomment `mod generics_shared_tuple_derive;`, `mod generics_shared_tuple_manual;`, and `mod generics_shared_tuple_only_test;`.
    *   Detailed Plan Step 2: Request user to run `cargo check --tests --package former` and `cargo test --package former -- former_enum_tests`.
    *   Detailed Plan Step 3: Analyze results. Propose fixes if needed (may involve `former_meta` using `context.md`). Focus on `generics_shared_tuple_derive` and `generics_shared_tuple_manual` tests.
    *   Crucial Design Rules: [Proc Macro: Development Workflow](#proc-macro-development-workflow), [Minimal Changes](#enhancements-only-implement-whats-requested)
    *   Verification Strategy: Successful compilation and passing tests for the `generics_shared_tuple_*` group within `former_enum_tests`. **Analyze logs critically.**

*   [⚫] **Increment 9:** Uncomment and Test Keyword Variant (`keyword_variant_*`)
    *   Detailed Plan Step 1: Modify `module/core/former/tests/inc/mod.rs`. Uncomment `mod keyword_variant_derive;` and `mod keyword_variant_only_test;`.
    *   Detailed Plan Step 2: Request user to run `cargo check --tests --package former` and `cargo test --package former -- former_enum_tests`.
    *   Detailed Plan Step 3: Analyze results. Propose fixes if needed. Focus on `keyword_variant_derive` tests.
    *   Crucial Design Rules: [Proc Macro: Development Workflow](#proc-macro-development-workflow), [Minimal Changes](#enhancements-only-implement-whats-requested)
    *   Verification Strategy: Successful compilation and passing tests for the `keyword_variant_*` group within `former_enum_tests`. **Analyze logs critically.**

*   [⚫] **Increment 10:** Uncomment and Test Scalar Generic Tuple (`scalar_generic_tuple_*`)
    *   Detailed Plan Step 1: Modify `module/core/former/tests/inc/mod.rs`. Uncomment `mod scalar_generic_tuple_derive;`, `mod scalar_generic_tuple_manual;`, and `mod scalar_generic_tuple_only_test;`.
    *   Detailed Plan Step 2: Request user to run `cargo check --tests --package former` and `cargo test --package former -- former_enum_tests`.
    *   Detailed Plan Step 3: Analyze results. Propose fixes if needed (may involve `former_meta` using `context.md`). Focus on `scalar_generic_tuple_derive` and `scalar_generic_tuple_manual` tests.
    *   Crucial Design Rules: [Proc Macro: Development Workflow](#proc-macro-development-workflow), [Minimal Changes](#enhancements-only-implement-whats-requested)
    *   Verification Strategy: Successful compilation and passing tests for the `scalar_generic_tuple_*` group within `former_enum_tests`. **Analyze logs critically.**

*   [⚫] **Increment 11:** Uncomment and Test Standalone Constructor Args (`standalone_constructor_args_*`)
    *   Detailed Plan Step 1: Modify `module/core/former/tests/inc/mod.rs`. Uncomment `mod standalone_constructor_args_derive;`, `mod standalone_constructor_args_manual;`, and `mod standalone_constructor_args_only_test;`.
    *   Detailed Plan Step 2: Request user to run `cargo check --tests --package former` and `cargo test --package former -- former_enum_tests`.
    *   Detailed Plan Step 3: Analyze results. Propose fixes if needed. Focus on `standalone_constructor_args_derive` and `standalone_constructor_args_manual` tests.
    *   Crucial Design Rules: [Proc Macro: Development Workflow](#proc-macro-development-workflow), [Minimal Changes](#enhancements-only-implement-whats-requested)
    *   Verification Strategy: Successful compilation and passing tests for the `standalone_constructor_args_*` group within `former_enum_tests`. **Analyze logs critically.**

*   [⚫] **Increment 12:** Uncomment and Test Standalone Constructor (`standalone_constructor_*`)
    *   Detailed Plan Step 1: Modify `module/core/former/tests/inc/mod.rs`. Uncomment `mod standalone_constructor_derive;`, `mod standalone_constructor_manual;`, and `mod standalone_constructor_only_test;`.
    *   Detailed Plan Step 2: Request user to run `cargo check --tests --package former` and `cargo test --package former -- former_enum_tests`.
    *   Detailed Plan Step 3: Analyze results. Propose fixes if needed. Focus on `standalone_constructor_derive` and `standalone_constructor_manual` tests.
    *   Crucial Design Rules: [Proc Macro: Development Workflow](#proc-macro-development-workflow), [Minimal Changes](#enhancements-only-implement-whats-requested)
    *   Verification Strategy: Successful compilation and passing tests for the `standalone_constructor_*` group within `former_enum_tests`. **Analyze logs critically.**

*   [⚫] **Increment 13:** Uncomment and Test Unit Variant (`unit_variant_*`)
    *   Detailed Plan Step 1: Modify `module/core/former/tests/inc/mod.rs`. Uncomment `mod unit_variant_derive;`, `mod unit_variant_manual;`, and `mod unit_variant_only_test;`.
    *   Detailed Plan Step 2: Request user to run `cargo check --tests --package former` and `cargo test --package former -- former_enum_tests`.
    *   Detailed Plan Step 3: Analyze results. Propose fixes if needed. Focus on `unit_variant_derive` and `unit_variant_manual` tests.
    *   Crucial Design Rules: [Proc Macro: Development Workflow](#proc-macro-development-workflow), [Minimal Changes](#enhancements-only-implement-whats-requested)
    *   Verification Strategy: Successful compilation and passing tests for the `unit_variant_*` group within `former_enum_tests`. **Analyze logs critically.**

*   [⚫] **Increment 14:** Uncomment and Test `usecase1.rs`
    *   Detailed Plan Step 1: Modify `module/core/former/tests/inc/mod.rs`. Uncomment `mod usecase1;`.
    *   Detailed Plan Step 2: Read `module/core/former/tests/inc/former_enum_tests/usecase1.rs`. Uncomment file content if necessary. Address the `// qqq : xxx : uncomment and make it working` task. Propose necessary code changes and remove the task comment.
    *   Detailed Plan Step 3: Request user to run `cargo check --tests --package former` and `cargo test --package former -- former_enum_tests`.
    *   Detailed Plan Step 4: Analyze results. This test uses default subformer generation for enum variants holding structs that also derive `Former`. Investigate `former_meta/src/derive_former/former_enum.rs` (likely `tuple_non_zero.rs` or `struct_non_zero.rs` using `context.md`) if issues arise. Propose fixes. Focus on `usecase1` tests.
    *   Crucial Design Rules: [Proc Macro: Development Workflow](#proc-macro-development-workflow), [Comments: Add Tasks and Label Simplifications](#comments-add-tasks-and-label-simplifications), [Minimal Changes](#enhancements-only-implement-whats-requested)
    *   Verification Strategy: Successful compilation and passing tests for `usecase1` within `former_enum_tests`. **Analyze logs critically.**

*   [⚫] **Increment 15:** Address `subform_collection_test.rs` (Known Compile Fail)
    *   Detailed Plan Step 1: Modify `module/core/former/tests/inc/mod.rs`. Uncomment `mod subform_collection_test;`.
    *   Detailed Plan Step 2: Read `module/core/former/tests/inc/former_enum_tests/subform_collection_test.rs`. Uncomment file content if necessary. Address the `// qqq : xxx : make it working` task.
    *   Detailed Plan Step 3: **Confirm with the user if this feature (`#[subform_entry]` for `Vec<Enum>`) should be implemented.** The comments indicate this test is expected to fail compilation because this is not currently supported.
        *   **If YES:** This is a significant feature addition. Propose a sub-plan to implement the necessary logic in `former_meta/src/derive_former/field.rs` (specifically `subform_entry_setter` using `context.md`) to handle enum variants. This involves generating code that can somehow select and start the correct former for a *specific enum variant* within the collection context. This is non-trivial.
        *   **If NO:** Modify the test file. Remove the test code and the file, adding a comment in `mod.rs` explaining the limitation, or comment out the test function with an explanation.
    *   Detailed Plan Step 4: Apply the chosen solution (implement feature or modify/remove test).
    *   Detailed Plan Step 5: Request user to run `cargo check --tests --package former`. If the feature was implemented, also run `cargo test --package former -- former_enum_tests`.
    *   Crucial Design Rules: [Proc Macro: Development Workflow](#proc-macro-development-workflow), [Comments: Add Tasks and Label Simplifications](#comments-add-tasks-and-label-simplifications), [Minimal Changes](#enhancements-only-implement-whats-requested), [Enhancements: Only Implement What’s Requested](#enhancements-only-implement-whats-requested)
    *   Verification Strategy: If implemented: Successful compilation and passing test for `subform_collection_test`. If removed/commented: Successful compilation and no test failures related to this file. **Analyze logs critically.**

*   [⚫] **Increment 16:** Final Verification
    *   Detailed Plan Step 1: Ensure all non-component enum test modules (`mod <test_file>;`) are uncommented in `module/core/former/tests/inc/mod.rs` (except potentially `subform_collection_test` if removed/commented).
    *   Detailed Plan Step 2: Request user to run `cargo check --tests --package former --all-targets`.
    *   Detailed Plan Step 3: Request user to run `cargo clippy --package former --all-targets -- -D warnings`.
    *   Detailed Plan Step 4: Request user to run `cargo test --package former --all-targets`. (This implicitly includes `former_enum_tests`).
    *   Crucial Design Rules: [Lints and warnings](#lints-and-warnings)
    *   Verification Strategy: Analyze output from user. Expect zero errors and zero warnings from `check` and `clippy`. Expect all tests for the `former` package to pass, paying close attention to the `former_enum_tests` results. **Analyze logs critically.**

### Requirements

*   **Adherence:** Strictly follow `code/gen` instructions, Design Rules, and Codestyle Rules for all modifications. Prioritize these rules over the existing style in the repository if conflicts arise.
*   **Paired Testing (Proc Macro Rule):** Ensure derived macro output (`_derive` tests) is always tested alongside its intended manual equivalent (`_manual` tests) within the same increment, following the [Proc Macro: Development Workflow](#proc-macro-development-workflow) rule. The `_only_test` files, if present, should also be uncommented in the same increment.
*   **Incremental Verification:** After each increment involving uncommenting a group of test files and making code changes:
    *   Ensure the relevant code compiles (`cargo check --tests --package former`).
    *   Run all active tests within the enum test module (`cargo test --package former -- former_enum_tests`). **Analyze logs critically**, focusing on the newly added tests (`_derive` and `_manual` variants) while ensuring previously passing tests remain successful.
*   **Task Handling:** Address `// xxx :` and `// qqq :` comments found in the currently uncommented test code according to the [Comments: Add Tasks and Label Simplifications](#comments-add-tasks-and-label-simplifications) rule. If a task is complex, convert it into a standard `// TODO:` comment with a brief explanation or suggest creating a dedicated issue.
*   **Component Model Exclusion:** Do *not* uncomment or attempt to fix tests within `module/core/former/tests/inc/components_tests/`. This module should remain inactive or be deleted as per the component model removal plan (`plan.md`).
*   **Minimal Changes:** Prioritize fixing existing tests with minimal changes, adhering to the [Minimal Changes](#enhancements-only-implement-whats-requested) rule. Avoid unnecessary refactoring unless required to make the test pass or adhere to rules.
*   **Plan Persistence:** Any modification to this plan (status updates, adding notes, refining steps) **must** be immediately persisted to `module/core/former/plan.md` using the `write_to_file` tool, and user confirmation of the successful write must be received before proceeding.
*   **Approval Gates:** Explicit user approval **must** be obtained before starting implementation of an increment (after detailed planning is finalized and written) and after successful verification of an increment (before moving to the next). User confirmation of successful `write_to_file` operations is also required.
*   **Context Generation:** This plan assumes a `context.md` file has been generated (via `generate_context.sh` planned and executed in prior steps) based on the files and crates listed in the `## Context` section. This `context.md` will be used during implementation.

## Notes & Insights

*   *(This section must always be present and preserved)*
*   **[Date/Inc #] Insight:** The `components_tests` module and its contents will be ignored as the component model is being removed per the other plan (`plan.md`).
*   **[Date/Inc #] Insight:** The task for `parametrized_dyn_manual.rs` (struct test) is removed from this plan's scope. It should be handled by `plan_dyn_trait_issue.md`.
*   **[Date/Inc #] Insight:** Several enum tests were initially commented out, suggesting potentially incomplete features or larger refactoring needs, especially around generics and subforms for enums. This plan addresses them incrementally, grouping related tests.
*   **[Date/Inc #] Insight:** `subform_collection_test.rs` is known to fail compilation and requires a user decision on whether to implement the underlying feature (`#[subform_entry]` for `Vec<Enum>`).
