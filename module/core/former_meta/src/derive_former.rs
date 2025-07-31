// File: module/core/former_meta/src/derive_former.rs

use super::*;
use macro_tools::{
  diag, typ, Result,
  proc_macro2::TokenStream,
  quote::{format_ident, quote, ToTokens},
  syn::spanned::Spanned,
};

mod former_enum;
use former_enum::former_for_enum;
mod former_struct;
use former_struct::former_for_struct;

mod field_attrs;

use field_attrs::*;
mod field;

use field::*;
mod struct_attrs;

use struct_attrs::*;

/// Represents the generic parameters for a `FormerDefinitionTypes`.
pub struct FormerDefinitionTypesGenerics<'a> {
  pub impl_generics: &'a syn::punctuated::Punctuated<syn::GenericParam, syn::token::Comma>,
  pub ty_generics: &'a syn::punctuated::Punctuated<syn::GenericParam, syn::token::Comma>,
  pub where_clause: &'a syn::punctuated::Punctuated<syn::WherePredicate, syn::token::Comma>,
}

impl ToTokens for FormerDefinitionTypesGenerics<'_> {
  fn to_tokens(&self, tokens: &mut TokenStream) {
    self.impl_generics.to_tokens(tokens);
    self.ty_generics.to_tokens(tokens);
    self.where_clause.to_tokens(tokens);
  }
}

/// Generates the code for implementing the `FormerMutator` trait for a specified former definition type.
/// If the `custom` attribute is not specified, a default empty implementation is generated.
/// If the `debug` attribute is specified, it prints an example of a custom mutator implementation.
#[allow(clippy::format_in_format_args, clippy::unnecessary_wraps)]
pub fn mutator(
  _item: &syn::Ident,
  _original_input: &macro_tools::proc_macro2::TokenStream,
  mutator: &AttributeMutator,
  _former_definition_types: &syn::Ident,
  generics: &FormerDefinitionTypesGenerics<'_>,
  former_definition_types_ref: &proc_macro2::TokenStream,
) -> Result<TokenStream> {
  let impl_generics = generics.impl_generics;
  let _ty_generics = generics.ty_generics;
  let where_clause = generics.where_clause;
  
  let former_mutator_code = if mutator.custom.value(false) {
    // If custom mutator is requested via #[ mutator( custom ) ], generate nothing, assuming user provides the impl.
    quote! {}
  } else {
    // Otherwise, generate a default empty impl.
    quote! {
      impl< #impl_generics > former::FormerMutator
      for #former_definition_types_ref
      where
        #where_clause
      {
      }
    }
  };

  // If debug is enabled for the mutator attribute, print a helpful example,
  // but only if the `former_diagnostics_print_generated` feature is enabled.
  if mutator.debug.value(false) {
    #[cfg(feature = "former_diagnostics_print_generated")]
    {
      let debug = format!(
        r"
 = Example of custom mutator

 impl< {} > former::FormerMutator
 for {former_definition_types} < {} >
 where
   {}
 {{
   /// Mutates the context and storage of the entity just before the formation process completes.
   #[ inline ]
   fn form_mutation
   (
     storage : &mut Self::Storage,
     context : &mut Option< Self::Context >,
   )
   {{
     // Example: Set a default value if field 'a' wasn't provided
     // storage.a.get_or_insert_with( Default::default );
   }}
 }}
       ",
        format!("{}", quote! { #impl_generics }),
        format!("{}", quote! { #ty_generics }),
        format!("{}", quote! { #where_clause }),
      );
      let about = format!(
        r"derive : Former
    item : {item}",
      );
      diag::report_print(about, original_input, debug);
    }
  }

  Ok(former_mutator_code)
}

/// Generate documentation strings for the former struct and its module.
fn doc_generate(item: &syn::Ident) -> (String, String) {
  let doc_former_mod = format!(
    r" Implementation of former for [{item}].
"
  );

  let doc_former_struct = format!(
    r"
Structure to form [{item}]. Represents a forming entity designed to construct objects through a builder pattern.

This structure holds temporary storage and context during the formation process and
utilizes a defined end strategy to finalize the object creation.
"
  );

  (doc_former_mod, doc_former_struct)
}

/// Generate the whole Former ecosystem for either a struct or an enum.
/// This is the main entry point for the `#[derive(Former)]` macro.
#[allow(clippy::too_many_lines)]
pub fn former(input: proc_macro::TokenStream) -> Result<TokenStream> {
  let original_input: TokenStream = input.clone().into();
  let ast = syn::parse::<syn::DeriveInput>(input)?;

  // Parse ItemAttributes ONCE here from all attributes on the item
  let item_attributes = struct_attrs::ItemAttributes::from_attrs(ast.attrs.iter())?;
  // Determine has_debug based on the parsed item_attributes
  let has_debug = item_attributes.debug.is_some();

  // Dispatch based on whether the input is a struct, enum, or union.
  let result = match ast.data {
    syn::Data::Struct(ref data_struct) => {
      // Pass the parsed item_attributes and the correctly determined has_debug
      former_for_struct(&ast, data_struct, &original_input, &item_attributes, has_debug)
    }
    syn::Data::Enum(ref data_enum) => {
      // Pass the parsed item_attributes and the correctly determined has_debug
      former_for_enum(&ast, data_enum, &original_input, &item_attributes, has_debug)
    }
    syn::Data::Union(_) => {
      // Unions are not supported.
      Err(syn::Error::new(ast.span(), "Former derive does not support unions"))
    }
  }?;

  // Write generated code to file for debugging if needed
  #[cfg(debug_assertions)]
  std::fs::write("/tmp/generated_former_code.rs", result.to_string()).ok();

  // If the top-level `#[debug]` attribute was found, print the final generated code,
  // but only if the `former_diagnostics_print_generated` feature is enabled.
  if has_debug {
    #[cfg(feature = "former_diagnostics_print_generated")]
    {
      let about = format!("derive : Former\nstructure : {}", ast.ident);
      diag::report_print(about, &original_input, &result);
    }
  }

  Ok(result)
}
