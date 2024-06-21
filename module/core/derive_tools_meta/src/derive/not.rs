use super::*;
use macro_tools::
{
  attr,
  diag,
  Result,
  syn::ItemStruct,
};

pub fn not( input : proc_macro::TokenStream  ) -> Result< proc_macro2::TokenStream >
{
  let original_input = input.clone();
  let parsed = syn::parse::< ItemStruct >( input )?;
  let has_debug = attr::has_debug( parsed.attrs.iter() )?;
  let item_name = &parsed.ident;

  let result = qt!
  {
    impl ::core::ops::Not for #item_name
    {
      type Output = ();

      fn not( self ) -> Self::Output
      {
        ()
      }
    }
};

  if has_debug
  {
    let about = format!( "derive : Not\nstructure : {item_name}" );
    diag::report_print( about, &original_input, &result );
  }

  Ok( result )
}