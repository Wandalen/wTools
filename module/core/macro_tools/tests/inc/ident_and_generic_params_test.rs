//!
//! Test for `ident` and `generic_params` modules.
//!

#[ allow( unused_imports ) ]
use super::*;
use macro_tools::{ syn, quote, format_ident };
use convert_case::Case;

// Test Matrix for ident::cased_ident_from_ident
// Factors: Original Ident (normal, raw), Target Case (Snake, Camel, Pascal, Kebab, ScreamingSnake)
// Combinations:
// | ID    | Original Ident | Case           | Expected Output |
// |-------|----------------|----------------|-----------------|
// | I1.1  | `my_var`       | Snake          | `my_var`        |
// | I1.2  | `my_var`       | Camel          | `myVar`         |
// | I1.3  | `my_var`       | Pascal         | `MyVar`         |
// | I1.4  | `my_var`       | Kebab          | `my-var`        |
// | I1.5  | `my_var`       | ScreamingSnake | `MY_VAR`        |
// | I1.6  | `r#fn`         | Snake          | `r#fn`          |
// | I1.7  | `r#fn`         | Camel          | `r#fn`          |
// | I1.8  | `r#fn`         | Pascal         | `r#Fn`          |
// | I1.9  | `r#fn`         | Kebab          | `r#fn`          |
// | I1.10 | `r#fn`         | ScreamingSnake | `r#FN`          |
// | I1.11 | `struct`       | Pascal         | `r#Struct`      |
// | I1.12 | `MyStruct`     | Snake          | `my_struct`     |

#[ test ]
fn test_cased_ident_from_ident()
{
  // Test Matrix Row: I1.1
  let original = format_ident!( "my_var" );
  let got = macro_tools::ident::cased_ident_from_ident( &original, Case::Snake );
  assert_eq!( got.to_string(), "my_var" );

  // Test Matrix Row: I1.2
  let original = format_ident!( "my_var" );
  let got = macro_tools::ident::cased_ident_from_ident( &original, Case::Camel );
  assert_eq!( got.to_string(), "myVar" );

  // Test Matrix Row: I1.3
  let original = format_ident!( "my_var" );
  let got = macro_tools::ident::cased_ident_from_ident( &original, Case::Pascal );
  assert_eq!( got.to_string(), "MyVar" );

  // Test Matrix Row: I1.4
  let original = format_ident!( "my_var" );
  let got = macro_tools::ident::cased_ident_from_ident( &original, Case::Kebab );
  assert_eq!( got.to_string(), "my-var" );

  // Test Matrix Row: I1.5
  let original = format_ident!( "my_var" );
  let got = macro_tools::ident::cased_ident_from_ident( &original, Case::ScreamingSnake );
  assert_eq!( got.to_string(), "MY_VAR" );

  // Test Matrix Row: I1.6
  let original = format_ident!( "r#fn" );
  let got = macro_tools::ident::cased_ident_from_ident( &original, Case::Snake );
  assert_eq!( got.to_string(), "r#fn" );

  // Test Matrix Row: I1.7
  let original = format_ident!( "r#fn" );
  let got = macro_tools::ident::cased_ident_from_ident( &original, Case::Camel );
  assert_eq!( got.to_string(), "r#fn" );

  // Test Matrix Row: I1.8
  let original = format_ident!( "r#fn" );
  let got = macro_tools::ident::cased_ident_from_ident( &original, Case::Pascal );
  assert_eq!( got.to_string(), "r#Fn" );

  // Test Matrix Row: I1.9
  let original = format_ident!( "r#fn" );
  let got = macro_tools::ident::cased_ident_from_ident( &original, Case::Kebab );
  assert_eq!( got.to_string(), "r#fn" );

  // Test Matrix Row: I1.10
  let original = format_ident!( "r#fn" );
  let got = macro_tools::ident::cased_ident_from_ident( &original, Case::ScreamingSnake );
  assert_eq!( got.to_string(), "r#FN" );

  // Test Matrix Row: I1.11
  let original = format_ident!( "struct" );
  let got = macro_tools::ident::cased_ident_from_ident( &original, Case::Pascal );
  assert_eq!( got.to_string(), "r#Struct" );

  // Test Matrix Row: I1.12
  let original = format_ident!( "MyStruct" );
  let got = macro_tools::ident::cased_ident_from_ident( &original, Case::Snake );
  assert_eq!( got.to_string(), "my_struct" );
}

// Test Matrix for generic_params::GenericsRef
// Factors: Generics (empty, type params, lifetimes, const params, where clause)
// Combinations:
// | ID    | Generics Input                               | impl_generics_tokens_if_any | ty_generics_tokens_if_any | where_clause_tokens_if_any | type_path_tokens_if_any (Base Ident: MyType) |
// |-------|----------------------------------------------|-----------------------------|---------------------------|----------------------------|----------------------------------------------|
// | G1.1  | `<>`                                         | ``                          | ``                        | ``                         | `MyType`                                     |
// | G1.2  | `<T>`                                        | `<T>`                       | `<T>`                     | ``                         | `MyType<T>`                                  |
// | G1.3  | `<'a>`                                       | `<'a>`                      | `<'a>`                    | ``                         | `MyType<'a>`                                 |
// | G1.4  | `<const N: usize>`                           | `<const N: usize>`          | `<N>`                     | ``                         | `MyType<N>`                                  |
// | G1.5  | `<T: Debug, 'a, const N: usize>`             | `<T: Debug, 'a, const N: usize>` | `<T, 'a, N>`              | ``                         | `MyType<T, 'a, N>`                           |
// | G1.6  | `<T> where T: Default`                       | `<T>`                       | `<T>`                     | `where T: Default`         | `MyType<T>`                                  |
// | G1.7  | `<T: Debug> where T: Default + Clone`        | `<T: Debug>`                | `<T>`                     | `where T: Default + Clone` | `MyType<T>`                                  |
// | G1.8  | `<'a, T> where 'a: 'static, T: 'a`           | `<'a, T>`                   | `<'a, T>`                 | `where 'a: 'static, T: 'a` | `MyType<'a, T>`                              |

#[ test ]
fn test_generics_ref()
{
  let base_ident = format_ident!( "MyType" );

  // Test Matrix Row: G1.1
  let generics = syn::parse_quote! {};
  let generics_ref = macro_tools::generic_params::GenericsRef::new( &generics );
  assert_eq!( generics_ref.impl_generics_tokens_if_any().to_string(), "" );
  assert_eq!( generics_ref.ty_generics_tokens_if_any().to_string(), "" );
  assert_eq!( generics_ref.where_clause_tokens_if_any().to_string(), "" );
  assert_eq!( generics_ref.type_path_tokens_if_any( &base_ident ).to_string(), "MyType" );

  // Test Matrix Row: G1.2
  let generics = syn::parse_quote! { < T > };
  let generics_ref = macro_tools::generic_params::GenericsRef::new( &generics );
  assert_eq!( generics_ref.impl_generics_tokens_if_any().to_string(), "< T >" );
  assert_eq!( generics_ref.ty_generics_tokens_if_any().to_string(), "< T >" );
  assert_eq!( generics_ref.where_clause_tokens_if_any().to_string(), "" );
  assert_eq!( generics_ref.type_path_tokens_if_any( &base_ident ).to_string(), "MyType < T >" );

  // Test Matrix Row: G1.3
  let generics = syn::parse_quote! { < 'a > };
  let generics_ref = macro_tools::generic_params::GenericsRef::new( &generics );
  assert_eq!( generics_ref.impl_generics_tokens_if_any().to_string(), "< 'a >" );
  assert_eq!( generics_ref.ty_generics_tokens_if_any().to_string(), "< 'a >" );
  assert_eq!( generics_ref.where_clause_tokens_if_any().to_string(), "" );
  assert_eq!( generics_ref.type_path_tokens_if_any( &base_ident ).to_string(), "MyType < 'a >" );

  // Test Matrix Row: G1.4
  let generics = syn::parse_quote! { < const N : usize > };
  let generics_ref = macro_tools::generic_params::GenericsRef::new( &generics );
  assert_eq!( generics_ref.impl_generics_tokens_if_any().to_string(), "< const N : usize >" );
  assert_eq!( generics_ref.ty_generics_tokens_if_any().to_string(), "< N >" );
  assert_eq!( generics_ref.where_clause_tokens_if_any().to_string(), "" );
  assert_eq!( generics_ref.type_path_tokens_if_any( &base_ident ).to_string(), "MyType < N >" );

  // Test Matrix Row: G1.5
  let generics = syn::parse_quote! { < T : Debug, 'a, const N : usize > };
  let generics_ref = macro_tools::generic_params::GenericsRef::new( &generics );
  assert_eq!( generics_ref.impl_generics_tokens_if_any().to_string(), "< T : Debug, 'a, const N : usize >" );
  assert_eq!( generics_ref.ty_generics_tokens_if_any().to_string(), "< T, 'a, N >" );
  assert_eq!( generics_ref.where_clause_tokens_if_any().to_string(), "" );
  assert_eq!( generics_ref.type_path_tokens_if_any( &base_ident ).to_string(), "MyType < T, 'a, N >" );

  // Test Matrix Row: G1.6
  let generics = syn::parse_quote! { < T > where T : Default };
  let generics_ref = macro_tools::generic_params::GenericsRef::new( &generics );
  assert_eq!( generics_ref.impl_generics_tokens_if_any().to_string(), "< T >" );
  assert_eq!( generics_ref.ty_generics_tokens_if_any().to_string(), "< T >" );
  assert_eq!( generics_ref.where_clause_tokens_if_any().to_string(), "where T : Default" );
  assert_eq!( generics_ref.type_path_tokens_if_any( &base_ident ).to_string(), "MyType < T >" );

  // Test Matrix Row: G1.7
  let generics = syn::parse_quote! { < T : Debug > where T : Default + Clone };
  let generics_ref = macro_tools::generic_params::GenericsRef::new( &generics );
  assert_eq!( generics_ref.impl_generics_tokens_if_any().to_string(), "< T : Debug >" );
  assert_eq!( generics_ref.ty_generics_tokens_if_any().to_string(), "< T >" );
  assert_eq!( generics_ref.where_clause_tokens_if_any().to_string(), "where T : Default + Clone" );
  assert_eq!( generics_ref.type_path_tokens_if_any( &base_ident ).to_string(), "MyType < T >" );

  // Test Matrix Row: G1.8
  let generics = syn::parse_quote! { < 'a, T > where 'a : 'static, T : 'a };
  let generics_ref = macro_tools::generic_params::GenericsRef::new( &generics );
  assert_eq!( generics_ref.impl_generics_tokens_if_any().to_string(), "< 'a, T >" );
  assert_eq!( generics_ref.ty_generics_tokens_if_any().to_string(), "< 'a, T >" );
  assert_eq!( generics_ref.where_clause_tokens_if_any().to_string(), "where 'a : 'static , T : 'a" );
  assert_eq!( generics_ref.type_path_tokens_if_any( &base_ident ).to_string(), "MyType < 'a, T >" );
}