
use super::*;

//

pub fn inner_from( input : proc_macro::TokenStream ) -> Result< proc_macro2::TokenStream >
{

  let parsed = syn::parse::< InputParsed >( input )?;
  let field_type = parsed.field_type;
  let item_name = parsed.item_name;
  let result;
  if let Some(ident) = parsed.field_name{
    result = qt!
    {
      #[ automatically_derived ]
      impl From< #item_name > for #field_type
        {
          #[ inline( always ) ]
          fn from( src : #item_name ) -> Self
          {
            src.#ident
          }
        }
    }
  }
  else{
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
