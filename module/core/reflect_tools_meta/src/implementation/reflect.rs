
// use macro_tools::proc_macro2::TokenStream;
use super::*;

//

pub fn reflect( input : proc_macro::TokenStream ) -> Result< proc_macro2::TokenStream >
{
  let parsed = syn::parse::< syn::ItemStruct >( input )?;

  let result = qt!
  {
  };

  Ok( result )
}
