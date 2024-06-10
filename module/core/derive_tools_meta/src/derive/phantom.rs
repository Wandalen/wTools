use super::*;
use macro_tools::
{
  attr,
  diag,
  Result,
  phantom::add_to_item,
  quote::ToTokens,
  syn::ItemStruct,
};

pub fn phantom( input : proc_macro::TokenStream  ) -> Result< proc_macro2::TokenStream >
{
  let original_input = input.clone();
  let parsed = syn::parse::< ItemStruct >( input )?;
  let has_debug = attr::has_debug( parsed.attrs.iter() )?;
  let item_name = &parsed.ident;

  let result = add_to_item( &parsed ).to_token_stream();

  if has_debug
  {
    let about = format!( "derive : PhantomData\nstructure : {item_name}" );
    diag::report_print( about, &original_input, &result );
  }

  Ok( result )
}