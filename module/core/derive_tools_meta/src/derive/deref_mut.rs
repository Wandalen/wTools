
use super::*;
use macro_tools::{ Result };

//

pub fn deref_mut( input : proc_macro::TokenStream ) -> Result< proc_macro2::TokenStream >
{
  let parsed = syn::parse::< syn::ItemStruct >( input )?;
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

  Ok( result )
}
