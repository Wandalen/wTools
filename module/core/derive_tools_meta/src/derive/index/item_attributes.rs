use macro_tools::
{
  ct,
  syn_err,
  syn,
  qt,
  Result,
  AttributePropertyComponent,
  AttributePropertyOptionalSingletone,
};

/// Represents the attributes of a struct. Aggregates all its attributes.
#[ derive( Debug, Default ) ]
pub struct ItemAttributes
{
  pub debug : AttributePropertyDebug,
}

impl ItemAttributes
{
  /// Constructs a `ItemAttributes` instance from an iterator of attributes.
  ///
  /// This function parses the provided attributes and assigns them to the
  /// appropriate fields in the `ItemAttributes` struct.
  pub fn from_attrs< 'a >( attrs : impl Iterator< Item = & 'a syn::Attribute > ) -> Result< Self >
  {
    let result = Self::default();

    // Closure to generate an error message for unknown attributes.
    let error = | attr : & syn::Attribute | -> syn::Error
    {
      let known_attributes = ct::concatcp!
      (
        "Known attributes are: ",
        "debug",
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
        "debug" => {},
        _ => {},
        // _ => return Err( error( attr ) ),
      }
    }

    Ok( result )
  }
}

// == Attribute properties

/// Marker type for attribute property to specify whether to provide a sketch as a hint.
/// Defaults to `false`, which means no hint is provided unless explicitly requested.
#[ derive( Debug, Default, Clone, Copy ) ]
pub struct AttributePropertyDebugMarker;

impl AttributePropertyComponent for AttributePropertyDebugMarker
{
  const KEYWORD : & 'static str = "debug";
}

/// Specifies whether to provide a sketch as a hint.
/// Defaults to `false`, which means no hint is provided unless explicitly requested.
pub type AttributePropertyDebug = AttributePropertyOptionalSingletone< AttributePropertyDebugMarker >;

// == 


