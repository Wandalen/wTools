use macro_tools::{
  syn, quote,
  generic_params::{GenericsRef},
};
use syn::parse_quote;

#[test]
fn test_generics_ref_std() {
  // Test Matrix Rows: T5.6, T5.8, T5.10, T5.12
  let mut generics_std: syn::Generics = parse_quote! { <'a, T, const N: usize> };
  generics_std.where_clause = Some(parse_quote! { where T: 'a + core::fmt::Display, T: core::fmt::Debug });
  let enum_name: syn::Ident = parse_quote! { MyEnum };
  let generics_ref = GenericsRef::new(&generics_std);

  // T5.6
  let expected_impl = quote! { <'a, T, const N: usize> };
  let got_impl = generics_ref.impl_generics_tokens_if_any();
  assert_eq!(got_impl.to_string(), expected_impl.to_string());

  // T5.8
  let expected_ty = quote! { <'a, T, N> };
  let got_ty = generics_ref.ty_generics_tokens_if_any();
  assert_eq!(got_ty.to_string(), expected_ty.to_string());

  // T5.10
  let expected_where = quote! { where T: 'a + core::fmt::Display, T: core::fmt::Debug };
  let got_where = generics_ref.where_clause_tokens_if_any();
  assert_eq!(got_where.to_string(), expected_where.to_string());

  // T5.12
  let expected_path = quote! { MyEnum <'a, T, N> };
  let got_path = generics_ref.type_path_tokens_if_any(&enum_name);
  assert_eq!(got_path.to_string(), expected_path.to_string());
}

#[test]
fn test_generics_ref_empty() {
  // Test Matrix Rows: T5.7, T5.9, T5.11, T5.13
  let generics_empty: syn::Generics = parse_quote! {};
  let enum_name: syn::Ident = parse_quote! { MyEnum };
  let generics_ref = GenericsRef::new(&generics_empty);

  // T5.7
  let expected_impl = quote! {};
  let got_impl = generics_ref.impl_generics_tokens_if_any();
  assert_eq!(got_impl.to_string(), expected_impl.to_string());

  // T5.9
  let expected_ty = quote! {};
  let got_ty = generics_ref.ty_generics_tokens_if_any();
  assert_eq!(got_ty.to_string(), expected_ty.to_string());

  // T5.11
  let expected_where = quote! {};
  let got_where = generics_ref.where_clause_tokens_if_any();
  assert_eq!(got_where.to_string(), expected_where.to_string());

  // T5.13
  let expected_path = quote! { MyEnum };
  let got_path = generics_ref.type_path_tokens_if_any(&enum_name);
  assert_eq!(got_path.to_string(), expected_path.to_string());
}
