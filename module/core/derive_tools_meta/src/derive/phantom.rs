#![allow(dead_code)]
use macro_tools::{generic_params, struct_like::StructLike, Result, attr, syn, proc_macro2, return_syn_err, Spanned};

use super::item_attributes::{ItemAttributes};

///
/// Derive macro to implement `PhantomData` when-ever it's possible to do automatically.
///
pub fn phantom(input: proc_macro::TokenStream) -> Result<proc_macro2::TokenStream> {
  let _original_input = input.clone();
  let parsed = syn::parse::<StructLike>(input)?;
  let _has_debug = attr::has_debug(parsed.attrs().iter())?;
  let _item_attrs = ItemAttributes::from_attrs(parsed.attrs().iter())?;
  let _item_name = &parsed.ident();

  let (_generics_with_defaults, _generics_impl, _generics_ty, _generics_where) = generic_params::decompose(parsed.generics());

  match parsed {
    StructLike::Unit(ref _item) => {
      return_syn_err!(parsed.span(), "PhantomData can not be derived for unit structs");
    }
    StructLike::Struct(ref item) => {
      return_syn_err!(item.span(), "PhantomData can not be derived for structs");
    }
    StructLike::Enum(ref item) => {
      return_syn_err!(item.span(), "PhantomData can not be derived for enums");
    }
  };
}
