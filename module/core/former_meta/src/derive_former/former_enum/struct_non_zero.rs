// File: module/core/former_meta/src/derive_former/former_enum/struct_non_zero.rs
use super::*; // Use items from parent module (former_enum)

use macro_tools::
{
  generic_params, Result,
  quote::{ format_ident },
  ident,
};
use syn::
{
  self,
  Fields,
  GenericParam,
  punctuated::Punctuated,
  token::Comma,
};
use convert_case::{ Case, Casing };

/// Handles the generation of code for struct variants with non-zero fields.
#[ allow( clippy::too_many_lines ) ] // Keep this one for now
pub( super ) fn handle_struct_non_zero_variant
(
  ctx : &mut EnumVariantHandlerContext< '_ >,
) -> Result< () >
{
  // Extract necessary fields from context into local variables
  let variant = &ctx.variant;
  let variant_ident = &variant.ident;
  let variant_attrs = &ctx.variant_attrs;
  let _struct_attrs = &ctx.struct_attrs;
  let generics = &ctx.generics;
  let variant_field_info = &ctx.variant_field_info;
  let _vis = &ctx.vis;
  let _enum_name = &ctx.enum_name;

  // Define field_types here to make it available in multiple scopes
  let _field_types : Vec<syn::Type> = variant_field_info.iter().map( |f_info| f_info.ty.clone() ).collect(); // Collect owned types

  // Generate the snake_case method name, handling potential keywords
  let variant_name_str = variant_ident.to_string();
  let method_name_snake_str = variant_name_str.to_case( Case::Snake );
  // Use format_ident! instead of parse_quote! for robust identifier creation
  let method_name_ident_temp = format_ident!( "{}", method_name_snake_str, span = variant_ident.span() );
  let _method_name = ident::ident_maybe_raw( &method_name_ident_temp );

  let ( _enum_generics_with_defaults, _enum_generics_impl, enum_generics_ty_with_comma, _enum_generics_where_punctuated )
  = generic_params::decompose( generics );
  // Use the Option<&WhereClause> directly from generics by calling .as_ref()
  let _enum_generics_where_clause = ctx.merged_where_clause; // Renamed for clarity, prefixed with _

  // Create a version of enum_generics_ty *without* the trailing comma for use in type names
  let _enum_generics_ty_no_comma : Punctuated<GenericParam, Comma> = enum_generics_ty_with_comma.into_iter().collect(); // Use into_iter().collect()

  // Check if the attribute is present using .is_some()
  let wants_subform_scalar = variant_attrs.subform_scalar.is_some();
  let wants_scalar = variant_attrs.scalar.is_some();

  // Helper for conditional comma - Removed, logic embedded below

  match &variant.fields
  {
    Fields::Named( _fields ) =>
    {
      if wants_subform_scalar
      {
      }
      else if wants_scalar
      {
      }
    }
    Fields::Unnamed( _fields ) =>
    {
      if wants_subform_scalar
      {
      }
      else if wants_scalar
      {
      }
    }
    Fields::Unit =>
    {
      if wants_subform_scalar
      {
      }
      else if wants_scalar
      {
      }
    }
  }

  Ok( () )
}
