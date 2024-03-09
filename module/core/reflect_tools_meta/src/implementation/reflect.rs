
// use macro_tools::proc_macro2::TokenStream;
use super::*;

//

pub fn reflect( input : proc_macro::TokenStream ) -> Result< proc_macro2::TokenStream >
{
  let parsed = syn::parse::< type_struct::TypeStructParsed >( input )?;
  // let field_types = parsed.field_types;
  // let field_names = parsed.field_names;
  // let item_name = parsed.item_name;

  let result = qt!
  {
  };

  Ok( result )
}
