
#[ allow( clippy::wildcard_imports ) ]
use super::*;
use macro_tools::{ attr, diag, item_struct, Result };

//

pub fn as_ref( input : proc_macro::TokenStream ) -> Result< proc_macro2::TokenStream >
{
  let original_input = input.clone();
  let parsed = syn::parse::< syn::ItemStruct >( input )?;
  let has_debug = attr::has_debug( parsed.attrs.iter() )?;
  let field_type = item_struct::first_field_type( &parsed )?;
  let item_name = &parsed.ident;

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

  if has_debug
  {
    let about = format!( "derive : AsRef\nstructure : {item_name}" );
    diag::report_print( about, &original_input, &result );
  }

  Ok( result )
}
