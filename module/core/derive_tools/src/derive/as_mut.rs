
use crate::*;

pub fn as_mut( input : proc_macro::TokenStream ) -> Result< proc_macro2::TokenStream >
{
  let parsed = syn::parse::< InputParsed >( input )?;
  let field_type = parsed.field_type;
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
