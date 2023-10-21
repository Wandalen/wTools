
use super::*;

//

pub struct InputParsed
{
  pub item : syn::ItemStruct,
  pub field_type : syn::Type,
  pub item_name : syn::Ident,
}

//

impl syn::parse::Parse for InputParsed
{
  fn parse( input : ParseStream< '_ > ) -> Result< Self >
  {

    // let item = match syn::parse::< syn::ItemStruct >( input )
    // {
    //   Ok( original ) => original,
    //   Err( err ) => return Err( err ),
    // };

    let item : syn::ItemStruct = input.parse()?;

    // # example of input
    //
    // pub struct IsTransparent( bool );
    //

    let item_name = item.ident.clone();
    let fields = match item.fields
    {
      syn::Fields::Unnamed( ref fields ) => { &fields.unnamed },
      _ => return Err( syn_err!( item.fields.span(), "Not implemented" ) ),
    };
    if fields.len() != 1
    {
      return Err( syn_err!( fields.span(), "Expects exactly one field, not implemented for {}.", fields.len() ) );
    }
    let field = fields.first().cloned().unwrap();
    let field_type = field.ty.clone();

    Ok( Self { item, item_name, field_type } )
  }
}

//

impl quote::ToTokens for InputParsed
{
  fn to_tokens( &self, tokens : &mut proc_macro2::TokenStream )
  {
    self.item.to_tokens( tokens );
  }
}
