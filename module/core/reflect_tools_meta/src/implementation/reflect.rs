
// use macro_tools::proc_macro2::TokenStream;
use crate::*;
use macro_tools::{ Result, attr, diag };

//

pub fn reflect( input : proc_macro::TokenStream ) -> Result< proc_macro2::TokenStream >
{
  let original_input = input.clone();
  let parsed = syn::parse::< syn::ItemStruct >( input )?;
  let has_debug = attr::has_debug( parsed.attrs.iter() )?;
  let item_name = parsed.ident;

  let result = qt!
  {
  };

  if has_debug
  {
    let about = format!( "derive : Reflect\nstructure : {item_name}" );
    diag::report_print( about, &original_input, &result );
  }

  Ok( result )
}
