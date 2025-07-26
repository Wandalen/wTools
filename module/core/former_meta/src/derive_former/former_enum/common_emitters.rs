use super::*;
use macro_tools::{ Result, quote::quote };

pub fn handle( _ctx : &mut EnumVariantHandlerContext<'_> ) -> Result< proc_macro2::TokenStream >
{
  // This file is for common emitters, not a direct handler.
  // It will contain helper functions.
  // For now, return an empty TokenStream.
  Ok( quote!{} )
}
