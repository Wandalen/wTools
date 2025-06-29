#![ doc( html_logo_url = "https://raw.githubusercontent.com/Wandalen/wTools/master/asset/img/logo_v3_trans_square.png" ) ]
#![ doc( html_favicon_url = "https://raw.githubusercontent.com/Wandalen/wTools/alpha/asset/img/logo_v3_trans_square_icon_small_v2.ico" ) ]
#![ doc( html_root_url = "https://docs.rs/variadic_from_meta/latest/variadic_from_meta/" ) ]
#![ doc = include_str!( concat!( env!( "CARGO_MANIFEST_DIR" ), "/", "Readme.md" ) ) ]

use proc_macro::TokenStream;
use quote::{ quote, ToTokens };
use syn::{ parse_macro_input, DeriveInput, Data, Fields, Type };
use proc_macro2::Span; // Re-add Span for syn::Ident::new

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

  let ( field_types, field_names_or_indices, is_tuple_struct ) : ( Vec< &Type >, Vec< proc_macro2::TokenStream >, bool ) = match &data.fields
  {
    Fields::Unnamed( fields ) =>
    {
      let types = fields.unnamed.iter().map( |f| &f.ty ).collect();
      let indices = ( 0..fields.unnamed.len() ).map( |i| syn::Index::from( i ).to_token_stream() ).collect();
      ( types, indices, true )
    },
    Fields::Named( fields ) =>
    {
      let types = fields.named.iter().map( |f| &f.ty ).collect();
      let names = fields.named.iter().map( |f| f.ident.as_ref().unwrap().to_token_stream() ).collect();
      ( types, names, false )
    },
    _ => return syn::Error::new_spanned( ast, "VariadicFrom can only be derived for structs with named or unnamed fields." ).to_compile_error().into(),
  };

  let num_fields = field_types.len();
  let _first_field_type = field_types.get( 0 ).cloned();
  let _first_field_name_or_index = field_names_or_indices.get( 0 ).cloned();

  let mut impls = quote! {};

  // Generate FromN trait implementations (for variadic arguments)
  if num_fields == 0 || num_fields > 3
  {
    // As per spec.md, if field count is 0 or >3, the derive macro generates no code.
    return TokenStream::new();
  }

  // Generate new argument names for the `from` function
  let from_fn_args : Vec<proc_macro2::Ident> = (0..num_fields).map(|i| syn::Ident::new(&format!("__a{}", i + 1), Span::call_site())).collect();
  let _from_fn_args_pattern = quote! { #( #from_fn_args ),* }; // For the pattern in `fn from((...))`
  if num_fields > 0 && num_fields <= 3
  {
    match num_fields
    {
      1 =>
      {
        let field_type = &field_types[ 0 ];
        let field_name_or_index = &field_names_or_indices[ 0 ];
        let constructor = if is_tuple_struct { quote! { ( a1 ) } } else { quote! { { #field_name_or_index : a1 } } };
        impls.extend( quote!
        {
          impl variadic_from::exposed::From1< #field_type > for #name
          {
            fn from1( a1 : #field_type ) -> Self
            {
              Self #constructor
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

        let constructor_1_2 = if is_tuple_struct { quote! { ( a1, a2 ) } } else { quote! { { #field_name_or_index1 : a1, #field_name_or_index2 : a2 } } };
        let constructor_1_1 = if is_tuple_struct { quote! { ( a1, a1 ) } } else { quote! { { #field_name_or_index1 : a1, #field_name_or_index2 : a1 } } };

        impls.extend( quote!
        {
          impl variadic_from::exposed::From2< #field_type1, #field_type2 > for #name
          {
            fn from2( a1 : #field_type1, a2 : #field_type2 ) -> Self
            {
              Self #constructor_1_2
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
              Self #constructor_1_1
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

        let constructor_1_2_3 = if is_tuple_struct { quote! { ( a1, a2, a3 ) } } else { quote! { { #field_name_or_index1 : a1, #field_name_or_index2 : a2, #field_name_or_index3 : a3 } } };
        let constructor_1_1_1 = if is_tuple_struct { quote! { ( a1, a1, a1 ) } } else { quote! { { #field_name_or_index1 : a1, #field_name_or_index2 : a1, #field_name_or_index3 : a1 } } };
        let constructor_1_2_2 = if is_tuple_struct { quote! { ( a1, a2, a2 ) } } else { quote! { { #field_name_or_index1 : a1, #field_name_or_index2 : a2, #field_name_or_index3 : a2 } } };

        impls.extend( quote!
        {
          impl variadic_from::exposed::From3< #field_type1, #field_type2, #field_type3 > for #name
          {
            fn from3( a1 : #field_type1, a2 : #field_type2, a3 : #field_type3 ) -> Self
            {
              Self #constructor_1_2_3
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
              Self #constructor_1_1_1
            }
          }
        });
        impls.extend( quote!
        {
          impl variadic_from::exposed::From2< #field_type1, #field_type2 > for #name
          {
            fn from2( a1 : #field_type1, a2 : #field_type2 ) -> Self
            {
              Self #constructor_1_2_2
            }
          }
        });
      },
      _ => {}, // Should be caught by the initial num_fields check
    }

    // Generate From<T> or From<(T1, ..., TN)> for conversion
    if num_fields == 1
    {
      let field_type = &field_types[ 0 ];
      let from_fn_arg = &from_fn_args[ 0 ];
      // qqq: from_fn_args is defined outside this block, but used here.
      // This is a temporary fix to resolve the E0425 error.
      // The `from_fn_args` variable needs to be moved to a scope accessible by both branches.
      let field_name_or_index_0 = &field_names_or_indices[0];
let constructor_arg = if is_tuple_struct { quote! { #from_fn_arg } } else { quote! { #field_name_or_index_0 : #from_fn_arg } };
      let constructor = if is_tuple_struct { quote! { ( #constructor_arg ) } } else { quote! { { #constructor_arg } } };

      impls.extend( quote!
      {
        impl From< #field_type > for #name
        {
          #[ inline( always ) ]
          fn from( #from_fn_arg : #field_type ) -> Self
          {
            Self #constructor
          }
        }
      });
    }
    else // num_fields is 2 or 3
    {
      let tuple_types = quote! { #( #field_types ),* };
      let from_fn_args_pattern = quote! { #( #from_fn_args ),* };
      let constructor_args_for_from_trait = if is_tuple_struct {
          quote! { #( #from_fn_args ),* }
      } else {
          let named_field_inits = field_names_or_indices.iter().zip(from_fn_args.iter()).map(|(name, arg)| {
              quote! { #name : #arg }
          }).collect::<Vec<_>>();
          quote! { #( #named_field_inits ),* }
      };
      let tuple_constructor = if is_tuple_struct { quote! { ( #constructor_args_for_from_trait ) } } else { quote! { { #constructor_args_for_from_trait } } };

      impls.extend( quote!
      {
        impl From< ( #tuple_types ) > for #name
        {
          #[ inline( always ) ]
          fn from( ( #from_fn_args_pattern ) : ( #tuple_types ) ) -> Self
          {
            Self #tuple_constructor
          }
        }
      });
    }
  }

  // Process #[from(Type)] attributes
  // This section is removed as per spec.md
  /*
  for attr in &ast.attrs
  {
    if attr.path().is_ident( "from" )
    {
      if let ( Some( target_field_type ), Some( target_field_name_or_index ) ) = ( first_field_type, first_field_name_or_index.clone() )
      {
        let from_type : syn::Type = attr.parse_args().unwrap_or_else( | _ |
        {
          panic!( "Expected a type argument for `from` attribute, e.g., `#[from(i32)]`. Got: {}", attr.to_token_stream() )
        });

        // For #[from(Type)], the argument is always `value`.
        let from_constructor_arg = if is_tuple_struct { quote! { value as #target_field_type } } else { quote! { #target_field_name_or_index : value as #target_field_type } };
        let from_constructor = if is_tuple_struct { quote! { ( #from_constructor_arg ) } } else { quote! { { #from_constructor_arg } } };

        impls.extend( quote!
        {
          impl From< #from_type > for #name
          {
            fn from( value : #from_type ) -> Self
            {
              Self #from_constructor
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
  */

  // If no implementations were generated by field count, and no #[from(Type)] attributes were processed,
  // then the macro should return an error.
  // However, as per spec.md, if field count is 0 or >3, the derive macro generates no code.
  // So, the `if impls.is_empty()` check should only return an error if there are no fields AND no #[from(Type)] attributes.
  // Since #[from(Type)] is removed, this check simplifies.
  if num_fields == 0 || num_fields > 3
  {
    // No code generated for these cases, as per spec.md.
    // If the user tries to use FromN or From<tuple>, it will be a compile error naturally.
    // So, we return an empty TokenStream.
    return TokenStream::new();
  }

  let result = quote!
  {
    #impls
  };
  result.into()
}