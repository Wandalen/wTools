use super::*;
use macro_tools::{ Result, quote::quote, syn };

pub fn handle( ctx : &mut EnumVariantHandlerContext ) -> Result< proc_macro2::TokenStream >
{
  // This file is for common emitters, not a direct handler.
  // It will contain helper functions.
  // For now, return an empty TokenStream.
  Ok( quote!{} )
}
