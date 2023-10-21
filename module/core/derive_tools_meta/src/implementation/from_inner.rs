
use super::*;

//

pub fn from_inner( input : proc_macro::TokenStream ) -> Result< proc_macro2::TokenStream >
{

  let parsed = syn::parse::< InputParsed >( input )?;
  let field_type = parsed.first_field_type()?;
  let item_name = parsed.item_name;

  let result = qt!
  {
    #[ automatically_derived ]
    impl From< #field_type > for #item_name
    {
      #[ inline( always ) ]
      fn from( src : #field_type ) -> Self
      {
        Self( src )
      }
    }
  };

  Ok( result )
}
