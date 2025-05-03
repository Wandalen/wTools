#[ allow( clippy::wildcard_imports ) ]
use super::*;
// Use re-exports from macro_tools
use macro_tools::
{
  qt,
  attr, diag, Result,
  proc_macro2::TokenStream,
  syn::Index,
};


///
/// Generates implementations of the `Assign` trait for each field of a struct.
///