use super::*;
use the_module::{ ident, syn, quote };
use convert_case::{ Case, Casing };

#[ test ]
fn cased_ident_from_ident_test()
{
  let ident1 = syn::parse_str::< syn::Ident >( "MyVariant" ).unwrap();
  let got = ident::cased_ident_from_ident( &ident1, Case::Snake ).unwrap();
  let exp = "my_variant";
  assert_eq!( got.to_string(), exp );

  let ident2 = syn::parse_str::< syn::Ident >( "my_variant" ).unwrap();
  let got = ident::cased_ident_from_ident( &ident2, Case::Snake ).unwrap();
  let exp = "my_variant";
  assert_eq!( got.to_string(), exp );

  let ident3 = syn::parse_str::< syn::Ident >( "r#fn" ).unwrap();
  let got = ident::cased_ident_from_ident( &ident3, Case::Snake ).unwrap();
  let exp = "r#fn";
  assert_eq!( got.to_string(), exp );

  let ident4 = syn::parse_str::< syn::Ident >( "r#MyKeyword" ).unwrap();
  let got = ident::cased_ident_from_ident( &ident4, Case::Snake ).unwrap();
  let exp = "my_keyword";
  assert_eq!( got.to_string(), exp );

  // TODO: Fix this test case
  // let ident5 = syn::parse_str::< syn::Ident >( "if" ).unwrap();
  // let got = ident::cased_ident_from_ident( &ident5, Case::Snake );
  // assert!( got.is_err() );
}