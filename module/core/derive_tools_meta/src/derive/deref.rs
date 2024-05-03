
use super::*;
use macro_tools::{ type_struct, Result };

pub fn deref( input : proc_macro::TokenStream ) -> Result< proc_macro2::TokenStream >
{
  let parsed = syn::parse::< type_struct::TypeStructParsed >( input )?;
  let field_type = parsed.first_field_type()?;
  let generic_arguments = parsed.generic_arguments();
  let item_name = parsed.item_name;
  let generics = parsed.generics;
  let where_clause = &generics.where_clause;

  let result = qt!
  {
    impl #generics ::core::ops::Deref for #item_name #generic_arguments #where_clause
    {
      type Target = #field_type;
      #[ inline( always ) ]
      fn deref( &self ) -> &Self::Target
      {
        &self.0
      }
    }
  };

  Ok( result )
}
