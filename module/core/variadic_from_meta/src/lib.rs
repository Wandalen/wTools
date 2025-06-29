#![ doc( html_logo_url = "https://raw.githubusercontent.com/Wandalen/wTools/master/asset/img/logo_v3_trans_square.png" ) ]
#![ doc( html_favicon_url = "https://raw.githubusercontent.com/Wandalen/wTools/alpha/asset/img/logo_v3_trans_square_icon_small_v2.ico" ) ]
#![ doc( html_root_url = "https://docs.rs/variadic_from_meta/latest/variadic_from_meta/" ) ]
#![ doc = include_str!( concat!( env!( "CARGO_MANIFEST_DIR" ), "/", "Readme.md" ) ) ]

use proc_macro::TokenStream;
use quote::{ quote, ToTokens };
use syn::{ parse_macro_input, DeriveInput, Data, Fields, Type };

/// Derive macro for `VariadicFrom`.
#[ proc_macro_derive( VariadicFrom, attributes( from ) ) ] // Re-enabled attributes(from)
pub fn variadic_from_derive( input : TokenStream ) -> TokenStream
{
  let ast = parse_macro_input!( input as DeriveInput );
  let name = &ast.ident;

  let data = match &ast.data
  {
    Data::Struct( data ) => data,
    _ => return syn::Error::new_spanned( ast, "VariadicFrom can only be derived for structs." ).to_compile_error().into(),
  };

  let ( field_types, field_names_or_indices ) : ( Vec< &Type >, Vec< proc_macro2::TokenStream > ) = match &data.fields
  {
    Fields::Unnamed( fields ) =>
    {
      let types = fields.unnamed.iter().map( |f| &f.ty ).collect();
      let indices = ( 0..fields.unnamed.len() ).map( |i| syn::Index::from( i ).to_token_stream() ).collect();
      ( types, indices )
    },
    Fields::Named( fields ) =>
    {
      let types = fields.named.iter().map( |f| &f.ty ).collect();
      let names = fields.named.iter().map( |f| f.ident.as_ref().unwrap().to_token_stream() ).collect();
      ( types, names )
    },
    _ => return syn::Error::new_spanned( ast, "VariadicFrom can only be derived for structs with named or unnamed fields." ).to_compile_error().into(),
  };

  let num_fields = field_types.len();
  let first_field_type = field_types.get( 0 ).cloned();
  let first_field_name_or_index = field_names_or_indices.get( 0 ).cloned();

  let mut impls = quote! {};

  // Generate FromN trait implementations (for variadic arguments)
  if num_fields == 0 || num_fields > 3
  {
    // This error is for the case where no #[from(Type)] attributes are present either.
    // If there are #[from(Type)] attributes, we proceed even with 0 or >3 fields.
    if ast.attrs.iter().all( |attr| !attr.path().is_ident("from") )
    {
      return syn::Error::new_spanned( ast, "VariadicFrom currently supports structs with 1 to 3 fields, or requires `#[from(Type)]` attributes." ).to_compile_error().into();
    }
  }

  if num_fields > 0 && num_fields <= 3
  {
    match num_fields
    {
      1 =>
      {
        let field_type = &field_types[ 0 ];
        let field_name_or_index = &field_names_or_indices[ 0 ];
        impls.extend( quote!
        {
          impl variadic_from::exposed::From1< #field_type > for #name
          {
            fn from1( a1 : #field_type ) -> Self
            {
              Self { #field_name_or_index : a1 }
            }
          }
        });
      },
      2 =>
      {
        let field_type1 = &field_types[ 0 ];
        let field_type2 = &field_types[ 1 ];
        let field_name_or_index1 = &field_names_or_indices[ 0 ];
        let field_name_or_index2 = &field_names_or_indices[ 1 ];
        impls.extend( quote!
        {
          impl variadic_from::exposed::From2< #field_type1, #field_type2 > for #name
          {
            fn from2( a1 : #field_type1, a2 : #field_type2 ) -> Self
            {
              Self { #field_name_or_index1 : a1, #field_name_or_index2 : a2 }
            }
          }
        });
        // Special case for From1 on a 2-field struct (as per Readme example)
        impls.extend( quote!
        {
          impl variadic_from::exposed::From1< #field_type1 > for #name
          {
            fn from1( a1 : #field_type1 ) -> Self
            {
              Self { #field_name_or_index1 : a1, #field_name_or_index2 : a1 }
            }
          }
        });
      },
      3 =>
      {
        let field_type1 = &field_types[ 0 ];
        let field_type2 = &field_types[ 1 ];
        let field_type3 = &field_types[ 2 ];
        let field_name_or_index1 = &field_names_or_indices[ 0 ];
        let field_name_or_index2 = &field_names_or_indices[ 1 ];
        let field_name_or_index3 = &field_names_or_indices[ 2 ];
        impls.extend( quote!
        {
          impl variadic_from::exposed::From3< #field_type1, #field_type2, #field_type3 > for #name
          {
            fn from3( a1 : #field_type1, a2 : #field_type2, a3 : #field_type3 ) -> Self
            {
              Self { #field_name_or_index1 : a1, #field_name_or_index2 : a2, #field_name_or_index3 : a3 }
            }
          }
        });
        // Special cases for From1 and From2 on a 3-field struct (similar to 2-field logic)
        impls.extend( quote!
        {
          impl variadic_from::exposed::From1< #field_type1 > for #name
          {
            fn from1( a1 : #field_type1 ) -> Self
            {
              Self { #field_name_or_index1 : a1, #field_name_or_index2 : a1, #field_name_or_index3 : a1 }
            }
          }
        });
        impls.extend( quote!
        {
          impl variadic_from::exposed::From2< #field_type1, #field_type2 > for #name
          {
            fn from2( a1 : #field_type1, a2 : #field_type2 ) -> Self
            {
              Self { #field_name_or_index1 : a1, #field_name_or_index2 : a2, #field_name_or_index3 : a2 }
            }
          }
        });
      },
      _ => {}, // Should be caught by the initial num_fields check
    }

    // Generate From<(T1, ..., TN)> for tuple conversion
    let tuple_types = quote! { #( #field_types ),* };
    let tuple_args = quote! { #( #field_names_or_indices ),* };

    impls.extend( quote!
    {
      impl From< ( #tuple_types ) > for #name
      {
        #[ inline( always ) ]
        fn from( ( #tuple_args ) : ( #tuple_types ) ) -> Self
        {
          Self { #tuple_args }
        }
      }
    });
  }

  // Process #[from(Type)] attributes
  for attr in &ast.attrs
  {
    if attr.path().is_ident( "from" )
    {
      if let ( Some( target_field_type ), Some( target_field_name_or_index ) ) = ( first_field_type, first_field_name_or_index.clone() )
      {
        let from_type : Type = attr.parse_args().unwrap_or_else( | _ |
        {
          panic!( "Expected a type argument for `from` attribute, e.g., `#[from(i32)]`. Got: {}", attr.to_token_stream() )
        });

        impls.extend( quote!
        {
          impl From< #from_type > for #name
          {
            fn from( value : #from_type ) -> Self
            {
              Self { #target_field_name_or_index : value as #target_field_type }
            }
          }
        });
      }
      else
      {
        return syn::Error::new_spanned( ast, "Struct must have at least one field to use `#[from(Type)]` attribute." ).to_compile_error().into();
      }
    }
  }

  if impls.is_empty()
  {
    return syn::Error::new_spanned( ast, "VariadicFrom requires at least one field or `#[from(Type)]` attribute." ).to_compile_error().into();
  }

  let result = quote!
  {
    #impls
  };
  result.into()
}