//
// ## Expected Enum Former Behavior
//
// This plan adheres to the following rules for `#[derive(Former)]` on enums:
//
// 1.  **`#[scalar]` Attribute:**
//     *   **Unit Variant:** Generates `Enum::variant() -> Enum`. (Handled by: `handle_unit_variant`)
//     *   **Zero-Field Variant (Tuple):** Generates `Enum::variant() -> Enum`. (Handled by: `handle_tuple_zero_variant`)
//     *   **Zero-Field Variant (Struct):** Generates `Enum::variant() -> Enum`. (Handled by: `handle_struct_zero_variant`)
//     *   **Single-Field Variant (Tuple):** Generates `Enum::variant(InnerType) -> Enum`. (Handled by: `handle_tuple_non_zero_variant`)
//     *   **Single-Field Variant (Struct):** Generates `Enum::variant { field: InnerType } -> Enum`. (Handled by: `handle_struct_non_zero_variant`)
//     *   **Multi-Field Variant (Tuple):** Generates `Enum::variant(T1, T2, ...) -> Enum`. (Handled by: `handle_tuple_non_zero_variant`)
//     *   **Multi-Field Variant (Struct):** Generates `Enum::variant { f1: T1, f2: T2, ... } -> Enum`. (Handled by: `handle_struct_non_zero_variant`)
//     *   **Error Cases:** Cannot be combined with `#[subform_scalar]`.
//
// 2.  **`#[subform_scalar]` Attribute:**
//     *   **Unit Variant:** Error. (Checked in: `handle_unit_variant`)
//     *   **Zero-Field Variant (Tuple or Struct):** Error. (Checked in: `handle_tuple_zero_variant`, `handle_struct_zero_variant`)
//     *   **Single-Field Variant (Tuple):** Generates `Enum::variant() -> InnerFormer<...>` (where `InnerFormer` is the former for the field's type). Requires the field type to be a path type deriving `Former`. (Handled by: `handle_tuple_non_zero_variant`)
//     *   **Single-Field Variant (Struct):** Generates `Enum::variant() -> VariantFormer<...>` (an implicit former for the variant itself). (Handled by: `handle_struct_non_zero_variant`)
//     *   **Multi-Field Variant (Tuple):** Error. Cannot use `subform_scalar` on multi-field tuple variants. (Checked in: `handle_tuple_non_zero_variant`)
//     *   **Multi-Field Variant (Struct):** Generates `Enum::variant() -> VariantFormer<...>` (an implicit former for the variant itself). (Handled by: `handle_struct_non_zero_variant`)
//
// 3.  **Default Behavior (No Attribute):**
//     *   **Unit Variant:** Generates `Enum::variant() -> Enum`. (Handled by: `handle_unit_variant`)
//     *   **Zero-Field Variant (Tuple):** Generates `Enum::variant() -> Enum`. (Handled by: `handle_tuple_zero_variant`)
//     *   **Zero-Field Variant (Struct):** Error. Requires `#[scalar]`. (Checked in: `handle_struct_zero_variant`)
//     *   **Single-Field Variant (Tuple):** Generates `Enum::variant() -> InnerFormer<...>` (where `InnerFormer` is the former for the field's type). Requires the field type to be a path type deriving `Former`. (Handled by: `handle_tuple_non_zero_variant`)
//     *   **Single-Field Variant (Struct):** Generates `Enum::variant() -> VariantFormer<...>` (an implicit former for the variant itself). (Handled by: `handle_struct_non_zero_variant`)
//     *   **Multi-Field Variant (Tuple):** Generates `Enum::variant(Field1Type, Field2Type, ...) -> Enum` (behaves like `#[scalar]`). (Handled by: `handle_tuple_non_zero_variant`)
//     *   **Multi-Field Variant (Struct):** Generates `Enum::variant() -> VariantFormer<...>` (an implicit former for the variant itself). (Handled by: `handle_struct_non_zero_variant`)
//
// 4.  **`#[standalone_constructors]` Attribute (Body Level):**
//     *   Generates top-level constructor functions for each variant (e.g., `my_variant()`).
//     *   Return type depends on `#[arg_for_constructor]` on fields within the variant (see Option 2 logic in Readme/advanced.md).
//
// # Target File Structure
//
// ```
// former_enum/  (directory: module/core/former_meta/src/derive_former/former_enum/)
// ├── mod.rs                             # Main module file for `former_enum`.
// │                                      # - Declares all sibling files as submodules.
// │                                      # - Contains the primary `former_for_enum` function.
// │                                      # - Houses the main dispatch logic to route to specific handlers.
// │                                      # - Defines `EnumVariantHandlerContext` and `EnumVariantFieldInfo`.
// │
// ├── common_emitters.rs                 # Contains shared helper functions for generating common code patterns
// │                                      # used by multiple variant handlers (e.g., direct constructors,
// │                                      # boilerplate for different subformer types).
// │
// ├── unit_variant_handler.rs            # Handles `Unit` variants.
// │                                      # - `#[scalar]` or Default: Generates direct constructor.
// │                                      # - `#[subform_scalar]`: Generates an error.
// │
// ├── tuple_zero_fields_handler.rs       # Handles `Tuple()` (zero-field tuple) variants.
// │                                      # - `#[scalar]` or Default: Generates direct constructor.
// │                                      # - `#[subform_scalar]`: Generates an error.
// │
// ├── struct_zero_fields_handler.rs      # Handles `Struct {}` (zero-field struct) variants.
// │                                      # - `#[scalar]`: Generates direct constructor.
// │                                      # - `#[subform_scalar]` or Default: Generates an error.
// │
// ├── tuple_single_field_scalar.rs       # Handles `Tuple(T1)` variants with the `#[scalar]` attribute.
// │                                      # - Generates a direct constructor: `fn variant(T1) -> Enum`.
// │
// ├── tuple_single_field_subform.rs       # Handles `Tuple(T1)` variants with `#[subform_scalar]` or default behavior.
// │                                      # - Generates a method returning an inner former: `fn variant() -> InnerFormer<...>`.
// │                                      # - Requires T1 to derive Former.
// │
// ├── tuple_multi_fields_scalar.rs       # Handles `Tuple(T1, T2, ...)` (multi-field tuple) variants with
// │                                      # `#[scalar]` or default behavior.
// │                                      # - Generates a direct constructor: `fn variant(T1, T2, ...) -> Enum`.
// │                                      # - (Note: `#[subform_scalar]` is an error for multi-field tuples,
// │                                      #   handled by dispatch logic in `mod.rs`).
// │
// ├── struct_single_field_scalar.rs      # Handles `Struct { f1:T1 }` (single-field struct) variants
// │                                      # with the `#[scalar]` attribute.
// │                                      # - Generates a direct constructor: `fn variant { f1:T1 } -> Enum`.
// │
// ├── struct_single_field_subform.rs     # Handles `Struct { f1:T1 }` variants with `#[subform_scalar]`
// │                                      # or default behavior.
// │                                      # - Generates a method returning an implicit variant former:
// │                                      #   `fn variant() -> VariantFormer<...>`.
// │
// ├── struct_multi_fields_scalar.rs      # Handles `Struct { f1:T1, ... }` (multi-field struct) variants
// │                                      # with the `#[scalar]` attribute.
// │                                      # - Generates a direct constructor: `fn variant { f1:T1, ... } -> Enum`.
// │
// └── struct_multi_fields_subform.rs     # Handles `Struct { f1:T1, ... }` variants with `#[subform_scalar]`
//                                        # or default behavior.
//                                        # - Generates a method returning an implicit variant former:
//                                        #   `fn variant() -> VariantFormer<...>`.
// ```
//
#![allow(clippy::wildcard_imports)] // Keep if present

use super::*;
use macro_tools::{ Result, quote::{ format_ident, quote }, syn };
use proc_macro2::TokenStream; // Corrected import for TokenStream
use macro_tools::generic_params::decompose; // Corrected path
use super::struct_attrs::ItemAttributes; // Corrected import
use super::field_attrs::FieldAttributes; // Corrected import


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
// qqq : Define EnumVariantFieldInfo struct
#[allow(dead_code)] // Suppress warnings about unused fields
pub(super) struct EnumVariantFieldInfo
{
  pub ident : syn::Ident,
  pub ty : syn::Type,
  pub attrs : FieldAttributes,
  pub is_constructor_arg : bool,
}

// qqq : Define EnumVariantHandlerContext struct
#[allow(dead_code)] // Suppress warnings about unused fields
pub(super) struct EnumVariantHandlerContext< 'a >
{
  pub ast : &'a syn::DeriveInput,
  pub variant : &'a syn::Variant,
  pub struct_attrs : &'a ItemAttributes,
  pub enum_name : &'a syn::Ident,
  pub vis : &'a syn::Visibility,
  pub generics : &'a syn::Generics,
  pub original_input : &'a TokenStream,
  pub variant_attrs : &'a FieldAttributes,
  pub variant_field_info : &'a [EnumVariantFieldInfo],
  pub merged_where_clause : Option< &'a syn::WhereClause >,
  pub methods : &'a mut Vec< TokenStream >,
  pub end_impls : &'a mut Vec< TokenStream >,
  pub standalone_constructors : &'a mut Vec< TokenStream >,
  pub has_debug : bool,
}

#[allow(clippy::too_many_lines)]
pub(super) fn former_for_enum
(
  ast : &syn::DeriveInput,
  data_enum : &syn::DataEnum,
  original_input : &TokenStream,
  item_attributes : &ItemAttributes, // Changed: Accept parsed ItemAttributes
  has_debug : bool
) -> Result< TokenStream >
{
  let enum_name = &ast.ident;
  let vis = &ast.vis;
  let generics = &ast.generics;
  // let struct_attrs = ItemAttributes::from_attrs( ast.attrs.iter() )?; // REMOVED: Use passed item_attributes
  let struct_attrs = item_attributes; // Use the passed-in item_attributes
  // qqq : Ensure ItemAttributes and FieldAttributes are accessible/imported

  // Diagnostic print for has_debug status (has_debug is now correctly determined by the caller)
  if has_debug {
    diag::report_print("DEBUG former_for_enum: has_debug is TRUE at start (passed in).", original_input, &quote!{ struct DebugFlagWasTrue; });
  } else {
    diag::report_print("DEBUG former_for_enum: has_debug is FALSE at start (passed in).", original_input, &quote!{ struct DebugFlagWasFalse; });
  }

  let mut methods = Vec::new();
  let mut end_impls = Vec::new();
  let mut standalone_constructors = Vec::new();
  let merged_where_clause = generics.where_clause.as_ref();

  for variant in &data_enum.variants
  {
    let variant_attrs = FieldAttributes::from_attrs( variant.attrs.iter() )?;
    let variant_field_info : Vec<Result<EnumVariantFieldInfo>> = match &variant.fields {
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
        }).collect(),
        syn::Fields::Unnamed(f) => f.unnamed.iter().enumerate().map(|(index, field)| {
            let attrs = FieldAttributes::from_attrs(field.attrs.iter())?;
            let is_constructor_arg = attrs.arg_for_constructor.value(false);
            Ok(EnumVariantFieldInfo {
                ident: format_ident!("_{}", index),
                ty: field.ty.clone(),
                attrs,
                is_constructor_arg,
            })
        }).collect(),
        syn::Fields::Unit => vec![],
    };
    let variant_field_info: Vec<EnumVariantFieldInfo> = variant_field_info.into_iter().collect::<Result<_>>()?;


    let mut ctx = EnumVariantHandlerContext
    {
      ast,
      variant,
      struct_attrs,
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
      syn::Fields::Unit => {
          let generated = unit_variant_handler::handle(&mut ctx)?;
          ctx.methods.push(generated); // Collect generated tokens
      },
      syn::Fields::Unnamed( fields ) => match fields.unnamed.len()
      {
        0 => {
            let generated = tuple_zero_fields_handler::handle(&mut ctx)?;
            ctx.methods.push(generated); // Collect generated tokens
        },
        1 =>
        {
          if ctx.variant_attrs.scalar.is_some() {
              let generated = tuple_single_field_scalar::handle(&mut ctx)?;
              ctx.methods.push(generated); // Collect generated tokens
          } else {
              let generated = tuple_single_field_subform::handle(&mut ctx)?;
              ctx.methods.push(generated); // Collect generated tokens
          }
        }
        _ =>
        {
          if ctx.variant_attrs.subform_scalar.is_some()
          {
            return Err( syn::Error::new_spanned( ctx.variant, "#[subform_scalar] cannot be used on tuple variants with multiple fields." ) );
          }
          let generated = tuple_multi_fields_scalar::handle(&mut ctx)?;
          ctx.methods.push(generated); // Collect generated tokens
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
          if ctx.variant_attrs.scalar.is_none()
          {
            return Err( syn::Error::new_spanned( ctx.variant, "Zero-field struct variants require `#[scalar]` attribute for direct construction." ) );
          }
          let generated = struct_zero_fields_handler::handle(&mut ctx);
          ctx.methods.push(generated); // Collect generated tokens
        }
        _len =>
        {
          if ctx.variant_attrs.scalar.is_some()
          {
            if fields.named.len() == 1
            {
              let generated = struct_single_field_scalar::handle(&mut ctx)?;
              ctx.methods.push(generated); // Collect generated tokens
            }
            else
            {
              let generated = struct_multi_fields_scalar::handle(&mut ctx);
              ctx.methods.push(generated); // Collect generated tokens
            }
          }
          else if fields.named.len() == 1
          {
            let generated = struct_single_field_subform::handle(&mut ctx)?;
            ctx.methods.push(generated); // Collect generated tokens
          }
          else
          {
            let generated = struct_multi_fields_subform::handle(&mut ctx);
            ctx.methods.push(generated); // Collect generated tokens
          }
          
        }
      }
    } // End of match
  } // End of loop

  let ( _enum_generics_with_defaults, enum_generics_impl, enum_generics_ty, _enum_generics_where_punctuated )
    = decompose( generics );

  // qqq : Need to separate generated tokens from handlers into methods, standalone_constructors, and end_impls.
  // Currently, all are collected into methods.

  let result = quote!
  {
      #[ automatically_derived ]
      impl< #enum_generics_impl > #enum_name< #enum_generics_ty >
      where
        #merged_where_clause
      {
          #( #methods )*
      }

      // Standalone constructors and end impls should be placed here, outside the impl block.
      #( #standalone_constructors )*
      #( #end_impls )* // Uncommented to emit VariantFormer definitions
  };

  if has_debug
  {
    let about = format!( "derive : Former\nenum : {enum_name}" );
    diag::report_print( about, original_input, &result );
  }

  Ok( result )
}
