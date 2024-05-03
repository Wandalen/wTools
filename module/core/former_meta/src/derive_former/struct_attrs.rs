
use super::*;
use macro_tools::{ attr, Result };

///
/// Definition of a field.
///

///
/// Attributes of a struct.
///

pub struct StructAttributes
{
  pub perform : Option< AttributePerform >,
  pub storage_fields : Option< AttributeStorageFields >,
}

impl StructAttributes
{
  // fn from_attrs( attributes : & Vec< syn::Attribute > ) -> Result< Self >
  pub fn from_attrs< 'a >( attrs : impl Iterator< Item = &'a syn::Attribute > ) -> Result< Self >
  {
    let mut perform = None;
    let mut storage_fields = None;

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
          match attr.meta
          {
            syn::Meta::List( ref meta_list ) =>
            {
              storage_fields.replace( syn::parse2::< AttributeStorageFields >( meta_list.tokens.clone() )? );
            },
            _ => return_syn_err!( attr, "Expects an attribute of format #[ storage_fields( a : i32, b : Option< String > ) ]
.\nGot: {}", qt!{ #attr } ),
          }
        }
        "perform" =>
        {
          match attr.meta
          {
            syn::Meta::List( ref meta_list ) =>
            {
              perform.replace( syn::parse2::< AttributePerform >( meta_list.tokens.clone() )? );
            },
            _ => return_syn_err!( attr, "Expects an attribute of format #[ perform( fn parse( mut self ) -> Request ) ]
.\nGot: {}", qt!{ #attr } ),
          }
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

    Ok( StructAttributes { perform, storage_fields } )
  }
}

///
/// Attribute to hold information about method to call after form.
///
/// `#[ perform( fn after1< 'a >() -> Option< &'a str > ) ]`
///

pub struct AttributePerform
{
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

///
/// Attribute to hold storage-specific fields.
/// Useful if formed structure should not have such fields.
///
/// `#[ storage_fields( a : i32, b : Option< String > ) ]`
///

pub struct AttributeStorageFields
{
  pub fields : syn::punctuated::Punctuated< syn::Field, syn::token::Comma >,
}

impl syn::parse::Parse for AttributeStorageFields
{
  fn parse( input : syn::parse::ParseStream< '_ > ) -> syn::Result< Self >
  {

    let fields : syn::punctuated::Punctuated< syn::Field, syn::Token![,] > =
    input.parse_terminated( syn::Field::parse_named, Token![,] )?;

    Ok( Self
    {
      fields,
      // fields : syn::Fields::Named( syn::FieldsNamed
      // {
      //   brace_token : Default::default(),
      //   named : fields,
      // }),
    })
  }
}
