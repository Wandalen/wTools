use super::*;
use macro_tools::
{
  ct,
  Result,
  AttributeComponent,
};

use former_types::Assign;

///
/// Attributes of the whole item.
///

/// Represents the attributes of a struct. Aggregates all its attributes.
#[ derive( Debug, Default ) ]
pub struct ItemAttributes
{
  /// Attribute for customizing generated code.
  pub config : ItemAttributeConfig,
}

impl ItemAttributes
{
  pub fn from_attrs< 'a >( attrs : impl Iterator< Item = &'a syn::Attribute > ) -> Result< Self >
  {
    let mut result = Self::default();

    let error = | attr : &syn::Attribute | -> syn::Error
    {
      let known_attributes = ct::concatcp!
      (
        "Known attributes are : ",
        ItemAttributeConfig::KEYWORD,
        ".",
      );
      syn_err!
      (
        attr,
        "Expects an attribute of format '#[ attribute( key1 = val1, key2 = val2 ) ]'\n  {known_attributes}\n  But got: '{}'",
        qt!{ #attr }
      )
    };

    for attr in attrs
    {

      let key_ident = attr.path().get_ident().ok_or_else( || error( attr ) )?;
      let key_str = format!( "{}", key_ident );

      match key_str.as_ref()
      {
        ItemAttributeConfig::KEYWORD => result.assign( ItemAttributeConfig::from_meta( attr )? ),
        _ => {},
      }
    }

    Ok( result )
  }
}

///
/// Attribute to hold parameters of forming for a specific field.
/// For example to avoid [Not](core::ops::Not) handling for it use `#[ not( off ) ]`
///
#[ derive( Debug, Default ) ]
pub struct ItemAttributeConfig
{
  /// Specifies whether [Not](core::ops::Not) fields should be handled by default.
  /// Can be altered using `on` and `off` attributes. But default it's `on`.
  /// `#[ not( on ) ]` - [Not](core::ops::Not) is generated unless `off` for the field is explicitly specified.
  /// `#[ not( off ) ]` - [Not](core::ops::Not) is not generated unless `on` for the field is explicitly specified.
  pub enabled : AttributePropertyEnabled,
}

impl AttributeComponent for ItemAttributeConfig
{
  const KEYWORD : &'static str = "not";

  fn from_meta( attr : &syn::Attribute ) -> Result< Self >
  {
    match attr.meta
    {
      syn::Meta::List( ref meta_list ) =>
      {
        return syn::parse2::< ItemAttributeConfig >( meta_list.tokens.clone() );
      },
      syn::Meta::Path( ref _path ) =>
      {
        return Ok( Default::default() )
      },
      _ => return_syn_err!( attr, "Expects an attribute of format `#[ not( off ) ]`. \nGot: {}", qt!{ #attr } ),
    }
  }

}

impl< IntoT > Assign< ItemAttributeConfig, IntoT > for ItemAttributes
where
  IntoT : Into< ItemAttributeConfig >,
{
  #[ inline( always ) ]
  fn assign( &mut self, component : IntoT )
  {
    self.config.assign( component.into() );
  }
}

impl< IntoT > Assign< ItemAttributeConfig, IntoT > for ItemAttributeConfig
where
  IntoT : Into< ItemAttributeConfig >,
{
  #[ inline( always ) ]
  fn assign( &mut self, component : IntoT )
  {
    let component = component.into();
    self.enabled.assign( component.enabled );
  }
}

impl< IntoT > Assign< AttributePropertyEnabled, IntoT > for ItemAttributeConfig
where
  IntoT : Into< AttributePropertyEnabled >,
{
  #[ inline( always ) ]
  fn assign( &mut self, component : IntoT )
  {
    self.enabled = component.into();
  }
}

impl syn::parse::Parse for ItemAttributeConfig
{
  fn parse( input : syn::parse::ParseStream< '_ > ) -> syn::Result< Self >
  {
    let mut result = Self::default();

    let error = | ident : &syn::Ident | -> syn::Error
    {
      let known = ct::concatcp!
      (
        "Known entries of attribute ", ItemAttributeConfig::KEYWORD, " are : ",
        EnabledMarker::KEYWORD_ON,
        ", ", EnabledMarker::KEYWORD_OFF,
        ".",
      );
      syn_err!
      (
        ident,
        r#"Expects an attribute of format '#[ not( off ) ]'
  {known}
  But got: '{}'
"#,
        qt!{ #ident }
      )
    };

    while !input.is_empty()
    {
      let lookahead = input.lookahead1();
      if lookahead.peek( syn::Ident )
      {
        let ident : syn::Ident = input.parse()?;
        match ident.to_string().as_str()
        {
          EnabledMarker::KEYWORD_ON => result.assign( AttributePropertyEnabled::from( true ) ),
          EnabledMarker::KEYWORD_OFF => result.assign( AttributePropertyEnabled::from( false ) ),
          _ => return Err( error( &ident ) ),
        }
      }
      else
      {
        return Err( lookahead.error() );
      }

      // Optional comma handling
      if input.peek( syn::Token![ , ] )
      {
        input.parse::< syn::Token![ , ] >()?;
      }
    }

    Ok( result )
  }
}

// ==
