#![ doc( html_logo_url = "https://raw.githubusercontent.com/Wandalen/wTools/master/asset/img/logo_v3_trans_square.png" ) ]
#![ doc( html_favicon_url = "https://raw.githubusercontent.com/Wandalen/wTools/alpha/asset/img/logo_v3_trans_square_icon_small_v2.ico" ) ]
#![ doc( html_root_url = "https://docs.rs/variadic_from_meta/latest/variadic_from_meta/" ) ]
#![ doc = include_str!( concat!( env!( "CARGO_MANIFEST_DIR" ), "/", "Readme.md" ) ) ]

use proc_macro::TokenStream;
use quote::{ quote, ToTokens };
use syn::{ parse_macro_input, DeriveInput, Data, Fields, Type };

/// Derive macro for `VariadicFrom`.
#[ proc_macro_derive( VariadicFrom, attributes( from ) ) ]
pub fn variadic_from_derive( input : TokenStream ) -> TokenStream
{
  let ast = parse_macro_input!( input as DeriveInput );
  let name = &ast.ident;

  let data = match &ast.data
  {
    Data::Struct( data ) => data,
    _ => return syn::Error::new_spanned( ast, "VariadicFrom can only be derived for structs." ).to_compile_error().into(),
  };

  let ( field_name, field_type, _field_index ) = match &data.fields
  {
    Fields::Unnamed( fields ) if fields.unnamed.len() == 1 =>
    {
      let field = &fields.unnamed[ 0 ];
      ( None, &field.ty, Some( 0 ) )
    },
    Fields::Named( fields ) if fields.named.len() == 1 =>
    {
      let field = &fields.named[ 0 ];
      ( field.ident.as_ref().map( | i | quote! { #i } ), &field.ty, None )
    },
    _ => return syn::Error::new_spanned( ast, "VariadicFrom can only be derived for structs with a single field, or with `#[from]` attribute on a single field." ).to_compile_error().into(),
  };

  let mut impls = quote! {};

  for attr in &ast.attrs
  {
    if attr.path().is_ident( "from" )
    {
      let from_type : Type = attr.parse_args().expect( &format!( "Expected a type argument for `from` attribute, e.g., `#[from(i32)]`. Got: {}", attr.to_token_stream() ) );
      let tokens = if let Some( field_name ) = &field_name
      {
        quote!
        {
          impl From< #from_type > for #name
          {
            fn from( value : #from_type ) -> Self
            {
              Self { #field_name : value as #field_type }
            }
          }
        }
      }
      else // if let Some( field_index ) = field_index // _field_index is not used directly here
      {
        // let index = syn::Index::from( field_index ); // _index is not used directly here
        quote!
        {
          impl From< #from_type > for #name
          {
            fn from( value : #from_type ) -> Self
            {
              Self( value as #field_type )
            }
          }
        }
      };
      impls.extend( tokens );
    }
  }

  if impls.is_empty()
  {
    return syn::Error::new_spanned( ast, "No `#[from(Type)]` attributes found. VariadicFrom requires at least one `#[from(Type)]` attribute." ).to_compile_error().into();
  }

  let result = quote!
  {
    #impls
  };
  result.into()
}