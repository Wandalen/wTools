
use super::*;

//

pub fn inner_from( input : proc_macro::TokenStream ) -> Result< proc_macro2::TokenStream >
{

  let parsed = syn::parse::< InputParsed >( input )?;
  let field_type = parsed.first_field_type()?;
  let field_name = parsed.first_field_name()?;
  let item_name = parsed.item_name;
  let result;
  if let Some(field_name) = field_name{
    result = qt!
    {
      #[ automatically_derived ]
      impl From< #item_name > for #field_type
      {
        #[ inline( always ) ]
        fn from( src : #item_name ) -> Self
        {
          src.#field_name
        }
      }
    }
  } else {
    result = qt!
    {
      #[ automatically_derived ]
      impl From< #item_name > for #field_type
      {
        #[ inline( always ) ]
        fn from( src : #item_name ) -> Self
        {
          src.0
        }
      }
    };
  }

  Ok( result )
}
