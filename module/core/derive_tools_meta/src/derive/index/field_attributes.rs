use macro_tools::
{
  ct,
  syn_err,
  syn,
  qt,
  Result,
  AttributePropertyComponent,
  AttributePropertyOptionalSingletone,
  Assign,
};

///
/// Attributes of a field / variant
///

/// Represents the attributes of a struct. Aggregates all its attributes.
#[ derive( Debug, Default ) ]
pub struct FieldAttributes
{
  /// Specifies whether we should generate Index implementation for the field.
  pub index : AttributePropertyIndex,
}

impl FieldAttributes
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
        "Known attributes are : ",
        ", ", AttributePropertyIndex::KEYWORD,
        ".",
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
        AttributePropertyIndex::KEYWORD => result.assign( AttributePropertyIndex::from( true ) ),
        _ => {},
        // _ => return Err( error( attr ) ),
      }
    }

    Ok( result )
  }
}

impl< IntoT > Assign< AttributePropertyIndex, IntoT > for FieldAttributes
where
  IntoT : Into< AttributePropertyIndex >,
{
  #[ inline( always ) ]
  fn assign( &mut self, component : IntoT )
  {
    self.index.assign( component.into() );
  }
}


// == Attribute properties

/// Marker type for attribute property to indicate whether a index code should be generated.
/// Defaults to `false`, meaning no index code is generated unless explicitly requested.
#[ derive( Debug, Default, Clone, Copy ) ]
pub struct AttributePropertyIndexMarker;

impl AttributePropertyComponent for AttributePropertyIndexMarker
{
   const KEYWORD : & 'static str = "index";
}

/// Indicates whether a index code should be generated.
/// Defaults to `false`, meaning no index code is generated unless explicitly requested.
pub type AttributePropertyIndex = AttributePropertyOptionalSingletone< AttributePropertyIndexMarker >;

// == 


