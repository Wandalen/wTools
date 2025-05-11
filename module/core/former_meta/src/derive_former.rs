// File: module/core/former_meta/src/derive_former.rs
#[ allow( clippy::wildcard_imports ) ]
use super::*;
use macro_tools::
{
  attr, diag, typ, Result,
  proc_macro2::TokenStream, quote::{ format_ident, quote }, syn::spanned::Spanned,
};

mod former_enum;
use former_enum::former_for_enum;
mod former_struct;
use former_struct::former_for_struct;

mod field_attrs;
#[ allow( clippy::wildcard_imports ) ]
use field_attrs::*;
mod field;
#[ allow( clippy::wildcard_imports ) ]
use field::*;
mod struct_attrs;
#[ allow( clippy::wildcard_imports ) ]
use struct_attrs::*;

/// Generates the code for implementing the `FormerMutator` trait for a specified former definition type.
/// If the `custom` attribute is not specified, a default empty implementation is generated.
/// If the `debug` attribute is specified, it prints an example of a custom mutator implementation.
#[ allow( clippy::format_in_format_args, clippy::unnecessary_wraps ) ]
pub fn mutator
(
  item : &syn::Ident,
  original_input : &macro_tools::proc_macro2::TokenStream,
  mutator : &AttributeMutator,
  former_definition_types : &syn::Ident,
  former_definition_types_generics_impl : &syn::punctuated::Punctuated< syn::GenericParam, syn::token::Comma >,
  former_definition_types_generics_ty : &syn::punctuated::Punctuated< syn::GenericParam, syn::token::Comma >,
  former_definition_types_generics_where : &syn::punctuated::Punctuated< syn::WherePredicate, syn::token::Comma >,
)
-> Result< TokenStream >
{
  let former_mutator_code = if mutator.custom.value( false )
  {
    // If custom mutator is requested via #[ mutator( custom ) ], generate nothing, assuming user provides the impl.
    quote!{}
  }
  else
  {
    // Otherwise, generate a default empty impl.
    quote!
    {
      impl< #former_definition_types_generics_impl > former::FormerMutator
      for #former_definition_types < #former_definition_types_generics_ty >
      where
        #former_definition_types_generics_where
      {
      }
    }
  };

  // If debug is enabled for the mutator attribute, print a helpful example.
  if mutator.debug.value( false )
  {
    let debug = format!
    (
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
      format!( "{}", quote!{ #former_definition_types_generics_impl } ),
      format!( "{}", quote!{ #former_definition_types_generics_ty } ),
      format!( "{}", quote!{ #former_definition_types_generics_where } ),
    );
    let about = format!
    (
r"derive : Former
item : {item}",
    );
    diag::report_print( about, original_input, debug );
  }

  Ok( former_mutator_code )
}


/// Generate documentation strings for the former struct and its module.
fn doc_generate( item : &syn::Ident ) -> ( String, String )
{

  let doc_former_mod = format!
  (
r" Implementation of former for [{item}].
"
  );

  let doc_former_struct = format!
  (
r"
Structure to form [{item}]. Represents a forming entity designed to construct objects through a builder pattern.

This structure holds temporary storage and context during the formation process and
utilizes a defined end strategy to finalize the object creation.
"
  );

  ( doc_former_mod, doc_former_struct )
}


/// Generate the whole Former ecosystem for either a struct or an enum.
/// This is the main entry point for the `#[derive(Former)]` macro.
#[ allow( clippy::too_many_lines ) ]
pub fn former( input : proc_macro::TokenStream ) -> Result< TokenStream >
{
  let original_input : TokenStream = input.clone().into();
  let ast = syn::parse::< syn::DeriveInput >( input )?;

  // Parse ItemAttributes ONCE here from all attributes on the item
  let item_attributes = struct_attrs::ItemAttributes::from_attrs( ast.attrs.iter() )?;
  // Determine has_debug based on the parsed item_attributes
  let has_debug = item_attributes.debug.is_some();

  // Dispatch based on whether the input is a struct, enum, or union.
  let result = match ast.data
  {
      syn::Data::Struct( ref data_struct ) =>
      {
          // Pass the parsed item_attributes and the correctly determined has_debug
          former_for_struct( &ast, data_struct, &original_input, &item_attributes, has_debug )
      },
      syn::Data::Enum( ref data_enum ) =>
      {
          // Pass the parsed item_attributes and the correctly determined has_debug
          former_for_enum( &ast, data_enum, &original_input, &item_attributes, has_debug )
      },
      syn::Data::Union( _ ) =>
      {
          // Unions are not supported.
          Err( syn::Error::new( ast.span(), "Former derive does not support unions" ) )
      }
  }?;

  // If the top-level `#[debug]` attribute was found, print the final generated code.
  if has_debug
  {
    let about = format!( "derive : Former\nstructure : {}", ast.ident );
    diag::report_print( about, &original_input, &result );
  }

  Ok( result )
}
