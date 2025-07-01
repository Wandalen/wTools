#![ allow( dead_code ) ]
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
/// Derive macro to implement `PhantomData` when-ever it's possible to do automatically.
///
pub fn phantom( input : proc_macro::TokenStream ) -> Result< proc_macro2::TokenStream >
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
      let _field_name_option = item_struct::first_field_name( item )?;
      generate
      (
        item_name,
        &generics_impl,
        &generics_ty,
        &generics_where,
        &field_type,
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
    let about = format!( "derive : PhantomData\nstructure : {item_name}" );
    diag::report_print( about, &original_input, &result );
  }

  Ok( result )
}

/// Generates `PhantomData` implementation for structs.
///
/// Example of generated code:
/// ```text
/// impl PhantomData for IsTransparent
/// {
///   type Output = bool;
///   fn phantom( self ) -> bool
///   {
///     self.0
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
)
-> proc_macro2::TokenStream
{
  qt!
  {
    #[ automatically_derived ]
    impl< #generics_impl > core::marker::PhantomData< #field_type > for #item_name< #generics_ty >
    where
      #generics_where
    {
      // No methods to implement for PhantomData
    }
  }
}

/// Generates `PhantomData` implementation for enum variants.
///
/// Example of generated code:
/// ```text
/// impl PhantomData for MyEnum
/// {
///   fn phantom( self ) -> i32
///   {
///     self.0
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
    return_syn_err!( fields.span(), "Expects a single field to derive PhantomData" );
  }

  let field = fields.iter().next().expect( "Expects a single field to derive PhantomData" );
  let field_type = &field.ty;
  

  if attrs.debug.value( false )
  {
    let debug = format!
    (
      r"
#[ automatically_derived ]
impl< {} > core::marker::PhantomData< {} > for {}< {} >
where
  {}
{{
  // No methods to implement for PhantomData
}}
      ",
      qt!{ #generics_impl },
      qt!{ #field_type },
      item_name,
      qt!{ #generics_ty },
      qt!{ #generics_where },
    );
    let about = format!
    (
r"derive : PhantomData
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
      impl< #generics_impl > core::marker::PhantomData< #field_type > for #item_name< #generics_ty >
      where
        #generics_where
      {
        // No methods to implement for PhantomData
      }
    }
  )
}
