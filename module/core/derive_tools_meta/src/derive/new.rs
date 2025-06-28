use macro_tools::
{
  diag,
  generic_params,
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
/// Derive macro to implement New when-ever it's possible to do automatically.
///
pub fn new( input : proc_macro::TokenStream ) -> Result< proc_macro2::TokenStream >
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
      generate_unit( item_name, &generics_impl, &generics_ty, &generics_where )
    },
    StructLike::Struct( ref item ) =>
    {
      let fields_result : Result< Vec< proc_macro2::TokenStream > > = item.fields.iter().map( | field |
      {
        let _attrs = FieldAttributes::from_attrs( field.attrs.iter() )?;
        let field_name = &field.ident;
        
        let field_name_assign = if field_name.is_some()
        {
          qt!{ #field_name : Default::default() }
        }
        else
        {
          qt!{ Default::default() }
        };
        Ok( field_name_assign )
      }).collect();

      let fields = fields_result?;

      generate_struct
      (
        item_name,
        &generics_impl,
        &generics_ty,
        &generics_where,
        &fields,
        item.fields.len(),
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
    let about = format!( "derive : New\nstructure : {item_name}" );
    diag::report_print( about, &original_input, &result );
  }

  Ok( result )
}

/// Generates `New` implementation for unit structs.
///
/// Example of generated code:
/// ```text
/// impl New for MyUnit
/// {
///   fn new() -> Self
///   {
///     Self
///   }
/// }
/// ```
fn generate_unit
(
  item_name : &syn::Ident,
  generics_impl : &syn::punctuated::Punctuated< syn::GenericParam, syn::token::Comma >,
  generics_ty : &syn::punctuated::Punctuated< syn::GenericParam, syn::token::Comma >,
  generics_where: &syn::punctuated::Punctuated< syn::WherePredicate, syn::token::Comma >,
)
-> proc_macro2::TokenStream
{
  qt!
  {
    #[ automatically_derived ]
    impl< #generics_impl > crate::New for #item_name< #generics_ty >
    where
      #generics_where
    {
      #[ inline( always ) ]
      fn new() -> Self
      {
        Self
      }
    }
  }
}

/// Generates `New` implementation for structs with fields.
///
/// Example of generated code:
/// ```text
/// impl New for MyStruct
/// {
///   fn new() -> Self
///   {
///     Self { field1: Default::default(), field2: Default::default() }
///   }
/// }
/// ```
fn generate_struct
(
  item_name : &syn::Ident,
  generics_impl : &syn::punctuated::Punctuated< syn::GenericParam, syn::token::Comma >,
  generics_ty : &syn::punctuated::Punctuated< syn::GenericParam, syn::token::Comma >,
  generics_where: &syn::punctuated::Punctuated< syn::WherePredicate, syn::token::Comma >,
  fields : &Vec< proc_macro2::TokenStream >,
  fields_len : usize,
)
-> proc_macro2::TokenStream
{
  let body = if fields_len == 0
  {
    qt!{ Self }
  }
  else
  {
    qt!{ Self { #( #fields ),* } }
  };

  qt!
  {
    #[ automatically_derived ]
    impl< #generics_impl > crate::New for #item_name< #generics_ty >
    where
      #generics_where
    {
      #[ inline( always ) ]
      fn new() -> Self
      {
        #body
      }
    }
  }
}

/// Generates `New` implementation for enum variants.
///
/// Example of generated code:
/// ```text
/// impl New for MyEnum
/// {
///   fn new() -> Self
///   {
///     Self::Variant( Default::default() )
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
    return_syn_err!( fields.span(), "Expects a single field to derive New" );
  }

  let field = fields.iter().next().expect( "Expects a single field to derive New" );
  let field_name = &field.ident;

  let body = if let Some( field_name ) = field_name
  {
    qt!{ Self::#variant_name { #field_name : Default::default() } }
  }
  else
  {
    qt!{ Self::#variant_name( Default::default() ) }
  };

  if attrs.debug.value( false )
  {
    let debug = format!
    (
      r"
#[ automatically_derived ]
impl< {} > crate::New for {}< {} >
where
  {}
{{
  #[ inline ]
  fn new() -> Self
  {{
    {}
  }}
}}
      ",
      qt!{ #generics_impl },
      item_name,
      qt!{ #generics_ty },
      qt!{ #generics_where },
      body,
    );
    let about = format!
    (
r"derive : New
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
      impl< #generics_impl > crate::New for #item_name< #generics_ty >
      where
        #generics_where
      {
        #[ inline ]
        fn new() -> Self
        {
          #body
        }
      }
    }
  )
}
