
use super::*;

//

pub fn as_ref( input : proc_macro::TokenStream ) -> Result< proc_macro2::TokenStream >
{
  let parsed = syn::parse::< InputParsed >( input )?;
  let field_type = parsed.first_field_type()?;
  let item_name = parsed.item_name;

  let result = qt!
  {
    impl AsRef< #field_type > for #item_name
    {
      fn as_ref( &self ) -> &#field_type
      {
        &self.0
      }
    }
  };

  Ok( result )
}
