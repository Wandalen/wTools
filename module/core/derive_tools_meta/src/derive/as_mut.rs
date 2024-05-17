
use super::*;
use macro_tools::{ item_struct, Result };

pub fn as_mut( input : proc_macro::TokenStream ) -> Result< proc_macro2::TokenStream >
{
  let parsed = syn::parse::< syn::ItemStruct >( input )?;
  let field_type = item_struct::first_field_type( &parsed )?;
  let item_name = parsed.ident;

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
