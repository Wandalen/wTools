use super::*;
use macro_tools::
{
  Result,
  AttributeComponent,
  AttributePropertyComponent,
  AttributePropertyOptionalSingletone,
};

use former_types::Assign;

///
/// Attributes of a field / variant
///

/// Represents the attributes of a struct. Aggregates all its attributes.
#[ derive( Debug, Default ) ]
pub struct FieldAttributes
{
  /// Attribute for customizing generated code.
  pub config : FieldAttributeConfig,
}

impl FieldAttributes
{

  pub fn from_attrs< 'a >( attrs : impl Iterator< Item = &'a syn::Attribute > ) -> Result< Self >
  {
    let mut result = Self::default();

    let error = | attr : &syn::Attribute | -> syn::Error
    {
      let known_attributes = const_format::concatcp!
      (
        "Known attirbutes are : ",
        "debug",
        ", ", FieldAttributeConfig::KEYWORD,
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

      // attributes does not have to be known
      // if attr::is_standard( &key_str )
      // {
      //   continue;
      // }

      match key_str.as_ref()
      {
        FieldAttributeConfig::KEYWORD => result.assign( FieldAttributeConfig::from_meta( attr )? ),
        "debug" => {},
        _ => {},
        // _ => return Err( error( attr ) ),
      }
    }

    Ok( result )
  }

}

///
/// Attribute to hold parameters of forming for a specific field or variant.
/// For example to avoid code From generation for it.
///
/// `#[ from( on ) ]`
///

#[ derive( Debug, Default ) ]
pub struct FieldAttributeConfig
{
  /// Specifies whether we should generate From implementation for the field.
  /// Can be altered using `on` and `off` attributes
  pub enabled : AttributePropertyEnabled,
  /// Specifies whether to print a sketch of generated `From` or not.
  /// Defaults to `false`, which means no code is printed unless explicitly requested.
  pub debug : AttributePropertyDebug,
  // qqq : apply debug properties to all brenches, not only enums
}

impl AttributeComponent for FieldAttributeConfig
{
  const KEYWORD : &'static str = "from";

  fn from_meta( attr : &syn::Attribute ) -> Result< Self >
  {
    match attr.meta
    {
      syn::Meta::List( ref meta_list ) =>
      {
        return syn::parse2::< FieldAttributeConfig >( meta_list.tokens.clone() );
      },
      syn::Meta::Path( ref _path ) =>
      {
        return Ok( Default::default() )
      },
      _ => return_syn_err!( attr, "Expects an attribute of format `#[ from( on ) ]`. \nGot: {}", qt!{ #attr } ),
    }
  }

}

impl< IntoT > Assign< FieldAttributeConfig, IntoT > for FieldAttributes
where
  IntoT : Into< FieldAttributeConfig >,
{
  #[ inline( always ) ]
  fn assign( &mut self, component : IntoT )
  {
    self.config.assign( component.into() );
  }
}

impl< IntoT > Assign< FieldAttributeConfig, IntoT > for FieldAttributeConfig
where
  IntoT : Into< FieldAttributeConfig >,
{
  #[ inline( always ) ]
  fn assign( &mut self, component : IntoT )
  {
    let component = component.into();
    self.enabled.assign( component.enabled );
    self.debug.assign( component.debug );
  }
}

impl< IntoT > Assign< AttributePropertyEnabled, IntoT > for FieldAttributeConfig
where
  IntoT : Into< AttributePropertyEnabled >,
{
  #[ inline( always ) ]
  fn assign( &mut self, component : IntoT )
  {
    self.enabled = component.into();
  }
}

impl< IntoT > Assign< AttributePropertyDebug, IntoT > for FieldAttributeConfig
where
  IntoT : Into< AttributePropertyDebug >,
{
  #[ inline( always ) ]
  fn assign( &mut self, component : IntoT )
  {
    self.debug = component.into();
  }
}

impl syn::parse::Parse for FieldAttributeConfig
{
  fn parse( input : syn::parse::ParseStream< '_ > ) -> syn::Result< Self >
  {
    let mut result = Self::default();

    let error = | ident : &syn::Ident | -> syn::Error
    {
      let known = const_format::concatcp!
      (
        "Known entries of attribute ", FieldAttributeConfig::KEYWORD, " are : ",
        AttributePropertyDebug::KEYWORD,
        ", ", EnabledMarker::KEYWORD_ON,
        ", ", EnabledMarker::KEYWORD_OFF,
        ".",
      );
      syn_err!
      (
        ident,
        r#"Expects an attribute of format '#[ from( on ) ]'
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
          AttributePropertyDebug::KEYWORD => result.assign( AttributePropertyDebug::from( true ) ),
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

// == attribute properties

/// Marker type for attribute property to specify whether to provide a generated code as a hint.
/// Defaults to `false`, which means no debug is provided unless explicitly requested.
#[ derive( Debug, Default, Clone, Copy ) ]
pub struct AttributePropertyDebugMarker;

impl AttributePropertyComponent for AttributePropertyDebugMarker
{
  const KEYWORD : &'static str = "debug";
}

/// Specifies whether to provide a generated code as a hint.
/// Defaults to `false`, which means no debug is provided unless explicitly requested.
pub type AttributePropertyDebug = AttributePropertyOptionalSingletone< AttributePropertyDebugMarker >;

// =

/// Marker type for attribute property to indicates whether `From` implementation for fields/variants should be generated.
#[ derive( Debug, Default, Clone, Copy ) ]
pub struct EnabledMarker;

impl EnabledMarker
{
  /// Keywords for parsing this attribute property.
  pub const KEYWORD_OFF : &'static str = "off";
  /// Keywords for parsing this attribute property.
  pub const KEYWORD_ON : &'static str = "on";
}

/// Specifies whether `From` implementation for fields/variants should be generated.
/// Can be altered using `on` and `off` attributes. But default it's `on`.
pub type AttributePropertyEnabled = AttributePropertyOptionalSingletone< EnabledMarker >;

// ==
