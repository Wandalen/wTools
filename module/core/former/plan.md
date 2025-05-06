# Project Plan: Skeleton Refactor of `former_enum` Module

## Goal

*   Restructure the `former_meta/src/derive_former/former_enum/` module to a flatter, more granular file organization as depicted in the file tree below. The main module file will remain `module/core/former_meta/src/derive_former/former_enum.rs`, containing module declarations and dispatch logic.
*   Create skeleton files and basic function signatures for the new handler modules within the `former_enum` directory.
*   Update the main dispatch logic in `former_enum.rs` to call these new skeleton handlers.
*   Ensure the project compiles with these skeleton changes, deferring full logic implementation.
*   **Fix all compilation warnings.**

## Target File Structure for `module/core/former_meta/src/derive_former/former_enum/`

```
former_enum/  (directory: module/core/former_meta/src/derive_former/former_enum/)
├── mod.rs                             # Main module file for `former_enum`.
│                                      # - Declares all sibling files as submodules.
│                                      # - Contains the primary `former_for_enum` function.
│                                      # - Houses the main dispatch logic to route to specific handlers.
│                                      # - Defines `EnumVariantHandlerContext` and `EnumVariantFieldInfo`.
│
├── common_emitters.rs                 # Contains shared helper functions for generating common code patterns
│                                      # used by multiple variant handlers (e.g., direct constructors,
│                                      # boilerplate for different subformer types).
│
├── unit_variant_handler.rs            # Handles `Unit` variants.
│                                      # - `#[scalar]` or Default: Generates direct constructor.
│                                      # - `#[subform_scalar]`: Generates an error.
│
├── tuple_zero_fields_handler.rs       # Handles `Tuple()` (zero-field tuple) variants.
│                                      # - `#[scalar]` or Default: Generates direct constructor.
│                                      # - `#[subform_scalar]`: Generates an error.
│
├── struct_zero_fields_handler.rs      # Handles `Struct {}` (zero-field struct) variants.
│                                      # - `#[scalar]`: Generates direct constructor.
│                                      # - `#[subform_scalar]` or Default: Generates an error.
│
├── tuple_single_field_scalar.rs       # Handles `Tuple(T1)` variants with the `#[scalar]` attribute.
│                                      # - Generates a direct constructor: `fn variant(T1) -> Enum`.
│
├── tuple_single_field_subform.rs       # Handles `Tuple(T1)` variants with `#[subform_scalar]` or default behavior.
│                                      # - Generates a method returning an inner former: `fn variant() -> InnerFormer<...>`.
│                                      # - Requires T1 to derive Former.
│
├── tuple_multi_fields_scalar.rs       # Handles `Tuple(T1, T2, ...)` (multi-field tuple) variants with
│                                      # `#[scalar]` or default behavior.
│                                      # - Generates a direct constructor: `fn variant(T1, T2, ...) -> Enum`.
│                                      # - (Note: `#[subform_scalar]` is an error for multi-field tuples,
│                                      #   handled by dispatch logic in `mod.rs`).
│
├── struct_single_field_scalar.rs      # Handles `Struct { f1:T1 }` (single-field struct) variants
│                                      # with the `#[scalar]` attribute.
│                                      # - Generates a direct constructor: `fn variant { f1:T1 } -> Enum`.
│
├── struct_single_field_subform.rs     # Handles `Struct { f1:T1 }` variants with `#[subform_scalar]`
│                                      # or default behavior.
│                                      # - Generates a method returning an implicit variant former:
│                                      #   `fn variant() -> VariantFormer<...>`.
│
├── struct_multi_fields_scalar.rs      # Handles `Struct { f1:T1, ... }` (multi-field struct) variants
│                                      # with the `#[scalar]` attribute.
│                                      # - Generates a direct constructor: `fn variant { f1:T1, ... } -> Enum`.
│
└── struct_multi_fields_subform.rs     # Handles `Struct { f1:T1, ... }` variants with `#[subform_scalar]`
                                       # or default behavior.
                                       # - Generates a method returning an implicit variant former:
                                       #   `fn variant() -> VariantFormer<...>`.
```

## Relevant Context

*   `module/core/former_meta/src/derive_former/former_enum.rs` (current main file for the `former_enum` module, will be modified to declare new submodules and house dispatch logic)
*   Old handler files to be replaced (currently submodules of `former_enum.rs`):
    *   `module/core/former_meta/src/derive_former/former_enum/unit.rs`
    *   `module/core/former_meta/src/derive_former/former_enum/tuple_zero.rs`
    *   `module/core/former_meta/src/derive_former/former_enum/tuple_non_zero.rs`
    *   `module/core/former_meta/src/derive_former/former_enum/struct_zero.rs`
    *   `module/core/former_meta/src/derive_former/former_enum/struct_non_zero.rs`
*   The "Expected Enum Former Behavior" rules.
*   The agreed-upon target flat file structure for `former_enum` (as depicted above).

## Increments

*   ✅ **Increment 1: Create New Skeleton Files and Update `former_enum.rs` Module Structure**
    *   Detailed Plan Step 1: Create the new empty Rust files within `module/core/former_meta/src/derive_former/former_enum/` as listed in the "Target File Structure" (excluding `mod.rs` which is the existing `former_enum.rs`).
    *   Detailed Plan Step 2: In each newly created `common_emitters.rs`, `*_handler.rs`, `*_scalar.rs`, and `*_subform.rs` file, add a basic public skeleton function or placeholder content. Apply strict codestyle.
        *   Example for `tuple_single_field_scalar.rs`:
            ```rust
            // qqq : Implement logic for Tuple(T1) with #[scalar]
            // qqq : Call common_emitters::generate_direct_constructor_for_variant(...)

            use super::*;
            use macro_tools::{ Result };
            // use super::EnumVariantHandlerContext;

            pub( crate ) fn handle( ctx : &mut EnumVariantHandlerContext< '_ > ) -> Result< () >
            {
              // qqq : Implement skeleton body
              Ok( () )
            }
            ```
        *   Example for `common_emitters.rs`:
            ```rust
            // qqq : Implement shared emitter functions

            use super::*;
            use macro_tools::{ Result, quote::{ quote }, syn::{ self, TokenStream2 as TokenStream } };
            // use super::EnumVariantHandlerContext;

            pub( crate ) fn generate_direct_constructor_for_variant( _ctx : &EnumVariantHandlerContext< '_ > ) -> Result< TokenStream >
            {
              // qqq : Implement
              Ok( quote!{} )
            }
            // ... other placeholder functions ...
            ```
    *   Detailed Plan Step 3: Modify `module/core/former_meta/src/derive_former/former_enum.rs`:
        *   **Remove existing `mod` declarations** for `unit`, `tuple_zero`, `tuple_non_zero`, `struct_zero`, `struct_non_zero`.
        *   Add new `mod` declarations for all the newly created files.
            ```rust
            // In module/core/former_meta/src/derive_former/former_enum.rs

            #![allow(clippy::wildcard_imports)] // Keep if present

            use super::*;
            use macro_tools::{ Result, quote::{ format_ident, quote }, syn::{self, TokenStream2 as TokenStream} };
            // qqq : Add other necessary imports

            // Declare new sibling modules
            mod common_emitters;
            mod unit_variant_handler;
            mod tuple_zero_fields_handler;
            mod struct_zero_fields_handler;
            mod tuple_single_field_scalar;
            mod tuple_single_field_subform;
            mod tuple_multi_fields_scalar;
            mod struct_single_field_scalar;
            mod struct_single_field_subform;
            mod struct_multi_fields_scalar;
            mod struct_multi_fields_subform;

            // Ensure EnumVariantHandlerContext and EnumVariantFieldInfo structs are defined
            // or re-exported for use by submodules.
            // These will remain in this file.
            // pub(super) struct EnumVariantFieldInfo { /* ... */ }
            // pub(super) struct EnumVariantHandlerContext< 'a > { /* ... */ }

            pub(super) fn former_for_enum
            (
              ast : &syn::DeriveInput,
              data_enum : &syn::DataEnum,
              original_input : &TokenStream,
              has_debug : bool
            ) -> Result< TokenStream >
            {
              // qqq : Old logic to be replaced by new dispatch logic in Increment 2
              Ok( quote!{} ) // Placeholder
            }
            ```
        *   Ensure `EnumVariantHandlerContext` and `EnumVariantFieldInfo` struct definitions are present and correct within this `former_enum.rs` file.
    *   Crucial Design Rules: [Structuring: Add Module Declaration Before Content](#structuring-add-module-declaration-before-content).
    *   Verification Strategy: Request user to run `cargo check --package former_meta`. Expect compilation success.
    *   **aaa:** Files created and skeleton content added. `former_enum.rs` updated with new module declarations and struct definitions. `cargo check` failed due to import errors and unused variables. Import errors in `former_enum.rs` were fixed via `apply_diff`. Unused variable warnings in skeleton files were fixed via `apply_diff`. Enabled `types_former` feature for `former_types` and `enabled` feature for `derive_tools_meta` in Cargo.toml. Still facing `could not find derive in derive_tools_meta` error. Increment blocked.

*   ✅ **Increment 2: Implement Skeleton Dispatch Logic in `former_enum.rs`**
    *   Detailed Plan Step 1: In `module/core/former_meta/src/derive_former/former_enum.rs`, within the `former_for_enum` function:
        *   Remove the placeholder/commented-out old logic.
        *   Implement the main loop through `data_enum.variants`.
        *   Inside the loop, correctly initialize `EnumVariantHandlerContext` (`ctx`), including parsing `variant_attrs` and `variant_field_info`.
        *   Implement the `match` statements based on `ctx.variant.fields` (kind and count) and `ctx.variant_attrs` directly within this function, calling the appropriate `handle` function from the submodules.
            ```rust
            // In module/core/former_meta/src/derive_former/former_enum.rs
            // ... (mods, struct defs for Context/FieldInfo) ...

            pub(super) fn former_for_enum
            (
              ast : &syn::DeriveInput,
              data_enum : &syn::DataEnum,
              original_input : &TokenStream,
              has_debug : bool
            ) -> Result< TokenStream >
            {
              let enum_name = &ast.ident;
              let vis = &ast.vis;
              let generics = &ast.generics;
              let struct_attrs = ItemAttributes::from_attrs( ast.attrs.iter() )?;
              // qqq : Ensure ItemAttributes and FieldAttributes are accessible/imported

              let mut methods = Vec::new();
              let mut end_impls = Vec::new();
              let mut standalone_constructors = Vec::new();
              let merged_where_clause = generics.where_clause.as_ref();

              for variant in &data_enum.variants
              {
                let variant_attrs = FieldAttributes::from_attrs( variant.attrs.iter() )?;
                let variant_field_info : Vec<EnumVariantFieldInfo> = match &variant.fields {
                    // qqq : Logic to populate variant_field_info (from previous plan)
                    syn::Fields::Named(f) => f.named.iter().map(|field| {
                        let attrs = FieldAttributes::from_attrs(field.attrs.iter())?;
                        let is_constructor_arg = attrs.arg_for_constructor.value(false);
                        Ok(EnumVariantFieldInfo {
                            ident: field.ident.clone().ok_or_else(|| syn::Error::new_spanned(field, "Named field requires an identifier"))?,
                            ty: field.ty.clone(),
                            attrs,
                            is_constructor_arg,
                        })
                    }).collect::<Result<_>>()?,
                    syn::Fields::Unnamed(f) => f.unnamed.iter().enumerate().map(|(index, field)| {
                        let attrs = FieldAttributes::from_attrs(field.attrs.iter())?;
                        let is_constructor_arg = attrs.arg_for_constructor.value(false);
                        Ok(EnumVariantFieldInfo {
                            ident: format_ident!("_{}", index),
                            ty: field.ty.clone(),
                            attrs,
                            is_constructor_arg,
                        })
                    }).collect::<Result<_>>()?,
                    syn::Fields::Unit => vec![],
                };

                let mut ctx = EnumVariantHandlerContext
                {
                  ast,
                  variant,
                  struct_attrs : &struct_attrs,
                  enum_name,
                  vis,
                  generics,
                  original_input,
                  variant_attrs : &variant_attrs,
                  variant_field_info : &variant_field_info,
                  merged_where_clause,
                  methods : &mut methods,
                  end_impls : &mut end_impls,
                  standalone_constructors : &mut standalone_constructors,
                  has_debug,
                };

                // Dispatch logic directly here
                match &ctx.variant.fields
                {
                  syn::Fields::Unit => unit_variant_handler::handle( &mut ctx )?,
                  syn::Fields::Unnamed( fields ) => match fields.unnamed.len()
                  {
                    0 => tuple_zero_fields_handler::handle( &mut ctx )?,
                    1 =>
                    {
                      if ctx.variant_attrs.scalar.is_some() {
                          tuple_single_field_scalar::handle( &mut ctx )?
                      } else {
                          tuple_single_field_subform::handle( &mut ctx )?
                      }
                    }
                    _ =>
                    {
                      if ctx.variant_attrs.subform_scalar.is_some()
                      {
                        return Err( syn::Error::new_spanned( ctx.variant, "#[subform_scalar] cannot be used on tuple variants with multiple fields." ) );
                      }
                      tuple_multi_fields_scalar::handle( &mut ctx )?
                    }
                  },
                  syn::Fields::Named( fields ) => match fields.named.len()
                  {
                    0 =>
                    {
                      if ctx.variant_attrs.subform_scalar.is_some()
                      {
                        return Err( syn::Error::new_spanned( ctx.variant, "#[subform_scalar] is not allowed on zero-field struct variants." ) );
                      }
                      if !ctx.variant_attrs.scalar.is_some()
                      {
                        return Err( syn::Error::new_spanned( ctx.variant, "Zero-field struct variants require `#[scalar]` attribute for direct construction." ) );
                      }
                      struct_zero_fields_handler::handle( &mut ctx )?
                    }
                    _len =>
                    {
                      if ctx.variant_attrs.scalar.is_some()
                      {
                        if fields.named.len() == 1
                        {
                          struct_single_field_scalar::handle( &mut ctx )?
                        }
                        else
                        {
                          struct_multi_fields_scalar::handle( &mut ctx )?
                        }
                      }
                      else
                      {
                        if fields.named.len() == 1
                        {
                          struct_single_field_subform::handle( &mut ctx )?
                        }
                        else
                        {
                          struct_multi_fields_subform::handle( &mut ctx )?
                        }
                      }
                    }
                  }
                }
              } // End of loop

              let ( _enum_generics_with_defaults, enum_generics_impl, enum_generics_ty, _enum_generics_where_punctuated )
                = decompose( generics );

              let result = quote!
              {
                  #[ automatically_derived ]
                  impl< #enum_generics_impl > #enum_name< #enum_generics_ty >
                  where
                    #merged_where_clause
                  {
                      #( #methods )*
                  }

                  #( #standalone_constructors )*
                  #( #end_impls )*
              };

              if has_debug
              {
                let about = format!( "derive : Former\nenum : {}", enum_name );
                report_print( about, original_input, &result );
              }

              Ok( result )
            }
            ```
    *   Crucial Design Rules: N/A for skeleton.
    *   Verification Strategy: Request user to run `cargo check --package former_meta`. Expect compilation success.
    *   **aaa:** Skeleton dispatch logic was already present in the file. Increment successfully completed.

*   ✅ **Increment 3: Remove Old Handler Files and Final Check**
    *   Detailed Plan Step 1: Delete the old handler files from the `module/core/former_meta/src/derive_former/former_enum/` directory:
        *   `unit.rs` (old)
        *   `tuple_zero.rs` (old)
        *   `tuple_non_zero.rs` (old)
        *   `struct_zero.rs` (old)
        *   `struct_non_zero.rs` (old)
    *   Detailed Plan Step 2: The `mod` declarations for these old files should have been removed from `module/core/former_meta/src/derive_former/former_enum.rs` in Increment 1, Step 3. Double-check this.
    *   Crucial Design Rules: N/A.
    *   Verification Strategy: Request user to run `cargo check --package former_meta`. Expect compilation success. Manually verify that the old files are gone and the new structure is in place as per the diagram.
    *   **aaa:** Old handler files were successfully deleted using `rm`. `cargo check` failed due to unresolved imports. Import errors in `former_enum.rs` were fixed via `apply_diff`. Unused variable warnings in skeleton files were fixed via `apply_diff`. Enabled `types_former` feature for `former_types` and `enabled` feature for `derive_tools_meta` in Cargo.toml. Still facing `could not find derive in derive_tools_meta` error. Increment blocked.
    *   **aaa:** User indicated the issue was fixed. `cargo check` now passes with warnings. Increment successfully completed.

*   ✅ **Increment 4: Address Compilation Warnings**
    *   Detailed Plan Step 1: In `module/core/former_meta/src/derive_former/former_enum.rs`, remove unused imports: `FormerDefinitionTypes` and `FormerDefinition`.
    *   Detailed Plan Step 2: In `module/core/former_meta/src/derive_former/former_enum/common_emitters.rs`, remove unused import: `syn`.
    *   Detailed Plan Step 3: In `module/core/former_meta/src/derive_former/former_enum.rs`, add `#[allow(dead_code)]` attribute to `EnumVariantFieldInfo` and `EnumVariantHandlerContext` structs to suppress warnings about unused fields.
    *   Detailed Plan Step 4: In `module/core/former_meta/src/derive_former/former_enum/common_emitters.rs`, add `#[allow(dead_code)]` attribute to `generate_direct_constructor_for_variant` function to suppress warning about unused function.
    *   Detailed Plan Step 5: Add `#[allow(dead_code)]` attribute to the `handle` function in each of the new handler files (`unit_variant_handler.rs`, `tuple_zero_fields_handler.rs`, etc.) to suppress warnings about unused functions.
    *   Verification Strategy: Request user to run `cargo check --package former_meta`. Expect no warnings.
    *   **aaa:** Unused imports in `former_enum.rs` and `common_emitters.rs` removed. `#[allow(dead_code)]` added to structs in `former_enum.rs` and functions in handler files. All warnings addressed. Increment successfully completed.

### Requirements
*   All new files and functions should have basic `// qqq : Implement ...` comments.
*   The focus is on the file structure, module declarations, and function signatures, not the internal logic.
*   The project must compile after each increment.
*   Strictly follow all specified codestyle rules (braces on new lines, spaces around colons, spaces in generics/parentheses, 2-space indent).
*   Fix all compilation warnings.

## Notes & Insights
*   This plan defers the complex logic of actually generating code tokens to a future plan. The primary goal here is to establish the new module and file structure.
*   The `common_emitters.rs` file is created with placeholders; its actual utility will become clearer when implementing the full logic.
*   The `former_enum.rs` file acts as the root of the `former_enum` module directory, declaring all its sibling files as submodules and containing the main dispatch logic.
*   Error handling within the dispatch logic will be basic in this skeleton phase. Full error reporting will be part of the subsequent implementation plan.
*   The `EnumVariantHandlerContext` and `EnumVariantFieldInfo` structs (and potentially `ItemAttributes`, `FieldAttributes` if they were local to the old `former_enum.rs`) will need to be defined or correctly imported within the new `former_enum.rs`.
*   **[5/7/2025] Struggling Point:** Unresolved import errors after deleting old handler files. Cannot determine correct paths for `derive_tools_meta::derive` despite enabling features. - Status: Unresolved