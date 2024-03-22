
use super::*;
use macro_tools::{ type_struct, Result };

//

pub fn deref_mut( input : proc_macro::TokenStream ) -> Result< proc_macro2::TokenStream >
{
  let parsed = syn::parse::< type_struct::TypeStructParsed >( input )?;
  let item_name = parsed.item_name;

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
