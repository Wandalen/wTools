
use super::*;
use macro_tools::{ item_struct, Result };

//

pub fn as_ref( input : proc_macro::TokenStream ) -> Result< proc_macro2::TokenStream >
{
  let parsed = syn::parse::< syn::ItemStruct >( input )?;
  let field_type = item_struct::first_field_type( &parsed )?;
  let item_name = parsed.ident;

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
