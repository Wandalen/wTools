
use super::*;
use iter_tools::{ Itertools, process_results };
use macro_tools::{ attr, diag, generic_params, generic_args, container_kind, typ, Result };
use proc_macro2::TokenStream;

///
/// Definition of a field.
///

///
/// Attributes of a struct.
///

pub struct StructAttributes
{
  perform : Option< AttributePerform >,
}

impl StructAttributes
{
  // fn from_attrs( attributes : & Vec< syn::Attribute > ) -> Result< Self >
  pub fn from_attrs< 'a >( attrs : impl Iterator< Item = &'a syn::Attribute > ) -> Result< Self >
  {
    let mut perform = None;
    for attr in attrs
    {
      let key_ident = attr.path().get_ident()
      .ok_or_else( || syn_err!( attr, "Expects an attribute of format #[ attribute( val ) ], but got:\n  {}", qt!{ #attr } ) )?;
      let key_str = format!( "{}", key_ident );

      if attr::is_standard( &key_str )
      {
        continue;
      }

      match key_str.as_ref()
      {
        "storage_fields" =>
        {
        }
        "perform" =>
        {
        }
        "debug" =>
        {
        }
        _ =>
        {
          return Err( syn_err!( attr, "Known structure attirbutes are : `storage_fields`, `perform`, `debug`.\nUnknown structure attribute : {}", qt!{ #attr } ) );
        }
      }
    }

    Ok( StructAttributes { perform } )
  }
}

///
/// Attribute to hold information about method to call after form.
///
/// `#[ perform( fn after1< 'a >() -> Option< &'a str > ) ]`
///

// xxx : move out
pub struct AttributePerform
{
  // paren_token : syn::token::Paren,
  pub signature : syn::Signature,
}

impl syn::parse::Parse for AttributePerform
{
  fn parse( input : syn::parse::ParseStream< '_ > ) -> Result< Self >
  {
    // let input2;
    Ok( Self
    {
      // paren_token : syn::parenthesized!( input2 in input ),
      // signature : input2.parse()?,
      signature : input.parse()?,
    })
  }
}
