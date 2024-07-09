use super::*;
use macro_tools::
{
  attr,
  diag,
  generic_params,
  struct_like::StructLike,
  Result
};

pub fn index( input : proc_macro::TokenStream ) -> Result< proc_macro2::TokenStream >
{
  let original_input = input.clone();
  let parsed = syn::parse::< StructLike >( input )?;
  let has_debug = attr::has_debug( parsed.attrs().iter() )?;
  let item_name = &parsed.ident();

  let ( _generics_with_defaults, generics_impl, generics_ty, generics_where )
  = generic_params::decompose( &parsed.generics() );

  let result = match parsed
  {
    StructLike::Struct( ref item ) =>
    generate_struct
    (
      item_name,
      &generics_impl,
      &generics_ty,
      &generics_where,
      &item.fields,

    ),
    StructLike::Enum( _ ) =>
    unimplemented!( "Index not implemented for Enum" ),
    StructLike::Unit( _ ) =>
    unimplemented!( "Index not implemented for Unit" ),
  }?;

  if has_debug
  {
    let about = format!( "derive : Not\nstructure : {item_name}" );
    diag::report_print( about, &original_input, &result );
  }

  Ok( result )
}

/// An aggregator function to generate `Index` implementation for tuple and named structs
fn generate_struct
(
  item_name : &syn::Ident,
  generics_impl : &syn::punctuated::Punctuated< syn::GenericParam, syn::token::Comma >,
  generics_ty : &syn::punctuated::Punctuated< syn::GenericParam, syn::token::Comma >,
  generics_where : &syn::punctuated::Punctuated< syn::WherePredicate, syn::token::Comma >,
  fields : &syn::Fields,
)
-> Result< proc_macro2::TokenStream >
{

  match fields
  {
    syn::Fields::Named( fields ) =>
    generate_struct_named_fields
    (
      item_name,
      generics_impl,
      generics_ty,
      generics_where,
      fields
    ),

    syn::Fields::Unnamed( fields ) =>
    generate_struct_tuple_fields
    (
      item_name,
      generics_impl,
      generics_ty,
      generics_where,
      fields
    ),

    syn::Fields::Unit =>
    unimplemented!( "Index not implemented for Unit" ),
  }
}


fn generate_struct_named_fields
(
  item_name : &syn::Ident,
  generics_impl : &syn::punctuated::Punctuated< syn::GenericParam, syn::token::Comma >,
  generics_ty : &syn::punctuated::Punctuated< syn::GenericParam, syn::token::Comma >,
  generics_where : &syn::punctuated::Punctuated< syn::WherePredicate, syn::token::Comma >,
  fields : &syn::FieldsNamed,
)
-> Result< proc_macro2::TokenStream >
{
  let fields = fields.named.clone();
  let non_empty_attrs : Vec< &syn::Field > = fields.iter().filter(| field |
    !field.attrs.is_empty()
  ).collect();

  if non_empty_attrs.len() != 1
  {
    return Err(
      syn::Error::new_spanned
      (
        &fields,
        "Only one field can include #[index] derive macro"
      )
    );
  }

  let generated = fields.iter().map(| field |
    {
    let field_name = &field.ident;

    if !field.attrs.is_empty()
    {
      qt!
      {
        &self.#field_name[ index ]
      }
    }
    else
    {
      qt!{ }
    }
  });

  Ok
  (
    qt!
    {
      #[ automatically_derived ]
      impl< #generics_impl > ::core::ops::Index< usize > for #item_name< #generics_ty >
      where
        #generics_where
      {
        type Output = T;
        #[ inline( always ) ]
        fn index( &self, index : usize ) -> &Self::Output
        {
          #( #generated )*
        }
      }
    }
  )
}

fn generate_struct_tuple_fields
(
  item_name : &syn::Ident,
  generics_impl : &syn::punctuated::Punctuated< syn::GenericParam, syn::token::Comma >,
  generics_ty : &syn::punctuated::Punctuated< syn::GenericParam, syn::token::Comma >,
  generics_where : &syn::punctuated::Punctuated< syn::WherePredicate, syn::token::Comma >,
  fields : &syn::FieldsUnnamed,
)
-> Result< proc_macro2::TokenStream >
{
  let fields = fields.unnamed.clone();
  let non_empty_attrs : Vec< &syn::Field > = fields.iter().filter(| field |
    !field.attrs.is_empty()
  ).collect();

  if non_empty_attrs.len() != 1
  {
    return Err(
      syn::Error::new_spanned
      (
        &fields,
        "Only one field can include #[index] derive macro"
      )
    );
  }

  let generated = fields.iter().enumerate().map(|( i, field )|
  {
    let i = syn::Index::from( i );
    if !field.attrs.is_empty() {
      qt!
      {
        &self.#i[ index ]
      }
    }
    else
    {
      qt!{ }
    }
  });

  Ok
  (
    qt!
    {
      #[ automatically_derived ]
      impl< #generics_impl > ::core::ops::Index< usize > for #item_name< #generics_ty >
      where
        #generics_where
      {
        type Output = T;
        #[ inline( always ) ]
        fn index( &self, index : usize ) -> &Self::Output
        {
          #( #generated )*
        }
      }
    }
  )
}

