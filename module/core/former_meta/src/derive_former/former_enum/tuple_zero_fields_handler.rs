// qqq : Implement logic for Tuple() variants

use super::*;
use macro_tools::{ Result, quote, syn };
use super::EnumVariantHandlerContext;
use convert_case::{ Case, Casing };
use proc_macro2::TokenStream; // Import TokenStream

#[allow(dead_code)] // Suppress warning about unused function
pub( crate ) fn handle( ctx : &mut EnumVariantHandlerContext< '_ > ) -> Result< TokenStream >
{
  // This handler is specifically for Tuple() variants.
  // The main dispatch should ensure this is only called for Tuple() variants.

  // Check for #[subform_scalar] on zero-field tuple variants and return a specific error
  if ctx.variant_attrs.subform_scalar.is_some()
  {
    return Err( syn::Error::new_spanned( ctx.variant, "#[subform_scalar] cannot be used on zero-field tuple variants." ) );
  }

  let variant_ident = &ctx.variant.ident;
  let enum_ident = &ctx.enum_name;
  let vis = &ctx.vis; // Get visibility

  // Convert variant identifier to snake_case for the method name using convert_case
  let method_ident_string = variant_ident.to_string().to_case( Case::Snake );
  let method_ident = syn::Ident::new( &method_ident_string, variant_ident.span() ); // Create new Ident with correct span

  // Generate the static constructor method: Enum::variant_name() -> Enum
  // This applies for both #[scalar] and default behavior on zero-field tuple variants.
  let generated_method = quote!
  {
    #[ inline( always ) ]
    pub fn #method_ident() -> #enum_ident
    {
      #enum_ident::#variant_ident()
    }
  };

  let mut generated_tokens = generated_method;

  // Generate standalone constructor if #[standalone_constructors] is present on the enum
  if ctx.struct_attrs.standalone_constructors.is_some()
  {
    let generated_standalone = quote!
    {
      #[ inline( always ) ]
      #vis fn #method_ident() -> #enum_ident
      {
        #enum_ident::#variant_ident()
      }
    };
    generated_tokens.extend(generated_standalone);
  }

  Ok( generated_tokens )
}