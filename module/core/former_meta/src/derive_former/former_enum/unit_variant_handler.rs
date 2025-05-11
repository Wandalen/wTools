// qqq : Implement logic for Unit variants

use super::*;
use macro_tools::{ Result, quote, syn };
use super::EnumVariantHandlerContext;
// use heck::ToSnakeCase; // Removed heck
use convert_case::{ Case, Casing }; // Import Case and Casing from convert_case
use proc_macro2::TokenStream; // Import TokenStream

#[allow(dead_code)] // Suppress warning about unused function
pub( crate ) fn handle( ctx : &mut EnumVariantHandlerContext< '_ > ) -> Result< TokenStream >
{
  // qqq : Implement skeleton body

  // Check for #[subform_scalar] on unit variants and return a specific error
  if ctx.variant_attrs.subform_scalar.is_some()
  {
    // Directly return a TokenStream containing compile_error!
    let error_message = "TEST ERROR: #[subform_scalar] cannot be used on unit variants. V3";
    return Ok(quote_spanned! { ctx.variant.span() =>
      compile_error!(#error_message);
    });
  }

  let variant_ident = &ctx.variant.ident;
  let enum_ident = &ctx.enum_name;
  let vis = &ctx.vis;

  let ( _generic_params_def, generic_params_impl, generic_params_ty, generic_params_where )
    = macro_tools::generic_params::decompose( &ctx.generics );

  let mut base_method_name = variant_ident.to_string();
  if base_method_name.starts_with("r#") {
    base_method_name = base_method_name[2..].to_string();
  }
  let method_ident_string = base_method_name.to_case( Case::Snake );
  let method_ident = syn::Ident::new( &method_ident_string, variant_ident.span() );

  // Generate the static constructor method
  let generated_method = quote!
  {
    #[ inline( always ) ]
    pub fn #method_ident() -> Self // Use Self
    {
      Self::#variant_ident // Use Self
    }
  };

  // Generate standalone constructor if #[standalone_constructors] is present on the enum
  if ctx.struct_attrs.standalone_constructors.is_some()
  {
    let generated_standalone = quote!
    {
      #[ inline( always ) ]
      #vis fn #method_ident< #generic_params_impl >() -> #enum_ident< #generic_params_ty > // Add generics
      where #generic_params_where // Add where clause
      {
        #enum_ident::< #generic_params_ty >::#variant_ident // Use turbofish
      }
    };
    ctx.standalone_constructors.push( generated_standalone );
  }

  Ok( generated_method )
}