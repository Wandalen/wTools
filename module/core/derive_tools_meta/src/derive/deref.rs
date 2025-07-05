use macro_tools::
{
  diag,
  struct_like::StructLike,
  Result,
  qt,
  attr,
  syn,
  proc_macro2,
  Spanned,
};
use macro_tools::diag::prelude::*;

use macro_tools::quote::ToTokens;


///
/// Derive macro to implement Deref when-ever it's possible to do automatically.
///
pub fn deref( input : proc_macro::TokenStream ) -> Result< proc_macro2::TokenStream >
{
  let original_input = input.clone();
  let parsed = syn::parse::< StructLike >( input )?;
  let has_debug = attr::has_debug( parsed.attrs().iter() )?;
  let item_name = &parsed.ident();

  let ( generics_impl, generics_ty, generics_where_option )
  = parsed.generics().split_for_impl();


  let result = match parsed
  {
    StructLike::Unit( ref item ) =>
    {
      return_syn_err!( item.span(), "Deref cannot be derived for unit structs. It is only applicable to structs with at least one field." );
    },
    StructLike::Struct( ref item ) =>
    {
      let fields_count = item.fields.len();
      let mut target_field_type = None;
      let mut target_field_name = None;
      let mut deref_attr_count = 0;

      if fields_count == 0 {
        return_syn_err!( item.span(), "Deref cannot be derived for structs with no fields." );
      } else if fields_count == 1 {
        // Single field struct: automatically deref to that field
        let field = item.fields.iter().next().expect( "Expects a single field to derive Deref" );
        target_field_type = Some( field.ty.clone() );
        target_field_name.clone_from( &field.ident );
      } else {
        // Multi-field struct: require #[deref] attribute on one field
        for field in &item.fields {
          if attr::has_deref( field.attrs.iter() )? {
            deref_attr_count += 1;
            target_field_type = Some( field.ty.clone() );
            target_field_name.clone_from( &field.ident );
          }
        }

        if deref_attr_count == 0 {
          return_syn_err!( item.span(), "Deref cannot be derived for multi-field structs without a `#[deref]` attribute on one field." );
        } else if deref_attr_count > 1 {
          return_syn_err!( item.span(), "Only one field can have the `#[deref]` attribute." );
        }
      }

      let field_type = target_field_type.ok_or_else(|| syn_err!( item.span(), "Could not determine target field type for Deref." ))?;
      let field_name = target_field_name;

      generate
      (
        item_name,
        &generics_impl, // Pass as reference
        &generics_ty, // Pass as reference
        generics_where_option,
        &field_type,
        field_name.as_ref(),
        &original_input,
      )
    },
    StructLike::Enum( ref item ) =>
    {
      return_syn_err!( item.span(), "Deref cannot be derived for enums. It is only applicable to structs with a single field or a field with `#[deref]` attribute." );
    },
  };

  if has_debug
  {
    let about = format!( "derive : Deref\nstructure : {item_name}" );
    diag::report_print( about, &original_input, &result );
  }

  Ok( result )
}

/// Generates `Deref` implementation for structs.
///
/// Example of generated code:
/// ```text
/// impl Deref for IsTransparent
/// {
///   type Target = bool;
///   fn deref( &self ) -> &bool
/// ///   {
/// ///     &self.0
/// ///   }
/// /// }
/// ```
fn generate
(
  item_name : &syn::Ident,
  generics_impl : &syn::ImplGenerics<'_>, // Use ImplGenerics with explicit lifetime
  generics_ty : &syn::TypeGenerics<'_>, // Use TypeGenerics with explicit lifetime
  generics_where: Option< &syn::WhereClause >, // Use WhereClause
  field_type : &syn::Type,
  field_name : Option< &syn::Ident >,
  original_input : &proc_macro::TokenStream,
)
-> proc_macro2::TokenStream
{
  let body = if let Some( field_name ) = field_name
  {
    qt!{ &self.#field_name }
  }
  else
  {
    qt!{ &self.0 }
  };

  let where_clause_tokens = if let Some( generics_where ) = generics_where
  {
    qt!{ where #generics_where }
  }
  else
  {
    proc_macro2::TokenStream::new()
  };

  let debug = format!
  (
    r"
impl {} core::ops::Deref for {} {}
{}
{{
  type Target = {};
  #[ inline ]
  fn deref( &self ) -> &{}
  {{
    {}
  }}
}}
    ",
    qt!{ #generics_impl },
    item_name,
    generics_ty.to_token_stream(), // Use generics_ty directly for debug
    where_clause_tokens,
    qt!{ #field_type },
    qt!{ #field_type },
    body,
  );
  let about = format!
  (
r"derive : Deref
item : {item_name}
field_type : {field_type:?}
field_name : {field_name:?}",
  );
  diag::report_print( about, original_input, debug.to_string() );

  qt!
  {
    #[ automatically_derived ]
    impl #generics_impl ::core::ops::Deref for #item_name #generics_ty #generics_where
    {
      type Target = #field_type;
      fn deref( &self ) -> & #field_type
      {
        #body
      }
    }
  }
}
