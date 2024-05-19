
use super::*;
use macro_tools::{ attr, diag, item_struct, Result };

pub fn deref( input : proc_macro::TokenStream ) -> Result< proc_macro2::TokenStream >
{
  let original_input = input.clone();
  let parsed = syn::parse::< syn::ItemStruct >( input )?;
  let has_debug = attr::has_debug( parsed.attrs.iter() )?;

  let item_name = &parsed.ident;
  let field_type = item_struct::first_field_type( &parsed )?;

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

  if has_debug
  {
    let about = format!( "derive : Deref\nstructure : {item_name}" );
    diag::report_print( about, &original_input, &result );
  }

  Ok( result )
}
