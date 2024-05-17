
use super::*;
use macro_tools::{ item_struct, Result };

pub fn deref( input : proc_macro::TokenStream ) -> Result< proc_macro2::TokenStream >
{
  let parsed = syn::parse::< syn::ItemStruct >( input )?;
  let field_type = item_struct::first_field_type( &parsed )?;
  let item_name = parsed.ident;

  let result = qt!
  {
    impl core::ops::Deref for #item_name
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
