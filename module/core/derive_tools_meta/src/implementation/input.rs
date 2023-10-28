
use super::*;

//

pub struct InputParsed
{
  pub item : syn::ItemStruct,
  pub item_name : syn::Ident,
  pub fields : syn::Fields,
  pub fields_many : Many< syn::Field >,
  // pub field_type : syn::Type,
}

impl InputParsed
{
  #[ allow( dead_code ) ]
  pub fn first_field_type( &self ) -> Result< syn::Type >
  {
    let maybe_field = match self.fields
    {
      syn::Fields::Named( ref fields ) => fields.named.first(),
      syn::Fields::Unnamed( ref fields ) => fields.unnamed.first(),
      _ => return Err( syn_err!( self.fields.span(), "Expects fields" ) ),
    };

    // let maybe_field = self.fields.0.first();
    // let maybe_field = self.fields;

    if let Some( field ) = maybe_field
    {
      return Ok( field.ty.clone() )
    }

    return Err( syn_err!( self.item.span(), "Expects type for fields" ) );
  }
}

//

impl syn::parse::Parse for InputParsed
{
  fn parse( input : ParseStream< '_ > ) -> Result< Self >
  {
    let item : syn::ItemStruct = input.parse()?;

    // # example of input
    //
    // pub struct IsTransparent( bool );
    //

    let item_name = item.ident.clone();
    let fields = item.fields.clone();
    let fields_many : Vec< syn::Field > = match item.fields
    {
      syn::Fields::Unnamed( ref fields ) => { fields.unnamed.iter().cloned().collect() },
      syn::Fields::Named( ref fields ) => { fields.named.iter().cloned().collect() },
      _ => return Err( syn_err!( item.fields.span(), "Not implemented" ) ),
    };

    // if fields.len() != 1
    // {
    //   return Err( syn_err!( fields.span(), "Expects exactly one field, not implemented for {}.", fields.len() ) );
    // }
    // let field = fields.first().cloned().unwrap();
    // let field_type = field.ty.clone();

    let fields_many = fields_many.into();
    Ok( Self { item, item_name, fields, fields_many } )
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
