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

  // _generic_params_where_local_decomp is unused below, ctx.merged_where_clause is used instead.
  let ( _generic_params_def, generic_params_impl, generic_params_ty, _generic_params_where_local_decomp )
    = macro_tools::generic_params::decompose( &ctx.generics );

  let method_ident = {
      let name_str = variant_ident.to_string();
      if let Some(core_name) = name_str.strip_prefix("r#") {
          // Original was raw, e.g., r#fn. core_name is "fn".
          // Snake case of "fn" is still "fn".
          // We need to create a raw ident for "fn".
          let snake_core_name = core_name.to_case(Case::Snake);
          syn::Ident::new_raw(&snake_core_name, variant_ident.span())
      } else {
          // Original was not raw, e.g., MyVariant.
          // Snake case it.
          let snake_name = name_str.to_case(Case::Snake);
          // If snake_name happens to be a keyword (e.g. if variant was "Struct"), make it raw.
          // Otherwise, a normal ident.
          // A simple check: if parsing as a normal ident fails, it's likely a keyword.
          // Also handle "_" explicitly as it's a valid ident but Ident::new("_",...) might be treated specially by some linters or contexts.
          // syn::parse_str::<syn::Ident> does not consider "_" a keyword.
          // Keywords list: https://doc.rust-lang.org/reference/keywords.html
          // We need to ensure that if snake_name is a keyword, new_raw is used.
          // Otherwise, new is fine.
          let is_keyword = matches!(snake_name.as_str(),
              "as" | "async" | "await" | "break" | "const" | "continue" | "crate" | "dyn" | "else" |
              "enum" | "extern" | "false" | "fn" | "for" | "if" | "impl" | "in" | "let" | "loop" |
              "match" | "mod" | "move" | "mut" | "pub" | "ref" | "return" | "Self" | "self" |
              "static" | "struct" | "super" | "trait" | "true" | "type" | "unsafe" | "use" |
              "where" | "while" |
              // Strict keywords (cannot be used as identifiers at all, even with r#)
              // "abstract" | "become" | "box" | "do" | "final" | "macro" | "override" |
              // "priv" | "typeof" | "unsized" | "virtual" | "yield" |
              // Weak keywords (special meaning in specific contexts)
              "union" | "'static" // 'static is not an ident, union is.
              // "macro_rules" is not a keyword in ident position.
          );
          if is_keyword {
              syn::Ident::new_raw(&snake_name, variant_ident.span())
          } else {
              syn::Ident::new(&snake_name, variant_ident.span())
          }
      }
  };

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
    // Use generic_params_impl and generic_params_ty from the decomposition at lines 29-30
    // let ( _generic_params_def, generic_params_impl, generic_params_ty, _generic_params_where_local_decomp ) = ...

    let fn_signature_generics = if ctx.generics.params.is_empty() {
        quote!{}
    } else {
        quote!{ < #generic_params_impl > }
    };

    let return_type_generics = if ctx.generics.params.is_empty() {
        quote!{}
    } else {
        quote!{ < #generic_params_ty > }
    };

    let enum_path_for_construction = if ctx.generics.params.is_empty() {
        quote!{ #enum_ident }
    } else {
        // generic_params_ty is from local decomposition at lines 29-30
        if generic_params_ty.is_empty() {
             quote!{ #enum_ident }
        } else {
             quote!{ #enum_ident::< #generic_params_ty > }
        }
    };

    // Use merged_where_clause from the context, which is Option< &WhereClause >
    let where_clause_tokens = match ctx.merged_where_clause {
        Some(clause) => quote!{ #clause }, // clause is &WhereClause here
        None => quote!{},
    };

    let generated_standalone = quote!
    {
      #[ inline( always ) ]
      #vis fn #method_ident #fn_signature_generics () -> #enum_ident #return_type_generics
      #where_clause_tokens
      {
        #enum_path_for_construction :: #variant_ident
      }
    };
    ctx.standalone_constructors.push( generated_standalone );
  }

  // Debug printing removed

  Ok( generated_method )
}