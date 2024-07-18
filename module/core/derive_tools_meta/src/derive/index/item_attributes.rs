use super::*;
use macro_tools::
{
  ct,
  Result,
  AttributeComponent,
  AttributePropertyComponent,
  AttributePropertyOptionalSyn,
  AttributePropertyOptionalSingletone,
};

/// Represents the attributes of a struct. Aggregates all its attributes.
#[ derive( Debug, Default ) ]
pub struct ItemAttributes
{
  /// Attribute for customizing generated code.
  pub index : ItemAttributeIndex,
  /// Specifies whether to provide a generated code as a hint.
  /// Defaults to `false`, which means no code is printed unless explicitly requested.
  pub debug : AttributePropertyDebug,
}

#[ derive( Debug, Default ) ]
pub struct ItemAttributeIndex
{
  /// Specifies what specific named field must implement Index.
  pub name : AttributePropertyName,
}

impl ItemAttributes
{
  /// Constructs a `ItemAttributes` instance from an iterator of attributes.
  ///
  /// This function parses the provided attributes and assigns them to the
  /// appropriate fields in the `ItemAttributes` struct.
  pub fn from_attrs< 'a >( attrs : impl Iterator< Item = & 'a syn::Attribute > ) -> Result< Self >
  {
    let mut result = Self::default();

    // Closure to generate an error message for unknown attributes.
    let error = | attr : & syn::Attribute | -> syn::Error
    {
      let known_attributes = ct::concatcp!
      (
        "Known attributes are: ",
        "debug",
        ", ", ItemAttributeIndex::KEYWORD,
        "."
      );
      syn_err!
      (
        attr,
        "Expects an attribute of format '#[ attribute ]'\n  {known_attributes}\n  But got: '{}'",
        qt! { #attr }
      )
    };

    for attr in attrs
    {
      let key_ident = attr.path().get_ident().ok_or_else( || error( attr ) )?;
      let key_str = format!( "{}", key_ident );
      match key_str.as_ref()
      {
         ItemAttributeIndex::KEYWORD => result.assign( ItemAttributeIndex::from_meta( attr )? ),
        "debug" => {},
        _ => {},
        // _ => return Err( error( attr ) ),
      }
    }

    Ok( result )
  }
}

impl AttributeComponent for ItemAttributeIndex
{
  const KEYWORD : &'static str = "index";

  fn from_meta( attr : &syn::Attribute ) -> Result< Self >
  {
    match attr.meta
    {
      syn::Meta::List( ref meta_list ) =>
      {
        return syn::parse2::< ItemAttributeIndex >( meta_list.tokens.clone() );
      },
      syn::Meta::Path( ref _path ) =>
      {
        return Ok( Default::default() )
      },
      _ => return_syn_err!( attr, "Expects an attribute of format `#[ from( on ) ]`. \nGot: {}", qt!{ #attr } ),
    }
  }

}


impl< IntoT > Assign< ItemAttributeIndex, IntoT > for ItemAttributes
where
  IntoT : Into< ItemAttributeIndex >,
{
  #[ inline( always ) ]
  fn assign( &mut self, component : IntoT )
  {
    self.index.assign( component.into() );
  }
}



impl< IntoT > Assign< AttributePropertyDebug, IntoT > for ItemAttributes
where
  IntoT : Into< AttributePropertyDebug >,
{
  #[ inline( always ) ]
  fn assign( &mut self, component : IntoT )
  {
    self.debug = component.into();
  }
}


impl< IntoT > Assign< ItemAttributeIndex, IntoT > for ItemAttributeIndex
where
  IntoT : Into< ItemAttributeIndex >,
{
  #[ inline( always ) ]
  fn assign( &mut self, component : IntoT )
  {
    let component = component.into();
    self.name.assign( component.name );
  }
}

impl< IntoT > Assign< AttributePropertyName, IntoT > for ItemAttributeIndex
where
  IntoT : Into< AttributePropertyName >,
{
  #[ inline( always ) ]
  fn assign( &mut self, component : IntoT )
  {
    self.name = component.into();
  }
}
  

impl syn::parse::Parse for ItemAttributeIndex
{
  fn parse( input : syn::parse::ParseStream< '_ > ) -> syn::Result< Self >
  {
    let mut result = Self::default();

    let error = | ident : &syn::Ident | -> syn::Error
    {
      let known = ct::concatcp!
      (
        "Known entries of attribute ", ItemAttributeIndex::KEYWORD, " are : ",
         AttributePropertyName::KEYWORD,
        ".",
      );
      syn_err!
      (
        ident,
        r#"Expects an attribute of format '#[ from( off ) ]'
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
          AttributePropertyName::KEYWORD => result.assign( AttributePropertyName::parse( input )? ),
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


// == Attribute properties

/// Marker type for attribute property of optional identifier that names the setter. It is parsed from inputs
/// like `name = field_name`.
#[ derive( Debug, Default, Clone, Copy ) ]
pub struct NameMarker;

impl AttributePropertyComponent for NameMarker
{
  const KEYWORD : &'static str = "name";
}

/// An optional identifier that names the setter. It is parsed from inputs
/// like `name = field_name`.
pub type AttributePropertyName = AttributePropertyOptionalSyn< syn::Ident, NameMarker >;

// =

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

// ==
