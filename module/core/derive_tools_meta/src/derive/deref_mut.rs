
use super::*;
use macro_tools::{ attr, diag, Result };

//

pub fn deref_mut( input : proc_macro::TokenStream ) -> Result< proc_macro2::TokenStream >
{
  let original_input = input.clone();
  let parsed = syn::parse::< syn::ItemStruct >( input )?;
  let has_debug = attr::has_debug( parsed.attrs.iter() )?;
  let item_name = parsed.ident;

  let result = qt!
  {
    impl core::ops::DerefMut for #item_name
    {
      #[ inline( always ) ]
      fn deref_mut( &mut self ) -> &mut Self::Target
      {
        &mut self.0
      }
    }
  };

  if has_debug
  {
    let about = format!( "derive : DerefMut\nstructure : {item_name}" );
    diag::report_print( about, &original_input, &result );
  }

  Ok( result )
}
