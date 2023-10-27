
use macro_tools::proc_macro2::TokenStream;
use proc_macro::Ident;

use super::*;

//

pub fn from_inner( input : proc_macro::TokenStream ) -> Result< proc_macro2::TokenStream >
{

  let parsed = syn::parse::< InputParsed >( input )?;
  let field_types = parsed.field_types()?;
  let field_names = parsed.field_names()?;
  let item_name = parsed.item_name;
  let result = 
  match (field_types.len(), field_names) {
    (1, Some(field_names)) => generate_from_single_field(&field_types[0], &field_names[0], item_name),
    (1, None) => generate_from_single_field_no_names(&field_types[0], item_name),
    (_, Some(field_names)) => generate_from_multiple_fields(&field_types, &field_names, item_name),
    (n, None) => generate_from_multiple_fields_no_names(&field_types, n, item_name),
  };
  Ok( result )
}

fn generate_from_single_field(field_type: &syn::Type, field_name: &syn::Ident, item_name: syn::Ident) -> TokenStream {
  qt! {
      #[automatically_derived]
      impl From<#field_type> for #item_name {
          #[inline(always)]
          fn from(src: #field_type) -> Self {
              Self { #field_name: src }
          }
      }
  }
}

fn generate_from_single_field_no_names(field_type: &syn::Type, item_name: syn::Ident) -> TokenStream {
  qt! {
      #[automatically_derived]
      impl From<#field_type> for #item_name {
          #[inline(always)]
          fn from(src: #field_type) -> Self {
              Self(src)
          }
      }
  }
}

fn generate_from_multiple_fields(field_types: &Vec<syn::Type>, field_names: &Vec<syn::Ident>, item_name: syn::Ident) -> TokenStream {
  let params: Vec<TokenStream> = field_names.iter().enumerate()
      .map(|(index, field_name)| {
          let index = index.to_string().parse::<TokenStream>().unwrap();
          qt! { #field_name: src.#index }
      })
      .collect();

  qt! {
      impl From<(#(#field_types), *)> for #item_name {
          #[inline(always)]
          fn from(src: (#(#field_types), *)) -> Self {
              #item_name { #(#params), * }
          }
      }
  }
}

fn generate_from_multiple_fields_no_names(field_types: &Vec<syn::Type>, n: usize, item_name: syn::Ident) -> TokenStream {
  let params: Vec<TokenStream> = (0..n)
      .map(|index| {
          let index = index.to_string().parse::<TokenStream>().unwrap();
          qt!(src.#index)
      })
      .collect();

  qt! {
      impl From<(#(#field_types), *)> for #item_name {
          #[inline(always)]
          fn from(src: (#(#field_types), *)) -> Self {
              #item_name(#(#params), *)
          }
      }
  }
}