// File: module/core/former_meta/src/derive_former/former_enum.rs
#![ allow( clippy::wildcard_imports ) ]

use proc_macro2::TokenStream; // Explicitly import TokenStream

// ==================================
// Refactoring Plan Documentation - UPDATED
// ==================================
//
// # Refactoring Plan for `former_for_enum`
//
// The main `former_for_enum` function has become complex due to handling
// multiple enum variant structures (Unit, Tuple, Struct) and field counts (0, 1, N)
// within nested match statements.
//
// **Goal:** Improve readability, maintainability, and testability by extracting
// the logic for handling each distinct variant case into its own dedicated function
// located in a separate file within a new submodule.
//
// **Extraction Cases & Logic Handoff:**
//
// The main `former_for_enum` function dispatches control to specific handlers based on
// the variant's field kind (`Unit`, `Unnamed`, `Named`) and field count. Each handler
// then implements the logic based on the presence of `#[scalar]` or `#[subform_scalar]`
// attributes, according to the rules defined below the documentation comment.
//
//

use super::*;

mod unit;
use unit::handle_unit_variant;

mod tuple_zero;
use tuple_zero::handle_tuple_zero_variant;

mod struct_zero;
use struct_zero::handle_struct_zero_variant;

// Add module declaration and use statement for struct_non_zero
mod struct_non_zero;
use struct_non_zero::handle_struct_non_zero_variant;

// Add module declaration and use statement for tuple_non_zero
mod tuple_non_zero;
use tuple_non_zero::handle_tuple_non_zero_variant; // FIX: Added missing use


use macro_tools::
{
  generic_params, Result,
  quote::{ format_ident, quote }, // Added ToTokens // Removed ToTokens from quote import // Added ToTokens back for derive // Removed ToTokens from quote import again
  ident, // Added for ident_maybe_raw
  // phantom, // Removed unused import
  diag, // Added for report_print
  // punctuated, // Removed unused import
  // parse_quote, // FIX: Removed unused import
};
#[ cfg( feature = "derive_former" ) ]
use convert_case::{ Case, Casing }; // Space before ;
// FIX: Removed unused imports
// use syn::punctuated::Punctuated;
// use syn::token::Comma;
// use syn::{ GenericArgument, GenericParam, TypeParam, ConstParam, LifetimeParam, /* Type, */ Expr };

// ==================================
//      Enum Variant Handling Rules (Consistent Logic) - UPDATED
// ==================================
//
// This macro implements the `Former` derive for enums based on the following consistent rules.
// Each case is handled by a specific function as noted:
//
// 1.  **`#[scalar]` Attribute:**
//     *   **Unit Variant:** Generates `Enum::variant() -> Enum`. (Handled by: `handle_unit_variant`)
//     *   **Zero-Field Variant (Tuple):** Generates `Enum::variant() -> Enum`. (Handled by: `handle_tuple_zero_variant`)
//     *   **Zero-Field Variant (Struct):** Generates `Enum::variant() -> Enum`. (Handled by: `handle_struct_zero_variant`)
//     *   **Single-Field Variant (Tuple):** Generates `Enum::variant(InnerType) -> Enum`. (Handled by: `handle_tuple_non_zero_variant`) // <<< CORRECTED Handler
//     *   **Single-Field Variant (Struct):** Generates `Enum::variant { field: InnerType } -> Enum`. (Handled by: `handle_struct_non_zero_variant`) // <<< CORRECTED Handler
//     *   **Multi-Field Variant (Tuple):** Generates `Enum::variant(T1, T2, ...) -> Enum` (Handled by: `handle_tuple_non_zero_variant`)
//     *   **Multi-Field Variant (Struct):** Generates `Enum::variant { f1: T1, f2: T2, ... } -> Enum` (Handled by: `handle_struct_non_zero_variant`)
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
//     *   **Single-Field Variant (Struct):** Generates `Enum::variant() -> VariantFormer<...>` (an implicit former for the variant itself). (Handled by: `handle_struct_non_zero_zero_variant`)
//     *   **Multi-Field Variant (Tuple):** Generates `Enum::variant(Field1Type, Field2Type, ...) -> Enum` (behaves like `#[scalar]`). (Handled by: `handle_tuple_non_zero_variant`)
//     *   **Multi-Field Variant (Struct):** Generates `Enum::variant() -> VariantFormer<...>` (an implicit former for the variant itself). (Handled by: `handle_struct_non_zero_variant`)
//
// Body attribute `standalone_constructors` creates stand-alone, top-level constructors for struct/enum. for struct it's always single function, for enum it's as many functions as enum has vartianys.
//
// ==================================


/// Temporary storage for field information needed during generation.
#[derive(Clone)] // <<< Added Clone
#[ derive( Debug ) ] // Added Debug derive

pub(super) struct EnumVariantFieldInfo
{
  // index : usize, // Removed unused field
  ident : syn::Ident,
  ty : syn::Type,
  #[allow(dead_code)] // Keep attrs field even if unused for now
  attrs : FieldAttributes,
  is_constructor_arg : bool,
}


/// Context struct holding all necessary information for enum variant handlers.
///
/// This struct consolidates the various pieces of data and output collectors
/// required by the handler functions (`handle_*_variant`), simplifying their
/// signatures and making context passing more manageable.
#[ derive( Debug ) ] // Added Debug derive for potential debugging // Added ToTokens derive // Use direct ToTokens // Use quot...
pub(super) struct EnumVariantHandlerContext< 'a > // Use pub(super) as it's used within the derive_former module
{
  /// Reference to the original derive input AST.
  #[allow(dead_code)] // Field is not currently read by handlers, but may be useful later.
  pub ast : &'a syn::DeriveInput,
  /// Reference to the specific variant being processed.
  pub variant : &'a syn::Variant,
  /// Parsed attributes from the enum struct itself.
  pub struct_attrs : &'a ItemAttributes,
  /// Identifier of the enum.
  pub enum_name : &'a syn::Ident,
  /// Visibility of the enum.
  pub vis : &'a syn::Visibility,
  /// Generics of the enum.
  pub generics : &'a syn::Generics,
  /// Reference to the original `proc_macro` `TokenStream` input.
  pub original_input : &'a TokenStream, // Change type back to proc_macro::TokenStream // Corrected type to proc_macro2::TokenStream
  /// Parsed attributes from the specific variant being processed.
  pub variant_attrs : &'a FieldAttributes,
  /// Collected information about the fields within the current variant.
  pub variant_field_info : &'a [ EnumVariantFieldInfo ], // Use slice for borrowed Vec data
  /// The merged where clause for the enum's impl block.
  pub merged_where_clause : Option< &'a syn::WhereClause >,

  // Output Collectors
  /// Mutable reference to collect generated method `TokenStreams`.
  pub methods : &'a mut Vec< TokenStream >,
  /// Mutable reference to collect generated `end_impl` `TokenStreams` (e.g., implicit formers).
  pub end_impls : &'a mut Vec< TokenStream >,
  /// Mutable reference to collect generated standalone constructor `TokenStreams`.
  pub standalone_constructors : &'a mut Vec< TokenStream >,

  // Flags
  /// Flag indicating if the `#[debug]` attribute was present.
  pub has_debug : bool,
}

/// Generate the Former ecosystem for an enum.
#[ allow( clippy::too_many_lines ) ]
pub(super) fn former_for_enum
(
  ast : &syn::DeriveInput,
  data_enum : &syn::DataEnum,
  original_input : &TokenStream, // Change type to proc_macro2::TokenStream
  has_debug : bool, // Added has_debug
) -> Result< TokenStream > // Change return type to proc_macro2::TokenStream
{
  let enum_name = &ast.ident;
  let vis = &ast.vis;
  let generics = &ast.generics;
  let ( _enum_generics_with_defaults, enum_generics_impl, enum_generics_ty, _enum_generics_where_punctuated ) // Use _ for unused where punctuated
  = generic_params::decompose( generics );
  // Use the Option<&WhereClause> directly from generics by calling .as_ref()
  let merged_where_clause = generics.where_clause.as_ref(); // FIX: Use .as_ref() here

  // --- DEBUG PRINT 1 ---
  // ...
  // --- END DEBUG PRINT 1 ---


  // Parse struct-level attributes
  let struct_attrs = ItemAttributes::from_attrs( ast.attrs.iter() )?;

  // Initialize vectors to collect generated code pieces
  let mut methods = Vec::new();
  let mut end_impls = Vec::new();
  let mut standalone_constructors = Vec::new(); // <<< Vector to store standalone constructors

  // Iterate through each variant of the enum
  for variant in &data_enum.variants
  {
    let variant_ident = &variant.ident; // Prefixed with _

    // --- DEBUG PRINT 2 ---
    // ...
    // --- END DEBUG PRINT 2 ---


    // Generate the snake_case method name, handling potential keywords
    let variant_name_str = variant_ident.to_string(); // Prefixed with _
    let method_name_snake_str = variant_name_str.to_case( Case::Snake ); // Prefixed with _
    let method_name_ident_temp = format_ident!( "{}", method_name_snake_str, span = variant_ident.span() ); // Prefixed with _
    let method_name = ident::ident_maybe_raw( &method_name_ident_temp ); // Prefixed with _

    // Parse attributes *from the variant* itself
    let variant_attrs = FieldAttributes::from_attrs( variant.attrs.iter() )?;
    let _wants_scalar = variant_attrs.scalar.is_some() && variant_attrs.scalar.as_ref().unwrap().setter(); // Renamed from _wants_scalar // Prefixed with _
    let _wants_subform_scalar = variant_attrs.subform_scalar.is_some(); // Renamed from _wants_subform_scalar // Prefixed with _

    // --- Prepare merged where clause for this variant's generated impls ---
    // let merged_where_clause = enum_generics_where.clone(); // Clone the Option<&WhereClause> // Removed redundant clone

    // <<< Added: Collect detailed field info for the current variant >>>
    let variant_field_info: Vec<EnumVariantFieldInfo> = match &variant.fields {
        syn::Fields::Named(f) => f.named.iter().map(|field| { // <<< Use _index
            let attrs = FieldAttributes::from_attrs(field.attrs.iter())?;
            let is_constructor_arg = attrs.arg_for_constructor.value(false);
            Ok(EnumVariantFieldInfo {
                // index, // Removed assignment to unused field
                ident: field.ident.clone().ok_or_else(|| syn::Error::new_spanned(field, "Named field requires an identifier"))?,
                ty: field.ty.clone(),
                attrs, // Store parsed field attributes
                is_constructor_arg,
            })
        }).collect::<Result<_>>()?,
        syn::Fields::Unnamed(f) => f.unnamed.iter().enumerate().map(|(index, field)| {
            let attrs = FieldAttributes::from_attrs(field.attrs.iter())?;
            let is_constructor_arg = attrs.arg_for_constructor.value(false);
            Ok(EnumVariantFieldInfo {
                // index, // Removed assignment to unused field
                ident: format_ident!("_{}", index), // Synthesize identifier - Note: still uses index here!
                ty: field.ty.clone(),
                attrs, // Store parsed field attributes
                is_constructor_arg,
            })
        }).collect::<Result<_>>()?,
        syn::Fields::Unit => vec![],
    };
    // <<< End Added >>>


    let mut ctx = EnumVariantHandlerContext
    {
      ast,
      variant,
      struct_attrs : &struct_attrs,
      enum_name,
      vis,
      generics,
      original_input, // Pass original_input directly (now correct type)
      variant_attrs : &variant_attrs,
      variant_field_info : &variant_field_info,
      merged_where_clause,
      methods : &mut methods,
      end_impls : &mut end_impls,
      standalone_constructors : &mut standalone_constructors,
      has_debug,
    };


    // Generate method based on the variant's fields
    match &variant.fields
    {
      // Case 1: Unit variant
      syn::Fields::Unit =>
      {
        handle_unit_variant( &mut ctx )?;
      },
      // Case 2: Tuple variant
      syn::Fields::Unnamed( fields ) =>
      {
        if variant_attrs.arg_for_constructor.value( false )
        {
          return Err( syn::Error::new_spanned( variant, "#[arg_for_constructor] cannot be applied directly to an enum variant identifier. Apply it to the fields *within* the variant instead, e.g., `MyVariant( #[arg_for_constructor] i32 )`." ) );
        }

        match fields.unnamed.len()
        {
          // Sub-case: Zero fields (treat like Unit variant)
          0 =>
          {
            handle_tuple_zero_variant( &mut ctx )?;
          }
          // Sub-case: Non-zero fields (Tuple(1) or Tuple(N))
          _ => // len >= 1
          {
            handle_tuple_non_zero_variant( &mut ctx )?;
          }
        }
      },
      // Case 3: Struct variant
      syn::Fields::Named( fields ) =>
      {
        if variant_attrs.arg_for_constructor.value( false )
        {
          return Err( syn::Error::new_spanned( variant, "#[arg_for_constructor] cannot be applied directly to an enum variant identifier. Apply it to the fields *within* the variant instead, e.g., `MyVariant { #[arg_for_constructor] field : i32 }`." ) );
        }

        match fields.named.len()
        {
            // Sub-case: Zero fields (Struct(0))
            0 =>
            {
                handle_struct_zero_variant( &mut ctx )?;
            }
            // Sub-case: Single field (Struct(1)) or Multi-field (Struct(N))
            _ => // len >= 1
            {
              handle_struct_non_zero_variant( &mut ctx )?;
            }
        }
      }
    }
  }

  // Assemble the final impl block containing the generated static methods and standalone constructors
  let methods_and_constructors_impl : TokenStream = quote!
  {
      #[ automatically_derived ]
      impl< #enum_generics_impl > #enum_name< #enum_generics_ty >
      where // Where clause on new line
        #merged_where_clause // FIX: Use the Option<&WhereClause> variable here
      { // Brace on new line
          #( #methods )* // Splice the collected methods here
          #( #standalone_constructors )* // Splice standalone constructors here
      } // Brace on new line
  }; // Remove into()

  // Assemble the end_impls (End structs, implicit formers, etc.)
  let end_impls_tokens : TokenStream = quote!
  {
      #( #end_impls )* // Splice the collected end_impls here
  }; // Remove into()

  // Combine the generated code pieces
  let result = quote!
  {
      #methods_and_constructors_impl
      #end_impls_tokens
  };

  if has_debug // Print generated code if #[debug] is present on the enum
  {
    let about = format!( "derive : Former\nenum : {enum_name}" );
    diag::report_print( about, original_input, &result );
  }

  Ok( result ) // Return proc_macro2::TokenStream directly
}
