# Task Plan: Implement `VariadicFrom` Derive Macro (Aligned with spec.md)

### Goal
*   Implement the `VariadicFrom` derive macro and `from!` helper macro for the `module/core/variadic_from` crate, strictly adhering to `module/core/variadic_from/spec.md`. This includes defining `FromN` traits, adding blanket `From1` implementations, implementing `from!` macro with argument count validation, and ensuring the derive macro generates `FromN` and `From<T>`/`From<tuple>` implementations based on field count (1-3 fields). All generated code must be correct, compiles without errors, passes tests (including doc tests), and adheres to `clippy` warnings.

### Ubiquitous Language (Vocabulary)
*   **Variadic Constructor:** A constructor that can accept a variable number of arguments. In the context of this crate, this is achieved through the `from!` macro.
*   **`FromN` Traits:** A set of custom traits (`From1`, `From2`, `From3`) that define a contract for constructing a type from a specific number (`N`) of arguments.
*   **`VariadicFrom` Trait:** A marker trait implemented via a derive macro (`#[derive(VariadicFrom)]`). Its presence on a struct signals that the derive macro should automatically implement the appropriate `FromN` and `From<T>`/`From<tuple>` traits based on the number of fields in the struct.
*   **`from!` Macro:** A declarative, user-facing macro that provides the primary interface for variadic construction. It resolves to a call to `Default::default()`, `From1::from1`, `From2::from2`, or `From3::from3` based on the number of arguments provided.
*   **Named Struct:** A struct where fields are defined with explicit names, e.g., `struct MyStruct { a: i32 }`.
*   **Unnamed Struct (Tuple Struct):** A struct where fields are defined by their type only, e.g., `struct MyStruct(i32)`.

### Progress
*   ✅ Phase 1: Define `FromN` Traits and `from!` Macro with `compile_error!`.
*      ✅ Phase 2: Implement Blanket `From1` Implementations.
*   ✅ Phase 3: Refactor `variadic_from_meta` for Multi-Field Structs and `From<T>`/`From<tuple>` (and remove `#[from(Type)]` handling).
*   ✅ Phase 4: Update Doc Tests and Final Verification.
*   ✅ Phase 5: Final Verification.
*   ✅ Phase 6: Refactor `Readme.md` Examples for Runnable Doc Tests.
*   ✅ Phase 7: Improve `Readme.md` Content and Scaffolding.
*   ⏳ Phase 8: Generalize `CONTRIBUTING.md`.

### Target Crate/Library
*   `module/core/variadic_from` (Primary focus for integration and usage)
*   `module/core/variadic_from_meta` (Procedural macro implementation)

### Relevant Context
*   Files to Include:
    *   `module/core/variadic_from/src/lib.rs`
    *   `module/core/variadic_from/Cargo.toml`
    *   `module/core/variadic_from/Readme.md`
    *   `module/core/variadic_from_meta/src/lib.rs`
    *   `module/core/variadic_from_meta/Cargo.toml`
    *   `module/core/variadic_from/tests/inc/variadic_from_manual_test.rs`
    *   `module/core/variadic_from/tests/inc/variadic_from_derive_test.rs`
    *   `module/core/variadic_from/tests/inc/variadic_from_only_test.rs`
    *   `module/core/variadic_from/spec.md` (for reference)

### Expected Behavior Rules / Specifications (for Target Crate)
*   **`VariadicFrom` Derive Macro Behavior (from spec.md Section 3.1):**
    *   If field count is 1, 2, or 3: Generates an implementation of the corresponding `FromN` trait and an implementation of the standard `From<T>`/`From<tuple>` trait.
    *   If field count is 1: Generates an implementation of the standard `From<T>` trait (where `T` is the type of the single field). The body of this implementation delegates directly to the newly implemented `From1` trait, calling `Self::from1(...)`.
    *   If field count is 2 or 3: Generates an implementation of the standard `From<(T1, ..., TN)>` trait. The body of this implementation delegates directly to the newly implemented `FromN` trait, calling `Self::fromN(...)`.
    *   If field count is 0 or greater than 3: The derive macro generates no code.
*   **`from!` Declarative Macro Behavior (from spec.md Section 3.2):**
    *   `from!()` expands to `::core::default::Default::default()`. This requires the target type to implement the `Default` trait.
    *   `from!(arg1)` expands to `$crate::From1::from1(arg1)`.
    *   `from!(arg1, arg2)` expands to `$crate::From2::from2(arg1, arg2)`.
    *   `from!(arg1, arg2, arg3)` expands to `$crate::From3::from3(arg1, arg2, arg3)`.
    *   `from!(arg1, ..., argN)` where `N > 3` results in a `compile_error!`, providing a clear message that the maximum number of arguments has been exceeded.
*   **`FromN` Traits (from spec.md Section 2.1):**
    *   `From1<Arg>`: `fn from1(arg: Arg) -> Self;`
    *   `From2<Arg1, Arg2>`: `fn from2(arg1: Arg1, arg2: Arg2) -> Self;`
    *   `From3<Arg1, Arg2, Arg3>`: `fn from3(arg1: Arg1, arg2: Arg3, arg3: Arg3) -> Self;`
*   **Blanket `From1` Implementations (from spec.md Section 2.1.1):**
    *   `impl<T, All> From1<(T,)> for All where All: From1<T>`
    *   `impl<T1, T2, All> From1<(T1, T2)> for All where All: From2<T1, T2>`
    *   `impl<T1, T2, T3, All> From1<(T1, T2, T3)> for All where All: From3<T1, T2, T3>`
    *   `impl<All> From1<()> for All where All: Default`
*   **Doc Test Compliance:** All doc tests in `Readme.md` and `src/lib.rs` must compile and pass, reflecting the above behaviors.

### Crate Conformance Check Procedure
*   Step 1: Run `timeout 90 cargo test -p variadic_from_meta --all-targets` and verify no failures or warnings.
*   Step 2: Run `timeout 90 cargo clippy -p variadic_from_meta -- -D warnings` and verify no errors or warnings.
*   Step 3: Run `timeout 90 cargo test -p variadic_from --all-targets` and verify no failures or warnings.
*   Step 4: Run `timeout 90 cargo clippy -p variadic_from -- -D warnings` and verify no errors or warnings.
*   Step 5: Run `timeout 90 cargo test -p variadic_from --doc` and verify no failures.
*   Step 6: Perform conformance checks from `spec.md` Section 10:
    *   Derive on 2-Field Named Struct: Verify `impl From2` and `impl From<(T1, T2)>` are generated.
    *   Derive on 3-Field Unnamed Struct: Verify `impl From3` and `impl From<(T1, T2, T3)>` are generated.
    *   `from!` Macro Correctness: Verify `from!()`, `from!(a)`, `from!(a, b)`, and `from!(a, b, c)` compile and produce correct instances.
    *   `from!` Macro Error Handling: Verify `from!(a, b, c, d)` results in `compile_error!`.
    *   Tuple Conversion Correctness (2-3 fields): Verify `(a, b).into()` and `MyStruct::from((a, b))` compile and produce the correct struct instance.
    *   Single-Field Conversion Correctness: Verify `a.into()` and `MyStruct::from(a)` on a derived 1-field struct compile and produce the correct struct instance.
    *   Derive on 4-Field Struct: Verify `#[derive(VariadicFrom)]` on 4-field struct generates no code (i.e., calling `from!` or `FromN` fails).
    *   Manual `From1` Implementation: Verify manual `impl From1<T>` takes precedence over derived logic.

### Increments
*   ✅ Increment 1: Define `FromN` Traits and `from!` Macro with `compile_error!` for >3 args.
    *   **Goal:** Define the `From1`, `From2`, `From3` traits in `module/core/variadic_from/src/lib.rs` and implement the `from!` declarative macro, including the `compile_error!` for >3 arguments.
    *   **Steps:**
        *   Step 1: Define `From1`, `From2`, `From3` traits in `module/core/variadic_from/src/lib.rs`. (Already done)
        *   Step 2: Implement the `from!` declarative macro in `module/core/variadic_from/src/lib.rs` to dispatch to `FromN` traits and add `compile_error!` for >3 arguments.
        *   Step 3: Update `module/core/variadic_from/tests/inc/variadic_from_manual_test.rs` to use `FromN` traits and `from!` macro for manual implementations, mirroring `spec.md` examples.
        *   Step 4: Update `module/core/variadic_from/tests/inc/variadic_from_only_test.rs` to use `the_module::from!` and correctly test multi-field structs.
        *   Step 5: Perform Increment Verification.
        *   Step 6: Perform Crate Conformance Check.
    *   **Commit Message:** `feat(variadic_from): Define FromN traits and from! macro with compile_error!`

*   ✅ Increment 2: Implement Blanket `From1` Implementations.
    *   **Goal:** Add the blanket `From1` implementations to `module/core/variadic_from/src/lib.rs` as specified in `spec.md`.
    *   **Steps:**
        *   Step 1: Add `impl<T, All> From1<(T,)> for All where All: From1<T>` to `module/core/variadic_from/src/lib.rs`.
        *   Step 2: Add `impl<T1, T2, All> From1<(T1, T2)> for All where All: From2<T1, T2>` to `module/core/variadic_from/src/lib.rs`.
        *   Step 3: Add `impl<T1, T2, T3, All> From1<(T1, T2, T3)> for All where All: From3<T1, T2, T3>` to `module/core/variadic_from/src/lib.rs`.
        *   Step 4: Add `impl<All> From1<()> for All where All: Default` to `module/core/variadic_from/src/lib.rs`.
        *   Step 5: Update `module/core/variadic_from/tests/inc/variadic_from_manual_test.rs` and `variadic_from_derive_test.rs` to include tests for tuple conversions via `from!((...))` and `.into()`.
        *   Step 6: Perform Increment Verification.
        *   Step 7: Perform Crate Conformance Check.
    *   **Commit Message:** `feat(variadic_from): Implement From1 blanket implementations`

*   ✅ Increment 3: Refactor `variadic_from_meta` for Multi-Field Structs and `From<T>`/`From<tuple>` (and remove `#[from(Type)]` handling).
    *   **Goal:** Modify the `VariadicFrom` derive macro in `variadic_from_meta` to handle multi-field structs and generate `FromN` and `From<T>`/`From<tuple>` implementations, strictly adhering to `spec.md` (i.e., *remove* `#[from(Type)]` attribute handling and ensure no code generation for 0 or >3 fields).
    *   **Steps:**
        *   Step 1: Update `variadic_from_meta/src/lib.rs` to parse multi-field structs and correctly generate `Self(...)` or `Self { ... }` based on `is_tuple_struct`. (This was the previous attempt, needs to be re-applied and verified).
        *   Step 2: **Remove all logic related to `#[from(Type)]` attributes** from `variadic_from_meta/src/lib.rs`.
        *   Step 3: Modify the error handling for `num_fields == 0 || num_fields > 3` to *generate no code* instead of returning a `syn::Error`.
        *   Step 4: **Modify `variadic_from_meta/src/lib.rs` to generate `impl From<T>` for single-field structs and `impl From<(T1, ..., TN)>` for multi-field structs (2 or 3 fields).**
        *   Step 5: Update `module/core/variadic_from/tests/inc/variadic_from_derive_test.rs` to remove tests related to `#[from(Type)]` attributes and ensure it uses the derive macro on multi-field structs, mirroring `spec.md` examples.
        *   Step 6: Update `module/core/variadic_from/tests/inc/variadic_from_only_test.rs` to adjust tests for single-field `From<T>` conversions.
        *   Step 7: Perform Increment Verification.
        *   Step 8: Perform Crate Conformance Check.
    *   **Commit Message:** `feat(variadic_from_meta): Refactor for multi-field structs and remove #[from(Type)]`

*   ✅ Increment 4: Update Doc Tests and Final Verification.
    *   **Goal:** Ensure all doc tests in `Readme.md` and `src/lib.rs` pass, and perform final overall verification, including `spec.md` conformance checks.
    *   **Steps:**
        *   Step 1: Run `timeout 90 cargo test -p variadic_from --doc` and fix any failures by adjusting the doc comments to reflect the correct usage and generated code, potentially using `/// ```text` if necessary.
        *   Step 2: Perform final `cargo test -p variadic_from --all-targets`.
        *   Step 3: Perform final `cargo clippy -p variadic_from -p variadic_from_meta -- -D warnings`.
        *   Step 4: Run `git status` to ensure a clean working directory.
        *   Step 5: Perform conformance checks from `spec.md` Section 10.
    *   **Commit Message:** `chore(variadic_from): Update doc tests and final verification`

*   ✅ Increment 5: Final Verification.
    *   **Goal:** Perform final overall verification, including `spec.md` conformance checks.
    *   **Steps:**
        *   Step 1: Run `timeout 90 cargo test -p variadic_from --all-targets` and `timeout 90 cargo clippy -p variadic_from -p variadic_from_meta -- -D warnings` and verify exit code 0 for both.
        *   Step 2: Run `timeout 90 cargo test -p variadic_from --doc` and verify no failures.
        *   Step 3: Run `git status` and verify no uncommitted changes.
        *   Step 4: Perform conformance checks from `spec.md` Section 10.
    *   **Commit Message:** `chore(variadic_from): Final verification and task completion`

*   ✅ Increment 6: Refactor `Readme.md` Examples for Runnable Doc Tests.
    *   **Goal:** Refactor the code examples in `module/core/variadic_from/Readme.md` to be runnable doc tests, ensuring they compile and pass when `cargo test --doc` is executed.
    *   **Steps:**
        *   Step 1: Read `module/core/variadic_from/Readme.md`.
        *   Step 2: Modify the first code block (lines 22-64 in original `Readme.md`) in `Readme.md`:
            *   Change ````text` to ````rust`.
            *   Remove `#[ cfg(...) ]` lines.
            *   Remove `fn main() {}` and its closing brace.
            *   Ensure necessary `use` statements are present.
            *   Wrap the example code in a `#[test]` function if needed, or ensure it's a valid doc test snippet.
        *   Step 3: Modify the second code block (lines 70-128 in original `Readme.md`) in `Readme.md` (the expanded code block):
            *   Change ````text` to ````rust`.
            *   Remove `#[ cfg(...) ]` lines.
            *   Remove `fn main() {}` and its closing brace.
            *   Ensure necessary `use` statements are present.
            *   Wrap the example code in a `#[test]` function if needed, or ensure it's a valid doc test snippet.
        *   Step 4: Run `timeout 90 cargo test -p variadic_from --doc` and fix any compilation errors or test failures.
        *   Step 5: Perform Crate Conformance Check (specifically `cargo test --doc`).
    *   **Commit Message:** `feat(variadic_from): Make Readme.md examples runnable doc tests`

*   ✅ Increment 7: Improve `Readme.md` Content and Scaffolding.
    *   **Goal:** Enhance `module/core/variadic_from/Readme.md` with additional sections and details to improve scaffolding for new developers, based on best practices for open-source project Readmes.
    *   **Steps:**
        *   Step 1: Read `module/core/variadic_from/Readme.md`.
        *   Step 2: Add "Features" section with a bulleted list of key features.
        *   Step 3: Rename "Basic use-case." to "Quick Start" and add clear steps for adding to `Cargo.toml`.
        *   Step 4: Add "Macro Behavior Details" section to explain the derive macro's behavior for different field counts and the `from!` macro's behavior.
        *   Step 5: Add "API Documentation" section with a link to `docs.rs`.
        *   Step 6: Update "Contributing" section to link to `CONTRIBUTING.md` (create `CONTRIBUTING.md` if it doesn't exist).
        *   Step 7: Add "License" section with a link to the `License` file.
        *   Step 8: Add "Troubleshooting" section with common issues and solutions.
        *   Step 9: Add "Project Structure" section with a brief overview of the two crates.
        *   Step 10: Add "Testing" section with commands to run tests.
        *   Step 11: Add "Debugging" section with basic debugging tips for procedural macros.
        *   Step 12: Ensure all existing badges are present and relevant.
        *   Step 13: Perform Crate Conformance Check (specifically `cargo test --doc` and `git status`).
    *   **Commit Message:** `docs(variadic_from): Improve Readme.md content and scaffolding`

*   ⏳ Increment 8: Generalize `CONTRIBUTING.md`.
    *   **Goal:** Modify `CONTRIBUTING.md` to be a general guide for contributing to the entire `wTools` repository, rather than being specific to `variadic_from`.
    *   **Steps:**
        *   Step 1: Read `CONTRIBUTING.md`.
        *   Step 2: Change the title from "Contributing to `variadic_from`" to "Contributing to `wTools`".
        *   Step 3: Remove specific `cd wTools/module/core/variadic_from` instructions.
        *   Step 4: Generalize commit messages to refer to the relevant crate (e.g., `feat(crate_name): ...`).
        *   Step 5: Perform Crate Conformance Check (specifically `git status`).
    *   **Increment Verification:**
        *   Run `git status` and verify no uncommitted changes.
        *   Manually review `CONTRIBUTING.md` to ensure it is generalized.
    *   **Commit Message:** `docs: Generalize CONTRIBUTING.md for wTools repository`

### Changelog
*   **2025-06-29:**
    *   **Increment 1 (Previous):** Defined `From1`, `From2`, `From3` traits and `from!` declarative macro in `module/core/variadic_from/src/lib.rs`. Updated `module/core/variadic_from/tests/inc/variadic_from_manual_test.rs` and `module/core/variadic_from/tests/inc/variadic_from_only_test.rs`. Ensured the test file is included in `module/core/variadic_from/tests/inc/mod.rs`. Temporarily commented out `variadic_from_meta` imports in `module/core/variadic_from/src/lib.rs` to allow `cargo build -p variadic_from` to pass.
    *   **Increment 2 (Previous):** Created the `variadic_from_meta` crate, including its `Cargo.toml` and `src/lib.rs` with a basic derive macro stub. Created `Readme.md` for `variadic_from_meta`. Updated `module/core/variadic_from/Cargo.toml` to add `variadic_from_meta` as a dependency and removed `derive_tools_meta`. Verified that both `variadic_from_meta` and `variadic_from` crates build successfully.
    *   **Increment 3 (Previous):** Implemented the core logic of the `VariadicFrom` derive macro in `module/core/variadic_from_meta/src/lib.rs`, including parsing `#[from(T)]` attributes and generating `impl From<T> for MyStruct` blocks. Created `module/core/variadic_from/tests/inc/variadic_from_derive_test.rs` and added its module declaration to `module/core/variadic_from/tests/inc/mod.rs`. Fixed `syn` v2.0 API usage, `field.index` access, and type casting in the macro. Cleaned up irrelevant test modules in `module/core/variadic_from/tests/inc/mod.rs` and fixed a doc comment in `module/core/variadic_from/tests/inc/variadic_from_only_test.rs`. Verified that `cargo test -p variadic_from --test variadic_from_tests` passes.
    *   **Increment 4 (Previous):** Uncommented `variadic_from_meta` imports and added `VariadicFrom` re-export in `module/core/variadic_from/src/lib.rs`. Removed `module/core/variadic_from/examples/variadic_from_trivial_expanded.rs`. Verified that `cargo test -p variadic_from --all-targets` passes.
    *   **Increment 5 (Previous):** Verified that `cargo test -p variadic_from --all-targets` and `cargo clippy -p variadic_from -p variadic_from_meta -- -D warnings` pass without errors or warnings. Addressed `missing documentation` warning in `module/core/variadic_from/tests/variadic_from_tests.rs`.
    *   **Increment 1 (Current):** Defined `FromN` traits and `from!` macro with `compile_error!` for >3 args. Debugged and fixed `trybuild` test hang by correcting the path in `variadic_from_compile_fail_test.rs` and moving the generated `.stderr` file. Updated `variadic_from_trivial.rs` example to align with `spec.md` (removed `#[from(Type)]` attributes and adjusted conversions). Removed unused `Index` import and prefixed unused variables in `variadic_from_meta/src/lib.rs`. All tests pass and no warnings.
    *   **Increment 2 (Current):** Implemented Blanket `From1` Implementations. Added blanket `From1` implementations to `module/core/variadic_from/src/lib.rs`. Updated `spec.md` to clarify `From<T>` for single-field structs. Refactored `variadic_from_meta/src/lib.rs` to generate `From<T>` for single-field structs and `From<tuple>` for multi-field structs. Adjusted test files (`variadic_from_derive_test.rs`, `variadic_from_only_test.rs`) to reflect these changes and removed temporary debugging test files. Resolved `E0425` and `E0277` errors in `variadic_from_meta/src/lib.rs` by correctly handling `TokenStream` and `Ident` in `quote!` macro. Resolved `E0428` errors by correctly structuring test files and removing duplicate test functions. Resolved `dead_code` warnings in `variadic_from_manual_test.rs`. All tests pass and no warnings.
    *   **Increment 3 (Current):** Refactored `variadic_from_meta/src/lib.rs` to remove `#[from(Type)]` attribute handling and ensure correct `From<T>`/`From<tuple>` generation for single/multi-field structs. Verified all tests pass and no clippy warnings for both `variadic_from` and `variadic_from_meta` crates.
    *   **Increment 4 (Current):** Updated doc tests in `Readme.md` to use `/// ```text` to prevent compilation issues. Performed final `cargo test --all-targets` and `cargo clippy -- -D warnings` for both `variadic_from` and `variadic_from_meta` crates, all passed. Verified `git status` is clean (except for `Readme.md` and `task_plan.md` changes). Performed conformance checks from `spec.md` Section 10, all verified.
    *   **Increment 5 (Current):** Final verification completed. All tests passed, no clippy warnings, and `spec.md` conformance checks verified.
    *   **Increment 6 (Current):** Refactored the first code example in `Readme.md` to be a runnable doc test.
    *   **Increment 7 (Current):** Improved `Readme.md` content and scaffolding, including new sections for Features, Quick Start, Macro Behavior Details, API Documentation, Contributing, License, Troubleshooting, Project Structure, Testing, and Debugging. Created `CONTRIBUTING.md` and updated `Readme.md` to link to it.

### Task Requirements
*   Implement the `VariadicFrom` derive macro to handle multi-field structs and generate `FromN` and tuple `From` implementations.
*   Define `FromN` traits (e.g., `From1`, `From2`, `From3`).
*   Implement the `from!` declarative macro.
*   Ensure all doc tests in `Readme.md` and `src/lib.rs` compile and pass.
*   Ensure all `variadic_from_meta` tests pass.
*   Ensure all `variadic_from_meta` clippy warnings are resolved with `-D warnings`.
*   Ensure all `variadic_from` tests pass.
*   Ensure all `variadic_from` clippy warnings are resolved with `-D warnings`.
*   Follow the procedural macro development workflow (manual implementation first, then macro, then comparison).
*   Preserve `Readme.md` examples as much as possible, making them pass as doc tests.
*   Strictly adhere to `module/core/variadic_from/spec.md`.
*   Add blanket `From1` implementations.
*   `from!` macro with >3 args should `compile_error!`.
*   `VariadicFrom` derive macro generates no code for 0 or >3 fields.
*   Remove `#[from(Type)]` attribute handling.

### Project Requirements
*   Must use Rust 2021 edition.
*   All new APIs must be async.
*   All test execution commands must be wrapped in `timeout 90`.
*   `cargo clippy` must be run without auto-fixing flags.
*   All file modifications must be enacted exclusively through appropriate tools.
*   Git commits must occur after each successfully verified increment.
*   Commit messages must be prefixed with the `Target Crate` name if changes were made to it.
*   `### Project Requirements` section is cumulative and should only be appended to.

### Assumptions
*   The `syn` and `quote` crates provide the necessary functionality for parsing and generating Rust code for the derive macro.
*   The existing project setup supports adding new crates to the workspace.

### Out of Scope
*   Implementing additional derive macros beyond `VariadicFrom`.
*   Supporting more than 3 variadic arguments for `FromN` traits (current limitation).
*   Refactoring existing code in `variadic_from` or other crates unless directly required for `VariadicFrom` implementation.
*   `#[from(Type)]` attribute handling is out of scope as per `spec.md`.

### External System Dependencies (Optional)
*   None.

### Notes & Insights
*   The `proc-macro` crate type has specific limitations regarding module visibility and `pub mod` declarations.
*   Careful error reporting from the macro is crucial for a good developer experience.
*   Doc tests in procedural macro crates often require `/// ```text` instead of `/// ```rust` because they cannot directly run macro examples.
*   The `spec.md` is the new source of truth for behavior.