
use super::*;
use macro_tools::{ type_struct, Result };

pub fn as_mut( input : proc_macro::TokenStream ) -> Result< proc_macro2::TokenStream >
{
  let parsed = syn::parse::< type_struct::TypeStructParsed >( input )?;
  let field_type = parsed.first_field_type()?;
  let item_name = parsed.item_name;

  let result = qt!
  {
    impl AsMut< #field_type > for #item_name
    {
      fn as_mut( &mut self ) -> &mut #field_type
      {
        &mut self.0
      }
    }
  };

  Ok( result )
}
