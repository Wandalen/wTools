// qqq : Implement logic for Struct { f1:T1 } with #[scalar]

use super::*;
use macro_tools::{Result, quote, syn};
use super::EnumVariantHandlerContext;
use proc_macro2::TokenStream; // Import TokenStream
use convert_case::{Case, Casing}; // Import Case and Casing from convert_case

#[allow(dead_code)] // Suppress warning about unused function
pub(crate) fn handle(ctx: &mut EnumVariantHandlerContext<'_>) -> Result<TokenStream> {
  // This handler is specifically for Struct { f1: T1 } variants with #[scalar].
  // The main dispatch should ensure this is only called for such variants.

  let variant_ident = &ctx.variant.ident;
  let enum_ident = &ctx.enum_name;
  let vis = &ctx.vis; // Get visibility

  // Get the single field's type and identifier
  let field = ctx
    .variant_field_info
    .first()
    .ok_or_else(|| syn::Error::new_spanned(ctx.variant, "Struct variant with #[scalar] must have exactly one field."))?;
  let field_ident = &field.ident;
  let field_ty = &field.ty;

  // Convert variant identifier to snake_case for the method name using convert_case
  let method_ident_string = variant_ident.to_string().to_case(Case::Snake);
  let method_ident = syn::Ident::new(&method_ident_string, variant_ident.span()); // Create new Ident with correct span

  // Generate the static constructor method: Enum::variant_name { field_name: FieldType } -> Enum
  let generated_method = quote! {
    #[ inline( always ) ]
    pub fn #method_ident( #field_ident : impl Into< #field_ty > ) -> #enum_ident
    {
      #enum_ident::#variant_ident { #field_ident: #field_ident.into() }
    }
  };

  let mut generated_tokens = generated_method;

  // Generate standalone constructor if #[standalone_constructors] is present on the enum
  if ctx.struct_attrs.standalone_constructors.is_some() {
    let generated_standalone = quote! {
      #[ inline( always ) ]
      #vis fn #method_ident( #field_ident : impl Into< #field_ty > ) -> #enum_ident
      {
        #enum_ident::#variant_ident { #field_ident: #field_ident.into() }
      }
    };
    generated_tokens.extend(generated_standalone);
  }

  Ok(generated_tokens)
}
