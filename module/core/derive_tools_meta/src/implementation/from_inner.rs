
use macro_tools::proc_macro2::TokenStream;

use super::*;

//

pub fn from_inner( input : proc_macro::TokenStream ) -> Result< proc_macro2::TokenStream >
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
          impl From< #field_type > for #item_name
            {
              #[ inline( always ) ]
              fn from( src : #field_type ) -> Self
              {
                Self{ #field_name: src }
              }
            }
          };
      }
      (1, None) => {
        let field_type = field_types.get(0).unwrap();
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
      (_, Some(field_names)) => {
        let mut params: Vec<TokenStream> = vec![];
        for (index, field_name) in field_names.iter().enumerate(){
          let index: TokenStream = index.to_string().parse()?;
          params.push(qt!{ #field_name: src.#index });
        }
        result = qt!
        {
          impl From < (# (#field_types), *) > for #item_name {
            #[ inline( always ) ]
            fn from( src: (# (#field_types), *)) -> #item_name {
              #item_name{# (#params), *}
            }
          }
        };
      }
      (n, None) => {
        let mut params: Vec<TokenStream> = vec![];
        for index in 0.. n {
          let index: TokenStream = index.to_string().parse()?;
          params.push(qt!(src.#index));
        }
          result = qt!
          {
            impl From < (# (#field_types), *) > for #item_name {
              #[ inline( always ) ]
              fn from( src: (# (#field_types), *)) -> #item_name {
                #item_name(# (#params), *)
              }
            }
          };
      }
  }
  Ok( result )
}
