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
    return Err( syn::Error::new_spanned( ctx.variant, "#[subform_scalar] cannot be used on unit variants." ) );
  }

  let variant_ident = &ctx.variant.ident;
  let enum_ident = &ctx.enum_name;
  let vis = &ctx.vis; // Get visibility

  // Convert variant identifier to snake_case for the method name using convert_case
  let method_ident_string = variant_ident.to_string().to_case( Case::Snake );
  let method_ident = syn::Ident::new( &method_ident_string, variant_ident.span() ); // Create new Ident with correct span

  // Generate the static constructor method
  let generated_method = quote!
  {
    #[ inline( always ) ]
    pub fn #method_ident() -> #enum_ident // Added pub and return type
    {
      #enum_ident::#variant_ident
    }
  };

  // ctx.methods.push( generated_method ); // Will be collected in former_for_enum

  let mut generated_tokens = generated_method;

  // Generate standalone constructor if #[standalone_constructors] is present on the enum
  if ctx.struct_attrs.standalone_constructors.is_some()
  {
    let generated_standalone = quote!
    {
      #[ inline( always ) ]
      #vis fn #method_ident() -> #enum_ident
      {
        #enum_ident::#variant_ident
      }
    };
    // ctx.standalone_constructors.push( generated_standalone ); // Will be collected in former_for_enum
    generated_tokens.extend(generated_standalone);
  }

  Ok( generated_tokens )
}