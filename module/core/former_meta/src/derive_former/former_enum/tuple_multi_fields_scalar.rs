// qqq : Implement logic for Tuple(T1, T2, ...) with #[scalar] or default

use super::*;
use macro_tools::{Result, quote, syn};
use super::EnumVariantHandlerContext;
use proc_macro2::TokenStream; // Import TokenStream
use convert_case::{Case, Casing}; // Import Case and Casing from convert_case

#[allow(dead_code)] // Suppress warning about unused function
pub(crate) fn handle(ctx: &mut EnumVariantHandlerContext<'_>) -> Result<TokenStream> {
  // This handler is specifically for Tuple(T1, T2, ...) variants with #[scalar] or default behavior.
  // The main dispatch should ensure this is only called for such variants.

  // Check for #[subform_scalar] on multi-field tuple variants and return a specific error
  // This check is also in the main dispatch, but good to have here for clarity/redundancy.
  if ctx.variant_attrs.subform_scalar.is_some() {
    return Err(syn::Error::new_spanned(
      ctx.variant,
      "#[subform_scalar] cannot be used on multi-field tuple variants.",
    ));
  }

  let variant_ident = &ctx.variant.ident;
  let enum_ident = &ctx.enum_name;
  let vis = &ctx.vis; // Get visibility

  // Get field information
  let fields = &ctx.variant_field_info;

  // Generate function arguments and variant construction code
  let args = fields.iter().map(|field| {
    let field_ident = &field.ident;
    let field_ty = &field.ty;
    quote! { #field_ident : impl Into< #field_ty > }
  });

  let variant_fields = fields.iter().map(|field| {
    let field_ident = &field.ident;
    quote! { #field_ident.into() }
  });

  // Convert variant identifier to snake_case for the method name using convert_case
  let method_ident_string = variant_ident.to_string().to_case(Case::Snake);
  let method_ident = syn::Ident::new(&method_ident_string, variant_ident.span()); // Create new Ident with correct span

  // Generate the static constructor method: Enum::variant_name(args...) -> Enum
  let generated_method = quote! {
    #[ inline( always ) ]
    pub fn #method_ident( #( #args ),* ) -> #enum_ident
    {
      #enum_ident::#variant_ident( #( #variant_fields ),* )
    }
  };

  let mut generated_tokens = generated_method;

  // Generate standalone constructor if #[standalone_constructors] is present on the enum
  if ctx.struct_attrs.standalone_constructors.is_some() {
    // Need to regenerate args and variant_fields for the standalone constructor quote
    let args = fields.iter().map(|field| {
      let field_ident = &field.ident;
      let field_ty = &field.ty;
      quote! { #field_ident : impl Into< #field_ty > }
    });

    let variant_fields = fields.iter().map(|field| {
      let field_ident = &field.ident;
      quote! { #field_ident.into() }
    });

    let generated_standalone = quote! {
      #[ inline( always ) ]
      #vis fn #method_ident( #( #args ),* ) -> #enum_ident
      {
        #enum_ident::#variant_ident( #( #variant_fields ),* )
      }
    };
    generated_tokens.extend(generated_standalone);
  }

  Ok(generated_tokens)
}
