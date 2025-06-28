use macro_tools::
{
  diag,
  generic_params,
  item_struct,
  struct_like::StructLike,
  Result,
  qt,
  attr,
  syn,
  proc_macro2,
  return_syn_err,
  Spanned,
};

use super::field_attributes::{ FieldAttributes };
use super::item_attributes::{ ItemAttributes };

///
/// Derive macro to implement Not when-ever it's possible to do automatically.
///
pub fn not( input : proc_macro::TokenStream ) -> Result< proc_macro2::TokenStream >
{
  let original_input = input.clone();
  let parsed = syn::parse::< StructLike >( input )?;
  let has_debug = attr::has_debug( parsed.attrs().iter() )?;
  let item_attrs = ItemAttributes::from_attrs( parsed.attrs().iter() )?;
  let item_name = &parsed.ident();

  let ( _generics_with_defaults, generics_impl, generics_ty, generics_where )
  = generic_params::decompose( parsed.generics() );

  let result = match parsed
  {
    StructLike::Unit( ref _item ) =>
    {
      return_syn_err!( parsed.span(), "Expects a structure with one field" );
    },
    StructLike::Struct( ref item ) =>
    {
      let field_type = item_struct::first_field_type( item )?;
      let field_name_option = item_struct::first_field_name( item )?;
      let field_name = field_name_option.as_ref();
      generate
      (
        item_name,
        &generics_impl,
        &generics_ty,
        &generics_where,
        &field_type,
        field_name,
      )
    },
    StructLike::Enum( ref item ) =>
    {
      let variants = item.variants.iter().map( | variant |
      {
        variant_generate
        (
          item_name,
          &item_attrs,
          &generics_impl,
          &generics_ty,
          &generics_where,
          variant,
          &original_input,
        )
      }).collect::< Result< Vec< proc_macro2::TokenStream > > >()?;

      qt!
      {
        #( #variants )*
      }
    },
  };

  if has_debug
  {
    let about = format!( "derive : Not\nstructure : {item_name}" );
    diag::report_print( about, &original_input, &result );
  }

  Ok( result )
}

/// Generates `Not` implementation for structs.
///
/// Example of generated code:
/// ```text
/// impl Not for IsTransparent
/// {
///   type Output = bool;
///   fn not( self ) -> bool
///   {
///     !self.0
///   }
/// }
/// ```
fn generate
(
  item_name : &syn::Ident,
  generics_impl : &syn::punctuated::Punctuated< syn::GenericParam, syn::token::Comma >,
  generics_ty : &syn::punctuated::Punctuated< syn::GenericParam, syn::token::Comma >,
  generics_where: &syn::punctuated::Punctuated< syn::WherePredicate, syn::token::Comma >,
  field_type : &syn::Type,
  field_name : Option< &syn::Ident >,
)
-> proc_macro2::TokenStream
{
  let body = if let Some( field_name ) = field_name
  {
    qt!{ !self.#field_name }
  }
  else
  {
    qt!{ !self.0 }
  };

  qt!
  {
    #[ automatically_derived ]
    impl< #generics_impl > core::ops::Not for #item_name< #generics_ty >
    where
      #generics_where
    {
      type Output = #field_type;
      #[ inline( always ) ]
      fn not( self ) -> #field_type
      {
        #body
      }
    }
  }
}

/// Generates `Not` implementation for enum variants.
///
/// Example of generated code:
/// ```text
/// impl Not for MyEnum
/// {
///   fn not( self ) -> i32
///   {
///     !self.0
///   }
/// }
/// ```
fn variant_generate
(
  item_name : &syn::Ident,
  item_attrs : &ItemAttributes,
  generics_impl : &syn::punctuated::Punctuated< syn::GenericParam, syn::token::Comma >,
  generics_ty : &syn::punctuated::Punctuated< syn::GenericParam, syn::token::Comma >,
  generics_where: &syn::punctuated::Punctuated< syn::WherePredicate, syn::token::Comma >,
  variant : &syn::Variant,
  original_input : &proc_macro::TokenStream,
)
-> Result< proc_macro2::TokenStream >
{
  let variant_name = &variant.ident;
  let fields = &variant.fields;
  let attrs = FieldAttributes::from_attrs( variant.attrs.iter() )?;

  if !attrs.enabled.value( item_attrs.enabled.value( true ) )
  {
    return Ok( qt!{} )
  }

  if fields.is_empty()
  {
    return Ok( qt!{} )
  }

  if fields.len() != 1
  {
    return_syn_err!( fields.span(), "Expects a single field to derive Not" );
  }

  let field = fields.iter().next().expect( "Expects a single field to derive Not" );
  let field_type = &field.ty;
  let field_name = &field.ident;

  let body = if let Some( field_name ) = field_name
  {
    qt!{ !self.#field_name }
  }
  else
  {
    qt!{ !self.0 }
  };

  if attrs.debug.value( false )
  {
    let debug = format!
    (
      r"
#[ automatically_derived ]
impl< {} > core::ops::Not for {}< {} >
where
  {}
{{
  type Output = {};
  #[ inline ]
  fn not( self ) -> {}
  {{
    {}
  }}
}}
      ",
      qt!{ #generics_impl },
      item_name,
      qt!{ #generics_ty },
      qt!{ #generics_where },
      qt!{ #field_type },
      qt!{ #field_type },
      body,
    );
    let about = format!
    (
r"derive : Not
item : {item_name}
field : {variant_name}",
    );
    diag::report_print( about, original_input, debug.to_string() );
  }

  Ok
  (
    qt!
    {
      #[ automatically_derived ]
      impl< #generics_impl > core::ops::Not for #item_name< #generics_ty >
      where
        #generics_where
      {
        type Output = #field_type;
        #[ inline ]
        fn not( self ) -> #field_type
        {
          #body
        }
      }
    }
  )
}
