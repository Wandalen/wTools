// qqq : Implement logic for Tuple() variants

use super::*;
use macro_tools::{Result, quote, syn};
use super::EnumVariantHandlerContext;
use convert_case::{Case, Casing};
use proc_macro2::TokenStream; // Import TokenStream

#[allow(dead_code)] // Suppress warning about unused function
pub(crate) fn handle(ctx: &mut EnumVariantHandlerContext<'_>) -> Result<TokenStream> {
  // This handler is specifically for Tuple() variants.
  // The main dispatch should ensure this is only called for Tuple() variants.

  // Check for #[subform_scalar] on zero-field tuple variants and return a specific error
  if ctx.variant_attrs.subform_scalar.is_some() {
    return Err(syn::Error::new_spanned(
      ctx.variant,
      "#[subform_scalar] cannot be used on zero-field tuple variants.",
    ));
  }

  let variant_ident = &ctx.variant.ident;
  let enum_ident = &ctx.enum_name;
  let vis = &ctx.vis;

  // Decompose generics (we need impl_generics and ty_generics from this)
  let ( _def_generics, impl_generics, ty_generics, _local_where_clause_option_unused ) = // Renamed to avoid confusion
      macro_tools::generic_params::decompose(ctx.generics);

  // Use merged_where_clause from the context for the standalone constructor's where clause
  let top_level_where_clause = match ctx.merged_where_clause {
    // Use ctx.merged_where_clause
    Some(clause) => quote! { where #clause }, // clause is &WhereClause here
    None => quote! {},
  };

  // Correctly create method_ident, handling raw identifiers
  let method_ident = {
    let name_str = variant_ident.to_string();
    if let Some(core_name) = name_str.strip_prefix("r#") {
      let snake_core_name = core_name.to_case(Case::Snake);
      syn::Ident::new_raw(&snake_core_name, variant_ident.span())
    } else {
      let snake_name = name_str.to_case(Case::Snake);
      let is_keyword = matches!(
        snake_name.as_str(),
        "as"
          | "async"
          | "await"
          | "break"
          | "const"
          | "continue"
          | "crate"
          | "dyn"
          | "else"
          | "enum"
          | "extern"
          | "false"
          | "fn"
          | "for"
          | "if"
          | "impl"
          | "in"
          | "let"
          | "loop"
          | "match"
          | "mod"
          | "move"
          | "mut"
          | "pub"
          | "ref"
          | "return"
          | "Self"
          | "self"
          | "static"
          | "struct"
          | "super"
          | "trait"
          | "true"
          | "type"
          | "unsafe"
          | "use"
          | "where"
          | "while"
          | "union"
      );
      if is_keyword {
        syn::Ident::new_raw(&snake_name, variant_ident.span())
      } else {
        syn::Ident::new(&snake_name, variant_ident.span())
      }
    }
  };

  // Static method: pub fn method_name() -> Self (Self will be EnumName<ty_generics>)
  let generated_method = quote! {
    #[ inline( always ) ]
    pub fn #method_ident() -> Self
    {
      Self::#variant_ident()
    }
  };

  // Standalone constructor
  if ctx.struct_attrs.standalone_constructors.is_some() {
    let fn_signature_generics = if ctx.generics.params.is_empty() {
      quote! {}
    } else {
      quote! { < #impl_generics > }
    };
    let return_type_generics = if ctx.generics.params.is_empty() {
      quote! {}
    } else {
      quote! { < #ty_generics > }
    };

    let enum_path_for_construction = if ctx.generics.params.is_empty() || ty_generics.is_empty() {
      quote! { #enum_ident }
    } else {
      quote! { #enum_ident::< #ty_generics > }
    };

    // Create unique name for standalone constructor: [enum_name]_[variant_snake_case]
    let standalone_method_name_str = format!("{}_{}", enum_ident.to_string().to_case(Case::Snake), method_ident);
    let standalone_method_ident = syn::Ident::new(&standalone_method_name_str, variant_ident.span());

    let generated_standalone = quote! {
      #[ inline( always ) ]
      #vis fn #standalone_method_ident #fn_signature_generics () -> #enum_ident #return_type_generics
      #top_level_where_clause
      {
        #enum_path_for_construction ::#variant_ident()
      }
    };
    ctx.standalone_constructors.push(generated_standalone);
  }

  Ok(generated_method) // Return only the static method tokens
}
