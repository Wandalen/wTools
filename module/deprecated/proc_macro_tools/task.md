# Change Proposal for proc_macro_tools

### Task ID
*   `TASK-20250622-182800-FormerRefactor-V2`

### Requesting Context
*   **Requesting Crate/Project:** `former_meta`
*   **Driving Feature/Task:** Refactoring of `#[derive(Former)]` for enum unit variants.
*   **Link to Requester's Plan:** `../../../core/former_meta/plan.md`
*   **Date Proposed:** 2025-06-22

### Overall Goal of Proposed Change
*   To add new, and refine existing, generalized utility functions to `proc_macro_tools` that will simplify identifier case conversion and the handling of `syn::Generics` in procedural macros.

### Problem Statement / Justification
*   The `former_meta` crate contains logic for converting identifiers to different cases (e.g., `PascalCase` to `snake_case`) and for quoting parts of generic parameter lists (`impl` generics, `ty` generics, `where` clauses). This logic is common and would be beneficial to other procedural macros. Extracting and refining it into `proc_macro_tools` will improve code reuse, reduce duplication, and increase maintainability. The existing `GenericsRef` API can also be made more ergonomic.

### Proposed Solution / Specific Changes
*   **API Changes:**
    *   New public function in `proc_macro_tools::ident`:
        ```rust
        /// Creates a new `syn::Ident` from an existing one, converting it to the specified case.
        /// This is more ergonomic than `new_ident_from_cased_str` as it handles extracting the string and span.
        /// Handles raw identifiers (e.g., `r#fn`) correctly.
        pub fn cased_ident_from_ident(original: &syn::Ident, case: convert_case::Case) -> syn::Ident;
        ```
    *   Refinements in `proc_macro_tools::generic_params`:
        ```rust
        // In impl<'a> GenericsRef<'a>

        /// Creates a new `GenericsRef`. Alias for `new_borrowed`.
        pub fn new(generics: &'a syn::Generics) -> Self;

        // Change the return type of the following methods from Result<TokenStream> to TokenStream,
        // as the current implementation does not return errors.

        /// Returns tokens for the `impl` part of the generics, e.g., `<T: Trait>`.
        pub fn impl_generics_tokens_if_any(&self) -> proc_macro2::TokenStream;

        /// Returns tokens for the type part of the generics, e.g., `<T>`.
        pub fn ty_generics_tokens_if_any(&self) -> proc_macro2::TokenStream;

        /// Returns tokens for the `where` clause, e.g., `where T: Trait`.
        pub fn where_clause_tokens_if_any(&self) -> proc_macro2::TokenStream;

        /// Returns tokens for a full type path with generics, e.g., `MyType<T>`.
        pub fn type_path_tokens_if_any(&self, type_name: &syn::Ident) -> proc_macro2::TokenStream;
        ```
    *   Update `proc_macro_tools::kw::KEYWORDS` to include Rust 2021 reserved keywords.

### Expected Behavior & Usage Examples (from Requester's Perspective)
*   ```rust
    // In former_meta:
    use proc_macro_tools::ident;
    use proc_macro_tools::generic_params::GenericsRef;

    let variant_ident = /* ... */;
    let method_ident = ident::cased_ident_from_ident(variant_ident, convert_case::Case::Snake);

    let generics = /* ... */;
    let generics_ref = GenericsRef::new(generics); // use new instead of new_borrowed
    let impl_generics = generics_ref.impl_generics_tokens_if_any(); // no .unwrap() needed
    let ty_generics = generics_ref.ty_generics_tokens_if_any();
    let where_clause = generics_ref.where_clause_tokens_if_any();
    ```

### Acceptance Criteria (for this proposed change)
*   The new function and API refinements are implemented and available in `proc_macro_tools`.
*   The new utilities are well-documented and have comprehensive unit tests.
*   The `former_meta` crate can successfully use these new utilities to refactor its unit variant handling.

### Potential Impact & Considerations
*   **Breaking Changes:** The change of return type on `GenericsRef` methods is a breaking change for any existing users of those methods. Given the context of this tool suite, this is likely acceptable.
*   **Dependencies:** Adds a dependency on `convert_case` to `proc_macro_tools` if not already present.
*   **Testing:** New unit tests must be added to `proc_macro_tools` to cover the new functionality and changes.