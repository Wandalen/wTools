use super::*;
use the_module::{generic_params::GenericsRef, syn, quote, parse_quote};

#[test]
fn generics_ref_refined_test() {
  let mut generics_std: syn::Generics = syn::parse_quote! { <'a, T: Display + 'a, const N: usize> };
  generics_std.where_clause = parse_quote! { where T: Debug };
  let generics_empty: syn::Generics = syn::parse_quote! {};
  let enum_name: syn::Ident = syn::parse_quote! { MyEnum };

  let generics_ref_std = GenericsRef::new(&generics_std);
  let generics_ref_empty = GenericsRef::new(&generics_empty);

  // impl_generics_tokens_if_any
  let got = generics_ref_std.impl_generics_tokens_if_any();
  let exp = quote! { <'a, T: Display + 'a, const N: usize> };
  assert_eq!(got.to_string(), exp.to_string());

  let got = generics_ref_empty.impl_generics_tokens_if_any();
  let exp = quote! {};
  assert_eq!(got.to_string(), exp.to_string());

  // ty_generics_tokens_if_any
  let got = generics_ref_std.ty_generics_tokens_if_any();
  let exp = quote! { <'a, T, N> };
  assert_eq!(got.to_string(), exp.to_string());

  let got = generics_ref_empty.ty_generics_tokens_if_any();
  let exp = quote! {};
  assert_eq!(got.to_string(), exp.to_string());

  // where_clause_tokens_if_any
  let got = generics_ref_std.where_clause_tokens_if_any();
  let exp = quote! { where T: Debug };
  assert_eq!(got.to_string(), exp.to_string());

  let got = generics_ref_empty.where_clause_tokens_if_any();
  let exp = quote! {};
  assert_eq!(got.to_string(), exp.to_string());

  // type_path_tokens_if_any
  let got = generics_ref_std.type_path_tokens_if_any(&enum_name);
  let exp = quote! { MyEnum <'a, T, N> };
  assert_eq!(got.to_string(), exp.to_string());

  let got = generics_ref_empty.type_path_tokens_if_any(&enum_name);
  let exp = quote! { MyEnum };
  assert_eq!(got.to_string(), exp.to_string());
}
