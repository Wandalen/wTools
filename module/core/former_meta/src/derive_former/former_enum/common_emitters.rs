use super::*;
use macro_tools::{ quote::quote };

#[allow(dead_code)]
pub fn placeholder() -> proc_macro2::TokenStream
{
  // This file is for common emitters, not a direct handler.
  // It will contain helper functions.
  // For now, return an empty TokenStream.
  quote!{}
}
