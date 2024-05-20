
use super::*;
use macro_tools::{ attr, diag, item_struct, Result };

pub fn deref( input : proc_macro::TokenStream ) -> Result< proc_macro2::TokenStream >
{
  let original_input = input.clone();
  let parsed = syn::parse::< type_struct::TypeStructParsed >( input )?;
  let has_debug = attr::has_debug( parsed.attrs.iter() )?;

  let field_type = item_struct::first_field_type( &parsed )?;
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

  if has_debug
  {
    let about = format!( "derive : Deref\nstructure : {item_name}" );
    diag::report_print( about, &original_input, &result );
  }

  Ok( result )
}
