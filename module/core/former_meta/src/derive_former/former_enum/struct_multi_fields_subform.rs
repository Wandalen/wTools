// qqq : Implement logic for Struct { f1:T1, ... } with #[subform_scalar] or default

use super::*;
use macro_tools::{ Result, quote, syn };
use super::EnumVariantHandlerContext;
use proc_macro2::TokenStream; // Import TokenStream
use convert_case::{ Case, Casing }; // Import Case and Casing from convert_case

#[allow(dead_code)] // Suppress warning about unused function
pub( crate ) fn handle( ctx : &mut EnumVariantHandlerContext< '_ > ) -> Result< TokenStream >
{
  // This handler is specifically for Struct { f1: T1, ... } variants with #[subform_scalar] or default behavior.
  // The main dispatch should ensure this is only called for such variants.

  let variant_ident = &ctx.variant.ident;
  let enum_ident = &ctx.enum_name;
  let vis = &ctx.vis; // Get visibility

  // Generate the name for the implicit variant former
  let variant_former_name = format_ident!("{}{}Former", enum_ident, variant_ident);

  // Convert variant identifier to snake_case for the method name using convert_case
  let method_ident_string = variant_ident.to_string().to_case( Case::Snake );
  let method_ident = syn::Ident::new( &method_ident_string, variant_ident.span() ); // Create new Ident with correct span

  // Generate the static method: Enum::variant_name() -> VariantFormer<...>
  let generated_method = quote!
  {
    #[ inline( always ) ]
    pub fn #method_ident() -> #variant_former_name // Return type is the implicit variant former
    {
      #variant_former_name::default() // Assuming the implicit former has a default constructor
      // qqq : Need to handle cases where the implicit former doesn't have Default
    }
  };

  let mut generated_tokens = generated_method;

  // Generate standalone constructor if #[standalone_constructors] is present on the enum
  if ctx.struct_attrs.standalone_constructors.is_some()
  {
    let generated_standalone = quote!
    {
      #[ inline( always ) ]
      #vis fn #method_ident() -> #variant_former_name // Return type is the implicit variant former
      {
        #variant_former_name::default() // Assuming the implicit former has a default constructor
        // qqq : Need to handle cases where the implicit former doesn't have Default
      }
    };
    generated_tokens.extend(generated_standalone);
  }

  // qqq : Need to generate the implicit variant former struct and its impl block.
  // This will likely involve using common_emitters or dedicated logic here.
  // For now, just returning the method/constructor tokens.

  Ok( generated_tokens )
}