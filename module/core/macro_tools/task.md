# Change Proposal for macro_tools

### Task ID
*   `TASK-20250628-181200-FixConstGenerics`

### Requesting Context
*   **Requesting Crate/Project:** `derive_tools`
*   **Driving Feature/Task:** Fixing `derive_tools` after `macro_tools` refactoring, specifically addressing issues with `Deref` and `DerefMut` derives when used with `const` generic parameters.
*   **Link to Requester's Plan:** `../derive_tools/task_plan.md`
*   **Date Proposed:** 2025-06-28

### Overall Goal of Proposed Change
*   To modify the `decompose` function in `macro_tools::generic_params` so that the `generics_for_ty` component correctly extracts only the identifier for `const` parameters, instead of the full `const N : usize` declaration. This will prevent "unexpected `const` parameter declaration" and "unconstrained const parameter" errors in downstream procedural macros that use this component for type paths.

### Problem Statement / Justification
*   The `derive_tools` crate, which depends on `derive_tools_meta` (which in turn depends on `macro_tools`), is failing its tests when deriving `Deref` and `DerefMut` for structs with `const` generic parameters.
*   The error messages, such as "unexpected `const` parameter declaration" and "unconstrained const parameter", indicate that the `generics_for_ty` component returned by `macro_tools::generic_params::decompose` is incorrectly including the full `const N : usize` syntax in type paths (e.g., `MyStruct<const N: usize>`) instead of just the identifier (`MyStruct<N>`).
*   This behavior is problematic because type paths should only contain the identifiers of generic parameters, not their full declarations. The current implementation of `decompose` for `ConstParam` in `generics_for_ty` is cloning the entire `ConstParam` structure, including `const_token`, `colon_token`, and `ty`, which is suitable for `impl` generics but not for type generics.

### Proposed Solution / Specific Changes
*   **File:** `module/core/macro_tools/src/generic_params.rs`
*   **Function:** `decompose`
*   **Specific Change:** Modify the `syn::GenericParam::Const` branch within the `decompose` function's loop for `generics_for_ty`. Instead of cloning the entire `ConstParam`, construct a new `ConstParam` that only retains the `ident` and `ty`, setting `const_token`, `colon_token`, `eq_token`, and `default` to `None` or their default empty values, similar to how `TypeParam` is handled for `generics_for_ty`.

*   **Conceptual Code Change (within `decompose` function, `syn::GenericParam::Const` branch for `generics_for_ty`):**
    ```rust
    // Current (problematic)
    // let ty_param = syn::GenericParam::Const( syn::ConstParam
    // {
    //   attrs : vec![],
    //   const_token : const_param.const_token,
    //   ident : const_param.ident.clone(),
    //   colon_token : const_param.colon_token,
    //   ty : const_param.ty.clone(),
    //   eq_token : None,
    //   default : None,
    // });

    // Proposed (simplified for type generics)
    let ty_param = syn::GenericParam::Const( syn::ConstParam
    {
      attrs : vec![],
      const_token : Default::default(), // Should be empty/None for type generics
      ident : const_param.ident.clone(),
      colon_token : None, // Should be None for type generics
      ty : const_param.ty.clone(), // Keep type for context, but not part of path
      eq_token : None,
      default : None,
    });
    ```
    *Note: The `ty` field of `ConstParam` is part of its definition, but it should not be rendered in the type path. The `quote!` macro should handle this correctly if `const_token` and `colon_token` are absent.*

### Expected Behavior & Usage Examples (from Requester's Perspective)
*   After this change, `macro_tools::generic_params::decompose` should return `generics_for_ty` that, when quoted, produces only the identifier for `const` parameters (e.g., `N` instead of `const N : usize`).
*   This will allow `derive_tools_meta`'s `Deref` and `DerefMut` macros to generate valid Rust code for structs with `const` generics, resolving the current compilation errors in `derive_tools`.

### Acceptance Criteria (for this proposed change)
*   The `decompose` function in `macro_tools::generic_params` is modified as described.
*   Running `cargo test -p derive_tools` (after this fix is applied to `macro_tools`) passes without the "unexpected `const` parameter declaration" or "unconstrained const parameter" errors.
*   The generated code for `Deref` and `DerefMut` for structs with `const` generics is syntactically correct and compiles.

### Potential Impact & Considerations
*   **Breaking Changes:** This is a bug fix in the behavior of `decompose` for `const` parameters in `generics_for_ty`. It should not introduce new breaking changes, as the previous behavior was incorrect for type paths.
*   **Dependencies:** No new dependencies.
*   **Performance:** No significant performance impact.
*   **Testing:** Existing tests for `generic_params::decompose` should be reviewed to ensure they cover `const` parameters correctly for all returned components. New tests might be needed to specifically assert the format of `generics_for_ty` for `const` parameters.

### Alternatives Considered (Optional)
*   Attempting to work around this issue within `derive_tools_meta` by manually parsing and re-quoting `const` parameters. This was rejected because the root cause is in `macro_tools`, and fixing it there is the most robust and maintainable solution.

### Notes & Open Questions
*   Confirm that setting `const_token` and `colon_token` to `Default::default()` or `None` (if applicable) for `ConstParam` in `generics_for_ty` is the correct way to make `quote!` render only the identifier.