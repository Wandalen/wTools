use super::*;
use macro_tools::
{
  attr, 
  diag, 
  struct_like::StructLike, 
  Result
};

pub fn index( input : proc_macro::TokenStream ) -> Result< proc_macro2::TokenStream > {
  let original_input = input.clone();
  let parsed = syn::parse::< StructLike >( input )?;
  let has_debug = attr::has_debug( parsed.attrs().iter() )?;
  let item_name = &parsed.ident();

  let result = match parsed 
  {
    StructLike::Struct( ref item ) => Err(
      syn::Error::new(
        item.fields.span(),
        "Not implemented yet",
      ) 
    ),
    StructLike::Enum(_) => 
    unimplemented!( "Index not implemented for Enum" ),
    StructLike::Unit(_) => 
    unimplemented!( "Index not implemented for Unit" ),
  }?;

  if has_debug 
  {
    let about = format!( "derive : Not\nstructure : {item_name}" );
    diag::report_print( about, &original_input, &result );
  }

  Ok( result )
}

