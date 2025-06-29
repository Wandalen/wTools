# Task Plan: Implement `VariadicFrom` Derive Macro

### Goal
*   The primary goal is to implement the `VariadicFrom` derive macro, allowing structs in the `module/core/variadic_from` crate to automatically generate `From` trait implementations for a variable number of arguments or tuples, and also `From<Type>` implementations based on `#[from(Type)]` attributes. This includes defining `FromN` traits and a `from!` helper macro, ensuring all generated code is correct, compiles without errors, passes tests (including doc tests), and adheres to `clippy` warnings.

### Ubiquitous Language (Vocabulary)
*   **`variadic_from`:** The main crate that will re-export the procedural macro, define `FromN` traits, implement the `from!` macro, and contain examples/tests.
*   **`variadic_from_meta`:** The procedural macro crate that will contain the `VariadicFrom` macro implementation.
*   **`VariadicFrom`:** The derive macro being implemented, which generates `FromN` trait implementations and `From` implementations for tuples, and also `From<Type>` implementations based on `#[from(Type)]` attributes.
*   **`FromN` traits:** Traits like `From1`, `From2`, `From3`, etc., which define conversion from `N` arguments.
*   **`from!` macro:** A declarative macro that provides a convenient syntax for constructing structs using variadic arguments, leveraging the `FromN` traits.
*   **`syn` / `quote`:** Core libraries used for parsing Rust code and generating new code within procedural macros.

### Progress
*   ✅ Phase 1: Define `FromN` Traits and `from!` Macro.
*   ⚫ Phase 2: Refactor `variadic_from_meta` for Multi-Field Structs and Variadic `From`.
*   ⚫ Phase 3: Implement `#[from(Type)]` Attribute Handling.
*   ⚫ Phase 4: Update Tests and Verify Doc Tests.
*   ⚫ Phase 5: Final Verification.

### Target Crate/Library
*   `module/core/variadic_from` (Primary focus for integration and usage)
*   `module/core/variadic_from_meta` (Procedural macro implementation)

### Relevant Context
*   Files to Include (for AI's reference, primarily from Target Crate):
    *   `module/core/variadic_from/src/lib.rs`
    *   `module/core/variadic_from/Cargo.toml`
    *   `module/core/variadic_from/Readme.md` (Crucial for doc tests and examples)
    *   `module/core/variadic_from_meta/src/lib.rs`
    *   `module/core/variadic_from_meta/Cargo.toml`
    *   `module/core/variadic_from/tests/inc/variadic_from_manual_test.rs`
    *   `module/core/variadic_from/tests/inc/variadic_from_derive_test.rs`
    *   `module/core/variadic_from/tests/inc/variadic_from_only_test.rs`
*   Crates for Documentation (for AI's reference, if `read_file` on docs is planned):
    *   `variadic_from`
    *   `variadic_from_meta`
    *   `syn`
    *   `quote`
*   External Crates Requiring `task.md` Proposals (if any identified during planning):
    *   None.

### Expected Behavior Rules / Specifications (for Target Crate)
*   **`VariadicFrom` Derive Macro Behavior:**
    *   When applied to a struct with `N` fields (where `1 <= N <= 3`):
        *   Generates `impl FromN<T1, ..., TN> for MyStruct`, where `T1` to `TN` are the types of the struct's fields.
        *   Generates `impl From<(T1, ..., TN)> for MyStruct` for tuple conversion.
        *   **Special Case for `From1` on multi-field structs:** If a struct has `N > 1` fields, `impl From1<T1>` should be generated, where `T1` is the type of the first field, and all fields are initialized with `a1`. (As per `Readme.md` example for `MyStruct`).
        *   **Special Case for `From2` on 3-field structs:** If a struct has 3 fields, `impl From2<T1, T2>` should be generated, where `T1` and `T2` are the types of the first two fields, and fields are initialized as `field1: a1, field2: a2, field3: a2`.
    *   When applied to a struct with `#[from(SourceType)]` attributes:
        *   Generates `impl From<SourceType> for MyStruct`, converting `SourceType` to the type of the *first field* of `MyStruct` using `as` casting.
        *   Multiple `#[from(SourceType)]` attributes are supported.
    *   **Error Handling:**
        *   Emits a compilation error if applied to enums or unit structs.
        *   Emits a compilation error if applied to structs with more than 3 fields (current limitation for `FromN` traits).
        *   Emits a compilation error for invalid `#[from(SourceType)]` attribute syntax.

*   **`from!` Declarative Macro Behavior:**
    *   `from!()`: Expands to `Default::default()`. Requires the target struct to implement `Default`.
    *   `from!(arg1)`: Expands to `MyStruct::from1(arg1)`. Requires the target struct to implement `From1`.
    *   `from!(arg1, arg2)`: Expands to `MyStruct::from2(arg1, arg2)`. Requires the target struct to implement `From2`.
    *   `from!(arg1, arg2, arg3)`: Expands to `MyStruct::from3(arg1, arg2, arg3)`. Requires the target struct to implement `From3`.

*   **Doc Test Compliance:** All doc tests in `module/core/variadic_from/Readme.md` and `module/core/variadic_from/src/lib.rs` must compile and pass, reflecting the above behaviors.

### Crate Conformance Check Procedure
*   Step 1: Run `timeout 90 cargo test -p variadic_from_meta --all-targets` and verify no failures or warnings.
*   Step 2: Run `timeout 90 cargo clippy -p variadic_from_meta -- -D warnings` and verify no errors or warnings.
*   Step 3: Run `timeout 90 cargo test -p variadic_from --all-targets` and verify no failures or warnings.
*   Step 4: Run `timeout 90 cargo clippy -p variadic_from -- -D warnings` and verify no errors or warnings.
*   Step 5: Run `timeout 90 cargo test -p variadic_from --doc` and verify no failures.

### Increments
*   ✅ Increment 1: Define `FromN` Traits and `from!` Macro.
    *   **Goal:** Define the `From1`, `From2`, `From3` traits in `module/core/variadic_from/src/lib.rs` and implement the `from!` declarative macro.
    *   **Steps:**
        *   Step 1: Define `From1`, `From2`, `From3` traits in `module/core/variadic_from/src/lib.rs`.
        *   Step 2: Implement the `from!` declarative macro in `module/core/variadic_from/src/lib.rs` to dispatch to `FromN` traits.
        *   Step 3: Update `module/core/variadic_from/tests/inc/variadic_from_manual_test.rs` to use `FromN` traits and `from!` macro for manual implementations, mirroring `Readme.md` examples.
        *   Step 4: Update `module/core/variadic_from/tests/inc/variadic_from_only_test.rs` to use `the_module::from!` and correctly test multi-field structs.
        *   Step 5: Perform Increment Verification.
        *   Step 6: Perform Crate Conformance Check.
    *   **Increment Verification:**
        *   Run `timeout 90 cargo build -p variadic_from` and verify exit code 0.
        *   Run `timeout 90 cargo test -p variadic_from --test variadic_from_tests` and verify exit code 0.
    *   **Commit Message:** `feat(variadic_from): Define FromN traits and from! macro`

*   ⚫ Increment 2: Refactor `variadic_from_meta` for Multi-Field Structs and Variadic `From`.
    *   **Goal:** Modify the `VariadicFrom` derive macro in `variadic_from_meta` to handle multi-field structs and generate `FromN` and tuple `From` implementations, including special cases for `From1` and `From2` on multi-field structs.
    *   **Steps:**
        *   Step 1: Update `variadic_from_meta/src/lib.rs` to parse multi-field structs.
        *   Step 2: Generate `impl FromN` for structs based on the number of fields, including special cases for `From1` on 2-field and 3-field structs, and `From2` on 3-field structs.
        *   Step 3: Generate `impl From<(T1, ..., TN)>` for tuple conversions.
        *   Step 4: Update `module/core/variadic_from/tests/inc/variadic_from_derive_test.rs` to use the derive macro on multi-field structs, mirroring `Readme.md` examples.
        *   Step 5: Perform Increment Verification.
        *   Step 6: Perform Crate Conformance Check.
    *   **Increment Verification:**
        *   Run `timeout 90 cargo test -p variadic_from --test variadic_from_tests` and verify exit code 0.
        *   Run `timeout 90 cargo test -p variadic_from_meta` and verify exit code 0.
    *   **Commit Message:** `feat(variadic_from_meta): Support multi-field structs and variadic From`

*   ⚫ Increment 3: Implement `#[from(Type)]` Attribute Handling.
    *   **Goal:** Extend the `VariadicFrom` derive macro to process `#[from(Type)]` attributes and generate `impl From<Type> for MyStruct` conversions.
    *   **Steps:**
        *   Step 1: Modify `variadic_from_meta/src/lib.rs` to parse `#[from(Type)]` attributes.
        *   Step 2: Generate `impl From<Type> for MyStruct` where `Type` is converted to the first field's type.
        *   Step 3: Update `module/core/variadic_from/tests/inc/variadic_from_derive_test.rs` to include structs with `#[from(Type)]` attributes and corresponding assertions.
        *   Step 4: Perform Increment Verification.
        *   Step 5: Perform Crate Conformance Check.
    *   **Increment Verification:**
        *   Run `timeout 90 cargo test -p variadic_from --test variadic_from_tests` and verify exit code 0.
        *   Run `timeout 90 cargo test -p variadic_from_meta` and verify exit code 0.
    *   **Commit Message:** `feat(variadic_from_meta): Implement #[from(Type)] attribute handling`

*   ⚫ Increment 4: Update Doc Tests and Final Verification.
    *   **Goal:** Ensure all doc tests in `Readme.md` and `src/lib.rs` pass, and perform final overall verification.
    *   **Steps:**
        *   Step 1: Run `timeout 90 cargo test -p variadic_from --doc` and fix any failures by adjusting the doc comments to reflect the correct usage and generated code.
        *   Step 2: Perform final `cargo test -p variadic_from --all-targets`.
        *   Step 3: Perform final `cargo clippy -p variadic_from -p variadic_from_meta -- -D warnings`.
        *   Step 4: Run `git status` to ensure a clean working directory.
    *   **Increment Verification:**
        *   Run `timeout 90 cargo test -p variadic_from --all-targets` and `timeout 90 cargo clippy -p variadic_from -p variadic_from_meta -- -D warnings` and verify exit code 0 for both.
        *   Run `timeout 90 cargo test -p variadic_from --doc` and verify no failures.
        *   Run `git status` and verify no uncommitted changes.
    *   **Commit Message:** `chore(variadic_from): Update doc tests and final verification`

### Changelog
*   **2025-06-29:**
    *   **Increment 1 (Previous):** Implemented manual `From` implementations for `MyStruct` and `NamedStruct` in `module/core/variadic_from/tests/inc/variadic_from_manual_test.rs` and `module/core/variadic_from/tests/inc/variadic_from_only_test.rs`. Ensured the test file is included in `module/core/variadic_from/tests/inc/mod.rs`. Temporarily commented out `variadic_from_meta` imports in `module/core/variadic_from/src/lib.rs` to allow `cargo build -p variadic_from` to pass.
    *   **Increment 2 (Previous):** Created the `variadic_from_meta` crate, including its `Cargo.toml` and `src/lib.rs` with a basic derive macro stub. Created `Readme.md` for `variadic_from_meta`. Updated `module/core/variadic_from/Cargo.toml` to add `variadic_from_meta` as a dependency and removed `derive_tools_meta`. Verified that both `variadic_from_meta` and `variadic_from` crates build successfully.
    *   **Increment 3 (Previous):** Implemented the core logic of the `VariadicFrom` derive macro in `module/core/variadic_from_meta/src/lib.rs`, including parsing `#[from(T)]` attributes and generating `impl From<T> for MyStruct` blocks. Created `module/core/variadic_from/tests/inc/variadic_from_derive_test.rs` and added its module declaration to `module/core/variadic_from/tests/inc/mod.rs`. Fixed `syn` v2.0 API usage, `field.index` access, and type casting in the macro. Cleaned up irrelevant test modules in `module/core/variadic_from/tests/inc/mod.rs` and fixed a doc comment in `module/core/variadic_from/tests/inc/variadic_from_only_test.rs`. Verified that `cargo test -p variadic_from --test variadic_from_tests` passes.
    *   **Increment 4 (Previous):** Uncommented `variadic_from_meta` imports and added `VariadicFrom` re-export in `module/core/variadic_from/src/lib.rs`. Removed `module/core/variadic_from/examples/variadic_from_trivial_expanded.rs`. Verified that `cargo test -p variadic_from --all-targets` passes.
    *   **Increment 5 (Previous):** Verified that `cargo test -p variadic_from --all-targets` and `cargo clippy -p variadic_from -p variadic_from_meta -- -D warnings` pass without errors or warnings. Addressed `missing documentation` warning in `module/core/variadic_from/tests/variadic_from_tests.rs`.
    *   **Increment 1 (Current):** Defined `From1`, `From2`, `From3` traits and `from!` declarative macro in `module/core/variadic_from/src/lib.rs`. Updated `module/core/variadic_from/tests/inc/variadic_from_manual_test.rs` and `module/core/variadic_from/tests/inc/variadic_from_only_test.rs` to use the new traits and macro. Verified successful build and test execution for `variadic_from`.

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

### External System Dependencies (Optional)
*   None.

### Notes & Insights
*   The `proc-macro` crate type has specific limitations regarding module visibility and `pub mod` declarations.
*   Careful error reporting from the macro is crucial for a good developer experience.
*   Doc tests in procedural macro crates often require `/// ```text` instead of `/// ```rust` because they cannot directly run macro examples.