# Project Plan: Refine Component Model Crates

## Goal

Refine the `component_model`, `component_model_meta`, and `component_model_types` crates to be production-ready, ensuring complete isolation from the original `former` crate where appropriate, consistency, clarity, conciseness, correctness, and adherence to all specified rules (codestyle, clippy). Also make sure there is no garbase left in code, examples or documentation from former. Bear in mind that all "former" words were replaced by "component_model", so if something does not have in name former it does not mean it's not garbage!

## Crates Involved

*   `component_model_types` (Core traits and types)

## Increments

*   ⏳ **Increment 1: Review & Refine `component_model_types` Crate**
    *   Detailed Plan Step 1: Read and analyze `src/lib.rs` for structure, exports, features, and potential `former` remnants. Propose necessary cleanup. *(Cleanup attempted, resulted in build errors - needs fixing)*
    *   Detailed Plan Step 2: Read and analyze `src/axiomatic.rs`. Check for clarity, correctness, rule adherence, and `former` remnants. Propose changes if needed.
    *   Detailed Plan Step 3: Read and analyze `src/definition.rs`. Check for clarity, correctness, rule adherence, and `former` remnants. Propose changes if needed. *(Partially done - build errors encountered)*
    *   Detailed Plan Step 4: Read and analyze `src/forming.rs`. Check for clarity, correctness, rule adherence, and `former` remnants. Propose changes if needed. *(Partially done - build errors encountered)*
    *   Detailed Plan Step 5: Read and analyze `src/storage.rs`. Check for clarity, correctness, rule adherence, and `former` remnants. Propose changes if needed.
    *   Detailed Plan Step 6: Read and analyze `src/component.rs`. Check for clarity, correctness, rule adherence (especially trait definitions like `Assign`), and `former` remnants. Propose changes if needed.
    *   Detailed Plan Step 7: Review `Cargo.toml` for dependencies, features (especially related to `no_std`, `use_alloc`), metadata, and correctness. Propose updates if needed.
    *   Detailed Plan Step 8: Review `Readme.md` for clarity, accuracy, consistency with code, and removal of `former` references/concepts. Propose updates if needed.
    *   Crucial Design Rules: [Visibility: Keep Implementation Details Private](https://github.com/Wandalen/wTools/blob/master/module/core/component_model/../../doc/rust/rules/design.md#visibility-keep-implementation-details-private), [Comments and Documentation](https://github.com/Wandalen/wTools/blob/master/module/core/component_model/../../doc/rust/rules/design.md#comments-and-documentation), [Code Style: Do Not Reformat Arbitrarily](https://github.com/Wandalen/wTools/blob/master/module/core/component_model/../../doc/rust/rules/design.md#code-style-do-not-reformat-arbitrarily)
    *   Verification Strategy: After each file modification, request user run `cargo build -p component_model_types` and provide output. **Analyze logs critically**. After all steps in this increment, request user run `cargo test -p component_model_types` and provide output. **Analyze logs critically**. Manual review against goals (clarity, correctness, consistency, rule adherence, `former` removal). Final clippy check in Increment 7.
*   ⚫ **Increment 2: Review & Refine `component_model_meta` Crate**
    *   Detailed Plan Step 1: Read and analyze `src/lib.rs` for structure, macro exports, features, and potential `former` remnants. Propose necessary cleanup.
    *   Detailed Plan Step 2: Read and analyze `src/component/component_from.rs`. Check macro logic for clarity, correctness, rule adherence, path resolution, error handling, and `former` remnants. Propose changes if needed.
    *   Detailed Plan Step 3: Read and analyze `src/component/from_components.rs`. Check macro logic for clarity, correctness, rule adherence, path resolution, error handling, and `former` remnants. Propose changes if needed.
    *   Detailed Plan Step 4: Read and analyze `src/component/component_assign.rs`. Check macro logic for clarity, correctness, rule adherence, path resolution, error handling, and `former` remnants. Propose changes if needed.
    *   Detailed Plan Step 5: Read and analyze `src/component/components_assign.rs`. Check macro logic for clarity, correctness, rule adherence, path resolution, error handling, and `former` remnants. Propose changes if needed.
    *   Detailed Plan Step 6: Review `Cargo.toml` for dependencies (esp. `proc-macro2`, `quote`, `syn`), features, metadata, and correctness. Propose updates if needed.
    *   Detailed Plan Step 7: Review `Readme.md` for clarity, accuracy, consistency with macro behavior, and removal of `former` references/concepts. Propose updates if needed.
    *   Crucial Design Rules: [Proc Macro: Development Workflow](https://github.com/Wandalen/wTools/blob/master/module/core/component_model/../../doc/rust/rules/design.md#proc-macro-development-workflow), [Structuring: Proc Macro and Generated Path Resolution](https://github.com/Wandalen/wTools/blob/master/module/core/component_model/../../doc/rust/rules/design.md#structuring-proc-macro-and-generated-path-resolution), [Comments and Documentation](https://github.com/Wandalen/wTools/blob/master/module/core/component_model/../../doc/rust/rules/design.md#comments-and-documentation)
    *   Verification Strategy: After each file modification, request user run `cargo build -p component_model_meta` and provide output. **Analyze logs critically**. After all steps in this increment, request user run `cargo test -p component_model_meta` (if tests exist) and provide output. **Analyze logs critically**. Manual review against goals. Final clippy check in Increment 7.
*   ⚫ **Increment 3: Review & Refine `component_model` Facade Crate**
    *   Detailed Plan Step 1: Read and analyze `src/lib.rs` for structure, re-exports (ensuring it exposes the intended public API from `_types` and `_meta`), features, and potential `former` remnants. Propose necessary cleanup.
    *   Detailed Plan Step 2: Review `Cargo.toml` for dependencies (should primarily be `_types` and `_meta`), features, metadata, and correctness. Ensure features correctly enable/disable re-exports. Propose updates if needed.
    *   Detailed Plan Step 3: Review `Readme.md` for clarity, accuracy, consistency with the exposed API, and removal of `former` references/concepts. Propose updates if needed.
    *   Crucial Design Rules: [Visibility: Keep Implementation Details Private](https://github.com/Wandalen/wTools/blob/master/module/core/component_model/../../doc/rust/rules/design.md#visibility-keep-implementation-details-private), [Comments and Documentation](https://github.com/Wandalen/wTools/blob/master/module/core/component_model/../../doc/rust/rules/design.md#comments-and-documentation)
    *   Verification Strategy: After each file modification, request user run `cargo build -p component_model` and provide output. **Analyze logs critically**. After all steps in this increment, request user run `cargo test -p component_model` and provide output. **Analyze logs critically**. Manual review against goals. Final clippy check in Increment 7.
*   ⚫ **Increment 4: Review & Refine Tests (`component_model` crate)**
    *   Detailed Plan Step 1: Analyze `tests/tests.rs`, `tests/smoke_test.rs`, `tests/experimental.rs` for correctness, clarity, coverage, and `former` remnants.
    *   Detailed Plan Step 2: Analyze `tests/inc/mod.rs` and all files under `tests/inc/components_tests/`. Verify test structure (manual vs macro, shared logic via `_only_test.rs`), correctness, clarity, coverage (especially macro edge cases), and removal of `former` remnants.
    *   Detailed Plan Step 3: Identify and fix commented-out tests (ref `// xxx : fix commented out tests` in `component_model/src/lib.rs`).
    *   Detailed Plan Step 4: Ensure all tests pass and cover the refined API and macro behaviors.
    *   Crucial Design Rules: [Testing: Avoid Writing Automated Tests Unless Asked](https://github.com/Wandalen/wTools/blob/master/module/core/component_model/../../doc/rust/rules/design.md#testing-avoid-writing-tests-unless-asked), [Proc Macro: Development Workflow](https://github.com/Wandalen/wTools/blob/master/module/core/component_model/../../doc/rust/rules/design.md#proc-macro-development-workflow) (test structure part)
    *   Verification Strategy: Request user run `cargo test --workspace --all-targets --all-features` and provide output. **Analyze logs critically** for failures or warnings. Manual review of test logic and coverage.
*   ⚫ **Increment 5: Review & Refine Examples (`component_model` & `component_model_types` crates)**
    *   Detailed Plan Step 1: Read and analyze `component_model/examples/component_model_trivial.rs`. Ensure it compiles, runs, is clear, up-to-date, and free of `former` remnants.
    *   Detailed Plan Step 2: Read and analyze `component_model/examples/readme.md`. Ensure consistency with the main Readme and code.
    *   Detailed Plan Step 3: Check for examples in `component_model_types/examples/` (if any) and analyze them similarly.
    *   Crucial Design Rules: [Comments and Documentation](https://github.com/Wandalen/wTools/blob/master/module/core/component_model/../../doc/rust/rules/design.md#comments-and-documentation)
    *   Verification Strategy: Request user run `cargo run --example <name>` for each example in `component_model` and `component_model_types`. Provide output. Manual review for clarity and correctness.
*   ⚫ **Increment 6: Final Readme Updates (All three crates)**
    *   Detailed Plan Step 1: Review and update `component_model/Readme.md` for overall clarity, usage instructions, feature explanations, and consistency.
    *   Detailed Plan Step 2: Review and update `component_model_meta/Readme.md` focusing on macro usage, attributes, and generated code examples.
    *   Detailed Plan Step 3: Review and update `component_model_types/Readme.md` focusing on core traits and concepts.
    *   Detailed Plan Step 4: Ensure crate-level documentation (`#![doc = ...]`) in each `lib.rs` is accurate and consistent.
    *   Crucial Design Rules: [Comments and Documentation](https://github.com/Wandalen/wTools/blob/master/module/core/component_model/../../doc/rust/rules/design.md#comments-and-documentation)
    *   Verification Strategy: Manual review of all three `Readme.md` files and `lib.rs` crate-level docs for accuracy, clarity, and consistency.
*   ⚫ **Increment 7: Final Rule Check (Clippy & Codestyle)**
    *   Detailed Plan Step 1: Run `cargo clippy --workspace --all-targets --all-features -- -D warnings`. Address any reported issues across all three crates.
    *   Detailed Plan Step 2: Run `cargo fmt --all --check`. Address any formatting issues across all three crates.
    *   Crucial Design Rules: All Codestyle and Design rules.
    *   Verification Strategy: Request user run `cargo clippy --workspace --all-targets --all-features -- -D warnings` and `cargo fmt --all --check`. Provide output. Confirm no errors or warnings remain.

## Notes & Insights

*   *(No notes yet)*
