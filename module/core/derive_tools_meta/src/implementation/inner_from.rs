
use macro_tools::proc_macro2::TokenStream;

use super::*;

//

pub fn inner_from( input : proc_macro::TokenStream ) -> Result< proc_macro2::TokenStream >
{

  let parsed = syn::parse::< InputParsed >( input )?;
  let field_types = parsed.field_types()?;
  let field_names = parsed.field_names()?;
  let item_name = parsed.item_name;
  let result;
  match (field_types.len(), field_names) {
      (1, Some(field_names)) => {
        let field_name = field_names.get(0).unwrap();
        let field_type = field_types.get(0).unwrap();
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
        };
      }
      (1, None) => {
        let field_type = field_types.get(0).unwrap();
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
      (_, Some(field_names)) => {
        let mut params: Vec<TokenStream> = vec![];
        for field_name in field_names.iter(){
          params.push(qt!{ src.#field_name });
        }
        result = qt!
        {
          #[ automatically_derived ]
          impl From< #item_name > for (# (#field_types), *)
          {
            #[ inline( always ) ]
            fn from( src : #item_name ) -> Self
            {
              (# (#params), *)
            }
          }
        };
      }
      (n, None) => {
        let mut params: Vec<TokenStream> = vec![];
        for index in 0..n{
          let index: TokenStream = index.to_string().parse()?;
          params.push(qt!{ src.#index });
        }
        result = qt!
        {
          #[ automatically_derived ]
          impl From< #item_name > for (# (#field_types), *)
          {
            #[ inline( always ) ]
            fn from( src : #item_name ) -> Self
            {
              (# (#params), *)
            }
          }
        };
      }
  };

  Ok( result )
}
