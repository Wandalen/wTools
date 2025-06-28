use super::*;
use macro_tools::
{
  attr,
  diag,
  generic_params,
  item_struct,
  struct_like::StructLike,
  Result,
  qt,
};

use super::field_attributes::{ FieldAttributes };
use super::item_attributes::{ ItemAttributes };
use super::field_attributes::AttributePropertyDebug;

///
/// Derive macro to implement DerefMut when-ever it's possible to do automatically.
///
pub fn deref_mut( input : proc_macro::TokenStream ) -> Result< proc_macro2::TokenStream >
{
  let original_input = input.clone();
  let parsed = syn::parse::< StructLike >( input )?;
  let has_debug = AttributePropertyDebug::from_attrs( parsed.attrs().iter() )?.value( false );
  let item_attrs = ItemAttributes::from_attrs( parsed.attrs().iter() )?;
  let item_name = &parsed.ident();

  let ( _generics_with_defaults, generics_impl, generics_ty, generics_where )
  = generic_params::decompose( &parsed.generics() );

  let result = match parsed
  {
    StructLike::Unit( ref _item ) =>
    {
      return_syn_err!( parsed.span(), "Expects a structure with one field" );
    },
    StructLike::Struct( ref item ) =>
    {
      let field_type = item_struct::first_field_type( &item )?;
      let field_name = item_struct::first_field_name( &item ).ok().flatten();
      generate
      (
        item_name,
        &generics_impl,
        &generics_ty,
        &generics_where,
        &field_type,
        field_name.as_ref(),
      )
    },
    StructLike::Enum( ref item ) =>
    {
      let variants_result : Result< Vec< proc_macro2::TokenStream > > = item.variants.iter().map( | variant |
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
      }).collect();

      let variants = match variants_result
      {
        Ok( v ) => v,
        Err( e ) => return Err( e ),
      };

      qt!
      {
        #( #variants )*
      }
    },
  };

  if has_debug
  {
    let about = format!( "derive : DerefMut\nstructure : {item_name}" );
    diag::report_print( about, &original_input, &result );
  }

  Ok( result )
}

/// Generates `DerefMut` implementation for structs.
///
/// Example of generated code:
/// ```rust
/// impl core::ops::DerefMut for IsTransparent
/// {
///   #[ inline( always ) ]
///   fn deref_mut( &mut self ) -> &mut Self::Target
///   {
///     &mut self.0
///   }
/// }
/// ```
fn generate
(
  item_name : &syn::Ident,
  generics_impl : &syn::punctuated::Punctuated< syn::GenericParam, syn::token::Comma >,
  generics_ty : &syn::punctuated::Punctuated< syn::GenericParam, syn::token::Comma >,
  generics_where: &syn::punctuated::Punctuated< syn::WherePredicate, syn::token::Comma >,
  _field_type : &syn::Type,
  field_name : Option< &syn::Ident >,
)
-> proc_macro2::TokenStream
{
  let body = if let Some( field_name ) = field_name
  {
    qt!{ &mut self.#field_name }
  }
  else
  {
    qt!{ &mut self.0 }
  };

  qt!
  {
    #[ automatically_derived ]
    impl< #generics_impl > core::ops::DerefMut for #item_name< #generics_ty >
    where
      #generics_where
    {
      #[ inline( always ) ]
      fn deref_mut( &mut self ) -> &mut Self::Target
      {
        #body
      }
    }
  }
}

/// Generates `DerefMut` implementation for enum variants.
///
/// Example of generated code:
/// ```rust
/// impl core::ops::DerefMut for MyEnum
/// {
///   fn deref_mut( &mut self, index : usize ) -> &mut Self::Output
///   {
///     &mut self.0[ index ]
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

  if !attrs.config.enabled.value( item_attrs.config.enabled.value( true ) )
  {
    return Ok( qt!{} )
  }

  if fields.is_empty()
  {
    return Ok( qt!{} )
  }

  if fields.len() != 1
  {
    return_syn_err!( fields.span(), "Expects a single field to derive DerefMut" );
  }

  let field = fields.iter().next().unwrap();
  let field_type = &field.ty;
  let field_name = &field.ident;

  let body = if let Some( field_name ) = field_name
  {
    qt!{ &mut self.#field_name }
  }
  else
  {
    qt!{ &mut self.0 }
  };

  if attrs.config.debug.value( false )
  {
    let debug = format_args!
    (
      r#"
#[ automatically_derived ]
impl< {} > core::ops::DerefMut for {}< {} >
where
  {}
{{
  #[ inline ]
  fn deref_mut( &mut self ) -> &mut {}
  {{
    {}
  }}
}}
      "#,
      qt!{ #generics_impl },
      item_name,
      qt!{ #generics_ty },
      qt!{ #generics_where },
      qt!{ #field_type },
      body,
    );
    let about = format!
    (
r#"derive : DerefMut
item : {item_name}
field : {variant_name}"#,
    );
    diag::report_print( about, original_input, debug.to_string() );
  }

  Ok
  (
    qt!
    {
      #[ automatically_derived ]
      impl< #generics_impl > core::ops::DerefMut for #item_name< #generics_ty >
      where
        #generics_where
      {
        #[ inline ]
        fn deref_mut( &mut self ) -> &mut #field_type
        {
          #body
        }
      }
    }
  )

}
