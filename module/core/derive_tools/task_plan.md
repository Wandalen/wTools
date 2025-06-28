# Task Plan: Fix `derive_tools` after `macro_tools` refactoring

### Goal
*   The primary goal is to fix the `module/core/derive_tools` crate, which stopped working after a refactoring of the `macro_tools` crate. This involves systematically identifying and resolving API incompatibilities, primarily within `derive_tools_meta`, which is a direct dependency of `derive_tools` and an indirect consumer of `macro_tools`. The process emphasizes small, incremental steps to avoid regressions.

### Ubiquitous Language (Vocabulary)
*   `Target Crate`: `module/core/derive_tools`
*   `External Crate`: `module/core/macro_tools`
*   `derive_tools_meta`: The procedural macro crate that `derive_tools` depends on.
*   `derive_tools_attributes`: A new helper crate to be created, which will contain shared attribute parsing logic for `derive_tools_meta`.
*   `API Incompatibility`: Changes in `macro_tools`'s public interface that break `derive_tools_meta`.
*   `Const Generics`: A specific type of generic parameter (`const N: usize`) that caused "unexpected `const` parameter declaration" and "unconstrained const parameter" errors due to incorrect token generation by `macro_tools`.
*   `Clippy Lints`: Static analysis warnings and errors from `clippy`.

### Progress
*   🚀 Phase 1 Complete (Increments 1-5)
*   🚧 Phase 2 In Progress (Increment 6)
*   Key Milestones Achieved: ✅ `derive_tools_meta` compiles (except for attribute import issues), `clippy` warnings addressed in `deref`, `deref_mut`, `as_ref`, `as_mut`, `variadic_from`, `not`, `phantom`, `index`, `index_mut`, `new`, `inner_from`. ⏳ `macro_tools` fix proposed.

### Target Crate/Library
*   `module/core/derive_tools`

### Relevant Context
*   Files to Include (for AI's reference, if `read_file` is planned, primarily from Target Crate):
    *   `module/core/derive_tools/Cargo.toml`
    *   `module/core/derive_tools/src/lib.rs`
    *   `module/core/derive_tools/tests/inc/deref/basic_test.rs`
    *   `module/core/derive_tools_meta/Cargo.toml`
    *   `module/core/derive_tools_meta/src/lib.rs`
    *   `module/core/derive_tools_meta/src/derive/from.rs`
    *   `module/core/derive_tools_meta/src/derive/deref.rs`
    *   `module/core/derive_tools_meta/src/derive/deref_mut.rs`
    *   `module/core/derive_tools_meta/src/derive/as_ref.rs`
    *   `module/core/derive_tools_meta/src/derive/as_mut.rs`
    *   `module/core/derive_tools_meta/src/derive/variadic_from.rs`
    *   `module/core/derive_tools_meta/src/derive/not.rs`
    *   `module/core/derive_tools_meta/src/derive/phantom.rs`
    *   `module/core/derive_tools_meta/src/derive/index.rs`
    *   `module/core/derive_tools_meta/src/derive/index_mut.rs`
    *   `module/core/derive_tools_meta/src/derive/new.rs`
    *   `module/core/derive_tools_meta/src/derive/inner_from.rs`
    *   `module/core/derive_tools_meta/src/attributes.rs` (to be moved to new crate)
*   Crates for Documentation (for AI's reference, if `read_file` on docs is planned):
    *   `derive_tools_meta`
    *   `macro_tools`
    *   `derive_tools_attributes` (new crate)
*   External Crates Requiring `task.md` Proposals (if any identified during planning):
    *   `module/core/macro_tools` (Reason: `macro_tools::generic_params::decompose` generates incorrect tokens for `const` generic parameters, causing `E0207` and `E0284` in `derive_tools_meta`'s `Deref` and `DerefMut` derives.)

### Expected Behavior Rules / Specifications (for Target Crate)
*   The `derive_tools` crate should compile and pass all its tests without errors or warnings.
*   The derive macros in `derive_tools_meta` should correctly generate code for various struct and enum types, including those with generic parameters (once the `macro_tools` fix is applied).
*   `clippy` should report no warnings or errors when run on `derive_tools` and `derive_tools_meta`.

### Crate Conformance Check Procedure
*   Run `timeout 90 cargo test -p derive_tools --all-targets` and verify no failures or warnings.
*   Run `timeout 90 cargo clippy -p derive_tools -- -D warnings` and verify no errors or warnings.

### Increments
*   ✅ Increment 1: Fix `derive_tools_meta/src/derive/from.rs` for `macro_tools` API changes.
    *   **Goal:** Update `from.rs` to align with the new `macro_tools` API, specifically `attr::has_debug` and `item_struct::field_types`.
    *   **Steps:**
        *   Update `from.rs` to use `attr::has_debug` correctly.
        *   Update `from.rs` to use `item_struct::field_types` correctly.
        *   Run `cargo build -p derive_tools_meta` to verify compilation.
    *   **Increment Verification:**
        *   `execute_command` to run `cargo build -p derive_tools_meta` and verify exit code 0.
    *   **Commit Message:** `fix(derive_tools_meta): Update from.rs for macro_tools API`

*   ✅ Increment 2: Fix `derive_tools_meta/src/derive/deref.rs` and `deref_mut.rs` for `macro_tools` API changes and `clippy` warnings.
    *   **Goal:** Update `deref.rs` and `deref_mut.rs` to align with the new `macro_tools` API, specifically `attr::has_debug`, and address `clippy` warnings.
    *   **Steps:**
        *   Update `deref.rs` to use `attr::has_debug` correctly.
        *   Update `deref_mut.rs` to use `attr::has_debug` correctly.
        *   Address `clippy::question_mark` by replacing `?` with `match` for `attr::has_debug`.
        *   Address `clippy::needless_raw_string_hashes` by removing `r#` if not strictly needed.
        *   Address `clippy::empty_line_after_doc_comments` and `clippy::doc_markdown` by removing `zzz : qqq : implement` comments and adding proper doc comments.
        *   Address `clippy::format_in_format_args` by using `format_args!` or pre-formatting.
        *   Address `clippy::too_many_lines` by considering refactoring if necessary (not critical for this increment).
    *   **Increment Verification:**
        *   `execute_command` to run `cargo build -p derive_tools_meta` and verify exit code 0.
    *   **Commit Message:** `fix(derive_tools_meta): Update deref.rs and deref_mut.rs for macro_tools API and clippy`

*   ✅ Increment 3: Fix `derive_tools_meta/src/derive/as_ref.rs`, `as_mut.rs`, `variadic_from.rs`, `not.rs`, `phantom.rs`, `index.rs`, `index_mut.rs` for `macro_tools` API changes and `clippy` warnings.
    *   **Goal:** Update the remaining derive macros to align with the new `macro_tools` API and address `clippy` warnings.
    *   **Steps:**
        *   Update `as_ref.rs` to use `syn::ItemStruct` and `item_struct::first_field_type`.
        *   Update `as_mut.rs` to use `syn::ItemStruct` and `item_struct::first_field_type`.
        *   Update `variadic_from.rs` to use `syn::ItemStruct` and `syn::Fields`.
        *   Update `not.rs` to use `ItemAttributes` and `FieldAttributes` from a centralized `attr` module.
        *   Update `phantom.rs` to use `syn::ItemStruct`, `attr::has_debug`, and `phantom::add_to_item`.
        *   Update `index.rs` to use `attr::has_debug`, `ItemAttributes::from_attrs`, and `FieldAttributes::from_attrs`.
        *   Update `index_mut.rs` to use `attr::has_debug`, `ItemAttributes::from_attrs`, and `FieldAttributes::from_attrs`.
        *   Centralize `ItemAttributes` and `FieldAttributes` into a new `attr.rs` module and update `lib.rs` to expose it.
        *   Address `clippy` warnings: `question_mark`, `needless_raw_string_hashes`, `empty_line_after_doc_comments`, `doc_markdown`, `format_in_format_args`, `too_many_lines`, `unnecessary_map_or`, `cloned_instead_of_copied`, `len_zero`, `needless_pass_by_value`, `used_underscore_binding`.
    *   **Increment Verification:**
        *   `execute_command` to run `cargo build -p derive_tools_meta` and verify exit code 0.
    *   **Commit Message:** `fix(derive_tools_meta): Update remaining derives for macro_tools API and clippy`

*   ✅ Increment 4: Fix `derive_tools_meta/src/derive/new.rs` for `macro_tools` API changes and `clippy` warnings.
    *   **Goal:** Update `new.rs` to align with the new `macro_tools` API and address `clippy` warnings.
    *   **Steps:**
        *   Remove `#[path]` attributes and `use` statements for `field_attributes` and `item_attributes`.
        *   Import `ItemAttributes` and `FieldAttributes` from `crate::derive::attr`.
        *   Address `clippy::question_mark` by replacing `variants_result?` with a `match` statement.
        *   Address `clippy::format_in_format_args` by using `format_args!` or by pre-formatting the inner strings.
        *   Address `clippy::needless_raw_string_hashes` by removing `r#` if not strictly needed.
        *   Address `clippy::empty_line_after_doc_comments` and `clippy::doc_markdown` by removing the `zzz : qqq : implement` comments and adding proper doc comments.
        *   Address `clippy::used_underscore_binding` by removing `_generics_with_defaults`.
        *   Address `clippy::len_zero` by replacing `fields.len() == 0` with `fields.is_empty()`.
        *   Delete `module/core/derive_tools_meta/src/derive/from/field_attributes.rs` and `module/core/derive_tools_meta/src/derive/from/item_attributes.rs`.
    *   **Increment Verification:**
        *   `execute_command` to run `cargo build -p derive_tools_meta` and verify exit code 0.
    *   **Commit Message:** `fix(derive_tools_meta): Update new.rs for macro_tools API and clippy`

*   ✅ Increment 5: Fix `derive_tools_meta/src/derive/inner_from.rs` for `macro_tools` API changes.
    *   **Goal:** Update `inner_from.rs` to align with the new `macro_tools` API and address `clippy` warnings.
    *   **Steps:**
        *   Update imports to use `crate::derive::attr::AttributePropertyDebug`.
        *   Address `clippy::len_zero` by replacing `field_types.len() == 0` with `field_types.is_empty()`.
        *   Address `clippy::format_in_format_args` by using `format_args!` or by pre-formatting.
    *   **Increment Verification:**
        *   `execute_command` to run `cargo build -p derive_tools_meta` and verify exit code 0.
    *   **Commit Message:** `fix(derive_tools_meta): Update inner_from.rs for macro_tools API and clippy`

*   ⚫ Increment 6: Create `derive_tools_attributes` helper crate and refactor `derive_tools_meta` to use it.
    *   **Goal:** Create a new helper crate `derive_tools_attributes` to house shared attribute parsing logic (`FieldAttributes`, `ItemAttributes`, etc.) and update `derive_tools_meta` to depend on and use this new crate.
    *   **Steps:**
        *   Create a new crate `module/core/derive_tools_attributes` using `cargo new --lib ../../core/derive_tools_attributes` from the current terminal's working directory.
        *   Move `module/core/derive_tools_meta/src/attributes.rs` content to `module/core/derive_tools_attributes/src/lib.rs`.
        *   Update `module/core/derive_tools_meta/Cargo.toml` to add `derive_tools_attributes` as a dependency.
        *   Update all derive macro files in `module/core/derive_tools_meta/src/derive/` to import `FieldAttributes`, `ItemAttributes`, and `AttributePropertyDebug` from `derive_tools_attributes`.
        *   Address remaining `clippy` warnings and compilation errors related to attribute imports and usage.
        *   Delete redundant attribute files (e.g., `src/derive/from/field_attributes.rs`, `src/derive/index/item_attributes.rs`, etc.) if they still exist.
    *   **Increment Verification:**
        *   `execute_command` to run `cargo build -p derive_tools_meta` and verify exit code 0.
        *   `execute_command` to run `cargo clippy -p derive_tools_meta -- -D warnings` and verify no errors or warnings.
    *   **Commit Message:** `feat(derive_tools_meta): Introduce derive_tools_attributes helper crate`

### Changelog
*   **2025-06-28:**
    *   `fix(derive_tools_meta): Update from.rs for macro_tools API`
    *   `fix(derive_tools_meta): Update deref.rs and deref_mut.rs for macro_tools API and clippy`
    *   `fix(derive_tools_meta): Update remaining derives for macro_tools API and clippy`
    *   `fix(derive_tools_meta): Update new.rs for macro_tools API and clippy`
    *   `fix(derive_tools_meta): Update inner_from.rs for macro_tools API and clippy`

### Task Requirements
*   All fixes must be incremental and verifiable.
*   Prioritize fixing compilation errors, then `clippy` warnings.
*   Changes to `macro_tools` must be proposed via `task.md`.

### Project Requirements
*   Must use Rust 2021 edition.
*   All new APIs must be async. (Not directly applicable to proc macros, but noted for general project context).

### Assumptions
*   The `macro_tools` fix will eventually be implemented, allowing full test suite to pass.
*   The `derive_tools_meta` crate is the primary focus for direct code modifications.

### Out of Scope
*   Implementing the `macro_tools` fix directly.
*   Major refactoring of `derive_tools` or `derive_tools_meta` beyond what's necessary to address API incompatibilities and `clippy` warnings.

### External System Dependencies (Optional)
*   None.

### Notes & Insights
*   The `search_and_replace` tool has been problematic for complex changes, requiring full file writes.
*   The `const` generics issue in `macro_tools` is a significant blocker for full test pass.
*   `ItemAttributes` and `FieldAttributes` were causing visibility issues and have been centralized into `src/derive/attr.rs`, but this approach failed due to proc-macro crate limitations. A new helper crate `derive_tools_attributes` is proposed as a solution.
*   Encountered issues with `mkdir` and `cwd` parameter interpretation, leading to a workaround using `cargo new --lib` with a relative path.