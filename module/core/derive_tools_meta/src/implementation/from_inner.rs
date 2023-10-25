
use super::*;

//

pub fn from_inner( input : proc_macro::TokenStream ) -> Result< proc_macro2::TokenStream >
{

  let parsed = syn::parse::< InputParsed >( input )?;
  let field_type = parsed.first_field_type()?;
  let field_name = parsed.first_field_name()?;
  let item_name = parsed.item_name;
  let result;
  if let Some( field_name ) = field_name {
    result = qt!
    {
    #[ automatically_derived ]
    impl From< #field_type > for #item_name
      {
        #[ inline( always ) ]
        fn from( src : #field_type ) -> Self
        {
          Self{ #field_name: src }
        }
      }
    };
  } else {
    result = qt!
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
  }
  Ok( result )
}
