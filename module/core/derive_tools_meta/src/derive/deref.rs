
use super::*;
use macro_tools::{ type_struct, Result };

pub fn deref( input : proc_macro::TokenStream ) -> Result< proc_macro2::TokenStream >
{
  let parsed = syn::parse::< type_struct::TypeStructParsed >( input )?;
  let field_type = parsed.first_field_type()?;
  let item_name = parsed.item_name;

  let result = qt!
  {
    impl core::ops::Deref for #item_name
    {
      type Target = #field_type;
      #[ inline( always ) ]
      fn deref( &self ) -> &Self::Target
      {
        &self.0
      }
    }
  };

  Ok( result )
}
