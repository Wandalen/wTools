
use crate::*;

//

pub fn deref_mut( input : proc_macro::TokenStream ) -> Result< proc_macro2::TokenStream >
{
  let parsed = syn::parse::< InputParsed >( input )?;
  // let field_type = parsed.field_type;
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
