
use super::*;

//

pub struct InputParsed
{
  pub item : syn::ItemStruct,
  pub item_name : syn::Ident,
  pub fields : syn::Fields,
  pub fields_many : Many< syn::Field >,
  pub field_types: Vec< syn::Type >,
  pub field_names: Option< Vec< syn::Ident > >,
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

  #[ allow( dead_code ) ]
  pub fn first_field_name( &self ) -> Result< Option< syn::Ident > >
  {
    let maybe_field = match self.fields
    {
      syn::Fields::Named( ref fields ) => fields.named.first(),
      syn::Fields::Unnamed( ref fields ) => fields.unnamed.first(),
      _ => return Err( syn_err!( self.fields.span(), "Expects fields" ) ),
    };

    if let Some( field ) = maybe_field
    {
      return Ok( field.ident.clone() )
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
      _ => return Ok( Self { item, item_name, fields, fields_many: Many(vec![]), field_types: vec![], field_names: None } ),
    };

    // if fields.len() != 1
    // {
    //   return Err( syn_err!( fields.span(), "Expects exactly one field, not implemented for {}.", fields.len() ) );
    // }
    // let field = fields.first().cloned().unwrap();
    // let field_type = field.ty.clone();
    let fields_many = fields_many.into();
    let field_types = field_types( &fields_many )?;
    let field_names = field_names( &fields_many )?;
    Ok( Self { item, item_name, fields, fields_many, field_types, field_names } )
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


fn field_types ( fields: &Many< syn::Field > ) -> Result< Vec< syn::Type> >
{
  let mut field_types: Vec< syn::Type > = vec![];
  for elem in fields 
  {
      field_types.push( elem.ty.clone() );
  }
  Ok( field_types )
}

fn field_names( fields: &Many< syn::Field > ) -> Result< Option< Vec< syn::Ident > > > 
{
  let mut field_names: Vec< syn::Ident > = vec![];
  for elem in fields 
  {
    if let Some( ident ) = &elem.ident  
    {
      field_names.push( ident.clone() );
    } 
    else 
    {
        return Ok( None );
    }
  }
  Ok( Some( field_names ) )
}