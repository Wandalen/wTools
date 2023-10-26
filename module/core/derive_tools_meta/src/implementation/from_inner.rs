
use macro_tools::proc_macro2::TokenStream;

use super::*;

//

pub fn from_inner( input : proc_macro::TokenStream ) -> Result< proc_macro2::TokenStream >
{

  let parsed = syn::parse::< InputParsed >( input )?;
  let field_type = parsed.first_field_type()?;
  let field_types = parsed.field_types()?;
  let field_names = parsed.field_names()?;
  let field_name = parsed.first_field_name()?;
  let item_name = parsed.item_name;
  let result;
  match ((field_types.len(), field_types, field_names), (field_name, field_type)) {
      ((1,..), (Some(field_name), field_type)) => {
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
      ((1,..), (None, field_type)) => {
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
      ((_, types,Some(names)), ..) => {
        let mut params: Vec<TokenStream> = vec![];
        for (index, field_name) in names.iter().enumerate(){
          let index: TokenStream = index.to_string().parse()?;
          params.push(qt!{ #field_name: src.#index });
        }
        result = qt!
        {
          impl From < (# (#types), *) > for #item_name {
            #[ inline( always ) ]
            fn from( src: (# (#types), *)) -> #item_name {
              #item_name{# (#params), *}
            }
          }
        };
      }
      ((_, types, None), ..) => {
        let mut params: Vec<TokenStream> = vec![];
        for index in 0.. types.len() {
          let index: TokenStream = index.to_string().parse()?;
          params.push(qt!(src.#index));
        }
          result = qt!
          {
            impl From < (# (#types), *) > for #item_name {
              #[ inline( always ) ]
              fn from( src: (# (#types), *)) -> #item_name {
                #item_name(# (#params), *)
              }
            }
          };
      }
  }
  Ok( result )
}
