#[cfg(test)]
mod tests {
    use macro_tools::syn::{self, parse_quote};
    use macro_tools::quote::{self, quote}; // Ensure quote is in scope
    use macro_tools::generic_params::GenericsRef; // The struct being tested

    #[test]
    fn t6_8_impl_generics_std() {
        // ID: T6.8 (`impl_generics_tokens_if_any` with `generics_std`)
        let generics_std: syn::Generics = parse_quote! { <T: Display + 'a, 'a, const N: usize> where T: Debug > };
        let generics_ref = GenericsRef::new_borrowed(&generics_std);
        let tokens = generics_ref.impl_generics_tokens_if_any().unwrap();
        let expected: proc_macro2::TokenStream = quote! { <T: Display + 'a, 'a, const N: usize> };
        assert_eq!(tokens.to_string(), expected.to_string());
    }

    #[test]
    fn t6_9_impl_generics_empty() {
        // ID: T6.9 (`impl_generics_tokens_if_any` with `generics_empty`)
        let generics_empty: syn::Generics = parse_quote! {};
        let generics_ref = GenericsRef::new_borrowed(&generics_empty);
        let tokens = generics_ref.impl_generics_tokens_if_any().unwrap();
        let expected: proc_macro2::TokenStream = quote! {};
        assert_eq!(tokens.to_string(), expected.to_string());
    }

    #[test]
    fn t6_10_ty_generics_std() {
        // ID: T6.10 (`ty_generics_tokens_if_any` with `generics_std`)
        let generics_std: syn::Generics = parse_quote! { <T: Display + 'a, 'a, const N: usize> where T: Debug > };
        let generics_ref = GenericsRef::new_borrowed(&generics_std);
        let tokens = generics_ref.ty_generics_tokens_if_any().unwrap();
        let expected: proc_macro2::TokenStream = quote! { <T, 'a, N> };
        assert_eq!(tokens.to_string(), expected.to_string());
    }

    #[test]
    fn t6_11_ty_generics_empty() {
        // ID: T6.11 (`ty_generics_tokens_if_any` with `generics_empty`)
        let generics_empty: syn::Generics = parse_quote! {};
        let generics_ref = GenericsRef::new_borrowed(&generics_empty);
        let tokens = generics_ref.ty_generics_tokens_if_any().unwrap();
        let expected: proc_macro2::TokenStream = quote! {};
        assert_eq!(tokens.to_string(), expected.to_string());
    }

    #[test]
    fn t6_12_where_clause_std() {
        // ID: T6.12 (`where_clause_tokens_if_any` with `generics_std`)
        let generics_std: syn::Generics = parse_quote! { <T: Display + 'a, 'a, const N: usize> where T: Debug > };
        let generics_ref = GenericsRef::new_borrowed(&generics_std);
        let tokens = generics_ref.where_clause_tokens_if_any().unwrap();
        let expected: proc_macro2::TokenStream = quote! { where T: Debug };
        assert_eq!(tokens.to_string(), expected.to_string());
    }

    #[test]
    fn t6_13_where_clause_empty() {
        // ID: T6.13 (`where_clause_tokens_if_any` with `generics_empty`)
        let generics_empty: syn::Generics = parse_quote! {};
        let generics_ref = GenericsRef::new_borrowed(&generics_empty);
        let tokens = generics_ref.where_clause_tokens_if_any().unwrap();
        let expected: proc_macro2::TokenStream = quote! {};
        assert_eq!(tokens.to_string(), expected.to_string());
    }

    #[test]
    fn t6_13b_where_clause_no_clause_but_generics() {
        let generics_no_where: syn::Generics = parse_quote! { <T> };
        let generics_ref = GenericsRef::new_borrowed(&generics_no_where);
        let tokens = generics_ref.where_clause_tokens_if_any().unwrap();
        let expected: proc_macro2::TokenStream = quote! {};
        assert_eq!(tokens.to_string(), expected.to_string());
    }


    #[test]
    fn t6_14_type_path_std() {
        // ID: T6.14 (`type_path_tokens_if_any` with `generics_std`, `enum_name`)
        let generics_std: syn::Generics = parse_quote! { <T: Display + 'a, 'a, const N: usize> where T: Debug > };
        let enum_name: syn::Ident = parse_quote! { MyEnum };
        let generics_ref = GenericsRef::new_borrowed(&generics_std);
        let tokens = generics_ref.type_path_tokens_if_any(&enum_name).unwrap();
        let expected: proc_macro2::TokenStream = quote! { MyEnum::<T, 'a, N> };
        assert_eq!(tokens.to_string(), expected.to_string());
    }

    #[test]
    fn t6_15_type_path_empty() {
        // ID: T6.15 (`type_path_tokens_if_any` with `generics_empty`, `enum_name`)
        let generics_empty: syn::Generics = parse_quote! {};
        let enum_name: syn::Ident = parse_quote! { MyEnum };
        let generics_ref = GenericsRef::new_borrowed(&generics_empty);
        let tokens = generics_ref.type_path_tokens_if_any(&enum_name).unwrap();
        let expected: proc_macro2::TokenStream = quote! { MyEnum };
        assert_eq!(tokens.to_string(), expected.to_string());
    }
}