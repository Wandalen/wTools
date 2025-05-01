// File: module/core/former_meta/src/derive_former/former_enum/tuple_zero.rs

use macro_tools::{ Result, quote::quote, syn, diag };
use convert_case::{ Case, Casing };
use super::ident;
use syn::{ parse_quote };
use super::EnumVariantHandlerContext; // Import the context struct

// #[ allow( clippy::too_many_arguments ) ] // No longer needed with context struct
pub( super ) fn handle_tuple_zero_variant
(
  ctx : &mut EnumVariantHandlerContext< '_ >, // Use context struct
)
->
Result< () >
{
  let variant_ident = &ctx.variant.ident;
  let variant_name_str = variant_ident.to_string();
  let method_name_snake_str = variant_name_str.to_case( Case::Snake );
  let method_name_ident_temp = parse_quote!( #method_name_snake_str );
  let method_name = ident::ident_maybe_raw( &method_name_ident_temp );

  // Check for #[subform_scalar] attribute - not allowed on zero-field tuple variants
  if ctx.variant_attrs.subform_scalar.is_some()
  {
    return Err( syn::Error::new_spanned( ctx.variant, "#[subform_scalar] is not allowed on zero-field tuple variants" ) );
  }

  // Access necessary fields from context
  let enum_name = ctx.enum_name;
  let vis = ctx.vis;
  let generics = ctx.generics;
  let merged_where_clause = ctx.merged_where_clause;

  // Generate the static method for the zero-field tuple variant
  let method = quote!
  {
    #[ inline( always ) ]
    #vis fn #method_name #generics #merged_where_clause () -> #enum_name #generics.ty
    {
      #enum_name :: #variant_ident ()
    }
  };

  ctx.methods.push( method.clone().into() ); // Add to methods via context // Added into()

  // If #[standalone_constructors] is present on the struct, add the method to standalone constructors
  if ctx.struct_attrs.standalone_constructors.is_some()
  {
      ctx.standalone_constructors.push( method.into() ); // Add to standalone constructors via context // Added into()
  }

  // Debug print if #[debug] is present on the enum
  if ctx.has_debug
  {
    let about = format!( "derive : Former\nenum : {enum_name}\nvariant : {variant_name_str}\nhandler : tuple_zero" );
    diag::report_print( about, ctx.original_input, ctx.methods.last().unwrap() ); // Use context for original_input and methods
  }

  Ok( () )
}
