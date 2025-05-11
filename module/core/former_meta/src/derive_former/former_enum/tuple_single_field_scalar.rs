// qqq : Implement logic for Tuple(T1) with #[scalar]
// qqq : Call common_emitters::generate_direct_constructor_for_variant(...)

use super::*;
use macro_tools::{ Result, quote, syn };
use super::EnumVariantHandlerContext;
use proc_macro2::TokenStream; // Import TokenStream
use convert_case::{ Case, Casing }; // Import Case and Casing from convert_case

#[allow(dead_code)] // Suppress warning about unused function
pub( crate ) fn handle( ctx : &mut EnumVariantHandlerContext< '_ > ) -> Result< TokenStream >
{
  // This handler is specifically for Tuple(T1) variants with #[scalar].
  // The main dispatch should ensure this is only called for such variants.

  let variant_ident = &ctx.variant.ident;
  let enum_ident = &ctx.enum_name;
  let vis = &ctx.vis;

  // Decompose generics for use in signatures (impl_generics and ty_generics are needed)
  let ( _def_generics, impl_generics, ty_generics, _local_where_clause_option ) =
      macro_tools::generic_params::decompose(&ctx.generics);

  // Use merged_where_clause from the context for the standalone constructor's where clause
  let where_clause = match ctx.merged_where_clause {
      Some(clause) => quote! { #clause }, // clause is &WhereClause here
      None => quote! {},
  };

  // Get the single field's type and identifier
  let field = ctx.variant_field_info.get(0).ok_or_else(|| {
      syn::Error::new_spanned(ctx.variant, "Tuple variant with #[scalar] must have exactly one field.")
  })?;
  let field_ty = &field.ty;
  let field_ident = &field.ident; // Use the generated identifier like _0

  // Correctly create method_ident, handling raw identifiers
  let method_ident = {
      let name_str = variant_ident.to_string();
      if let Some(core_name) = name_str.strip_prefix("r#") {
          let snake_core_name = core_name.to_case(Case::Snake);
          syn::Ident::new_raw(&snake_core_name, variant_ident.span())
      } else {
          let snake_name = name_str.to_case(Case::Snake);
          let is_keyword = matches!(snake_name.as_str(), "as" | "async" | "await" | "break" | "const" | "continue" | "crate" | "dyn" | "else" | "enum" | "extern" | "false" | "fn" | "for" | "if" | "impl" | "in" | "let" | "loop" | "match" | "mod" | "move" | "mut" | "pub" | "ref" | "return" | "Self" | "self" | "static" | "struct" | "super" | "trait" | "true" | "type" | "unsafe" | "use" | "where" | "while" | "union" );
          if is_keyword {
              syn::Ident::new_raw(&snake_name, variant_ident.span())
          } else {
              syn::Ident::new(&snake_name, variant_ident.span())
          }
      }
  };

  // Static method: pub fn method_name(field: impl Into<FieldTy>) -> Self
  // `Self` correctly refers to `EnumName<ty_generics>` within the impl block
  let generated_method = quote!
  {
    #[ inline( always ) ]
    pub fn #method_ident( #field_ident : impl Into< #field_ty > ) -> Self
    {
      Self::#variant_ident( #field_ident.into() )
    }
  };

  // Standalone constructor
  if ctx.struct_attrs.standalone_constructors.is_some()
  {
    let fn_signature_generics = if ctx.generics.params.is_empty() { quote!{} } else { quote!{ < #impl_generics > } };
    let return_type_generics = if ctx.generics.params.is_empty() { quote!{} } else { quote!{ < #ty_generics > } };
    // enum_path_for_construction is not strictly needed here as we use #enum_ident #return_type_generics for return
    // and #enum_ident::#variant_ident for construction path (generics inferred or explicit on #enum_ident if needed by context)

    let generated_standalone = quote!
    {
      #[ inline( always ) ]
      #vis fn #method_ident #fn_signature_generics ( #field_ident : impl Into< #field_ty > ) -> #enum_ident #return_type_generics
      #where_clause
      {
        #enum_ident::#variant_ident( #field_ident.into() ) // Generics for #enum_ident will be inferred by return type or must be specified if ambiguous
      }
    };
    // Instead of generated_tokens.extend(), push to ctx.standalone_constructors
    ctx.standalone_constructors.push(generated_standalone);
  }

  // This handler only returns the static method. Standalone constructors are collected in ctx.
  // let mut generated_tokens = generated_method; // Not needed anymore

  // qqq : Consider using common_emitters::generate_direct_constructor_for_variant
  // This handler's logic is simple enough that direct generation is fine for now.
  // If more complex direct constructors are needed, refactor into common_emitters.

  Ok( generated_method ) // Return only the static method tokens
}