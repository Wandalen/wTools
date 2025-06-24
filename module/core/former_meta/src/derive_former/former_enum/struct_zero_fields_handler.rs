use super::*;
use macro_tools::quote;
use proc_macro2::TokenStream; // Corrected import for TokenStream
// use former_types::FormerDefinition; // Not needed here

/// Handles zero-field struct variants with the `#[scalar]` attribute.
/// Returns generated tokens for the static method and optionally the standalone constructor.
pub( crate ) fn handle( ctx : &mut EnumVariantHandlerContext< '_ > ) -> TokenStream
{
  // This handler is specifically for variants with #[scalar]
  // The main dispatch should ensure this is only called for scalar zero-field struct variants.

  let enum_ident = &ctx.enum_name; // Use enum_name field
  let variant_ident = &ctx.variant.ident; // Use variant.ident field

  // Generate the static method: Enum::variant_name() -> Enum
  let static_method = quote!
  {
    #[ inline( always ) ]
    pub fn #variant_ident() -> #enum_ident
    {
      #enum_ident::#variant_ident {}
    }
  };

  let mut generated_tokens = static_method;

  // Check for #[standalone_constructors] on the enum
  // Access attributes from the enum's AST
  let has_standalone_constructors = ctx.ast.attrs.iter().any(|attr| attr.path().is_ident("standalone_constructors"));

  if has_standalone_constructors
  {
    // Generate the standalone constructor: fn variant_name() -> Enum
    let standalone_constructor = quote!
    {
      #[ inline( always ) ]
      pub fn #variant_ident() -> #enum_ident
      {
        #enum_ident::#variant_ident {}
      }
    };
    // Collect standalone constructors to be added outside the impl block
    // This requires the main derive macro to collect these tokens.
    // For now, we'll just return them as part of the handler's output.
    // The main macro will need to be updated to handle this.

    // Append standalone constructor tokens to the output
    generated_tokens.extend(standalone_constructor);

    // qqq : The main derive macro needs to collect standalone constructors
    // and place them in the correct scope (outside the enum impl block).
  }

  generated_tokens
}