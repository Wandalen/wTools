
use super::*;
use macro_tools::{ type_struct, Result };

//

pub fn deref_mut( input : proc_macro::TokenStream ) -> Result< proc_macro2::TokenStream >
{
  let parsed = syn::parse::< type_struct::TypeStructParsed >( input )?;
  let generic_arguments = parsed.generic_arguments();
  let item_name = parsed.item_name;
  let generics = parsed.generics;
  let where_clause = &generics.where_clause;

  let result = qt!
  {
    impl #generics ::core::ops::DerefMut for #item_name #generic_arguments #where_clause
    {
      #[ inline( always ) ]
      fn deref_mut( &mut self ) -> &mut Self::Target
      {
        &mut self.0
      }
    }
  };

  Ok( result )
}
