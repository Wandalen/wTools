// File: module/core/former_meta/src/derive_former/former_enum/unit.rs

use macro_tools::{ Result, quote::quote, syn, diag };
use convert_case::{ Case, Casing };
use super::ident;
use syn::{ parse_quote }; // Keep parse_quote
use super::{ EnumVariantHandlerContext }; // Keep EnumVariantHandlerContext


// #[ allow( clippy::too_many_arguments ) ] // Allow many arguments for handler functions // qqq: Removed as arguments are consolidated
pub( super ) fn handle_unit_variant< 'a > // Added explicit lifetime 'a
(
  ctx : &mut EnumVariantHandlerContext< 'a >, // Changed signature to accept context struct
)
->
Result< () >
{
  let variant_ident = &ctx.variant.ident; // Access from context
  let variant_name_str = variant_ident.to_string();
  let method_name_snake_str = variant_name_str.to_case( Case::Snake );
  let method_name_ident_temp = parse_quote!( #method_name_snake_str );
  let method_name = ident::ident_maybe_raw( &method_name_ident_temp );

  // Check for #[subform_scalar] attribute
  if ctx.variant_attrs.subform_scalar.is_some() // Access from context
  {
    return Err( syn::Error::new_spanned( ctx.variant, "#[subform_scalar] is not allowed on unit variants" ) ); // Access from context
  }

  // Generate the static method for the unit variant
  let vis = &ctx.vis; // Create local variable for visibility
  let enum_name = &ctx.enum_name; // Create local variable for enum name
  let ( impl_generics, ty_generics, where_clause ) = ctx.generics.split_for_impl(); // Split generics for correct interpolation
  let method = quote!
  {
    #[ inline( always ) ]
    #vis fn #method_name #impl_generics #where_clause () -> #enum_name #ty_generics // Use local variables and split generics
    {
      #enum_name :: #variant_ident // Access from context
    }
  };

  ctx.methods.push( method.clone().into() ); // Add to methods for the impl block, Access from context // Added into()

  // If #[standalone_constructors] is present on the struct, add the method to standalone constructors
  if ctx.struct_attrs.standalone_constructors.is_some() // Access from context
  {
      ctx.standalone_constructors.push( method.into() ); // Access from context // Added into()
  }


  // Debug print if #[debug] is present on the enum
  if ctx.has_debug // Access from context
  {
    let about = format!( "derive : Former\nenum : {}\nvariant : {}\nhandler : unit", ctx.enum_name, variant_name_str ); // Access from context
    diag::report_print( about, ctx.original_input, ctx.methods.last().unwrap() ); // Access from context
  }

  Ok( () )
}
